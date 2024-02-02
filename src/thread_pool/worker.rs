use std::{
	sync::{mpsc::Receiver, Arc, Mutex},
	thread,
};

use super::state::State;

pub enum Task {
	New(Box<dyn FnOnce() + Send>),
	Break,
}

#[derive(Debug)]
pub(crate) struct Worker;

#[thread_local]
pub static mut THREAD_LOCAL_PANIC_HOOK: Option<*mut dyn Fn()> = None;

impl Worker {
	/// This will immediately spawn a new thread
	pub fn new(receiver: Arc<Mutex<Receiver<Task>>>, id: usize, state: Arc<State>) -> Self {
		let bit_id = 1u64 << id;

		thread::Builder::new()
			.name(format!("ThreadPool - Worker {id}"))
			.spawn(move || {
				let panic_state = state.clone();

				unsafe {
					THREAD_LOCAL_PANIC_HOOK = Some(Box::<dyn Fn()>::into_raw(Box::new(move || {
						panic_state.panicking.add(bit_id)
					})));
				}

				loop {
					// Lock -> Get task -> Drop/Unlock
					let task = { receiver.lock().unwrap().recv().unwrap() };

					state.busy.add(bit_id);

					match task {
						Task::New(job) => job(),
						Task::Break => break,
					}

					state.busy.remove(bit_id);
				}
			})
			.expect(&format!("Failed to spawn Worker {id}"));

		Self {}
	}
}

impl Drop for Worker {
	fn drop(&mut self) {
		unsafe {
			let Some(w) = THREAD_LOCAL_PANIC_HOOK.take() else {
				return;
			};

			w.drop_in_place();
		}
	}
}
