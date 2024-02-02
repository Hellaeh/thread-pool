use std::sync::{
	mpsc::{self, Sender},
	Arc, Mutex,
};

use crate::{thread_pool::worker::THREAD_LOCAL_PANIC_HOOK, utils::optimal_number_of_threads};

use self::{
	state::{state_cell::StateCell, State},
	worker::{Task, Worker},
};

///
#[derive(Debug)]
pub struct ThreadPool {
	_workers: Vec<Worker>,
	sender: Sender<Task>,
	capacity: usize,
	state: Arc<State>,
}

/// Max number or thread supported atm
pub const MAX_THREADS: usize = std::mem::size_of::<StateCell>() * 8;

impl ThreadPool {
	/// Will return [`ThreadPool`] with `capacity = logical-cores - 1`
	#[inline]
	pub fn new() -> Self {
		Self::with_capacity(optimal_number_of_threads(u16::MAX as usize))
	}

	/// Will return [`ThreadPool`] with user defined capacity
	#[inline]
	pub fn with_capacity(capacity: usize) -> Self {
		assert!(
			capacity <= MAX_THREADS,
			"ThreadPool: Does not support capacity over {}",
			MAX_THREADS
		);

		let mut _workers = Vec::with_capacity(capacity);

		let (sender, receiver) = mpsc::channel();
		let receiver = Arc::new(Mutex::new(receiver));

		let state = Arc::new(State::default());

		let prev_hook = std::panic::take_hook();

		std::panic::set_hook(Box::new(move |info| {
			unsafe {
				match THREAD_LOCAL_PANIC_HOOK {
					Some(f) => (*f)(),
					None => {}
				}
			}

			prev_hook(info);
		}));

		for id in 0..capacity {
			_workers.push(Worker::new(receiver.clone(), id, state.clone()));
		}

		Self {
			_workers,
			sender,
			capacity,
			state,
		}
	}

	/// Returns [`ThreadPool`] capacity
	#[inline(always)]
	pub fn capacity(&self) -> usize {
		self.capacity
	}

	/// Executes passed function in
	#[inline]
	pub fn execute<F: FnOnce() + Send + 'static>(&self, f: F) {
		self
			.sender
			.send(Task::New(Box::new(f)))
			.expect("Error while sending job to thread worker")
	}

	/// Returns an iterator for capacity
	#[inline]
	pub fn iter(&self) -> std::ops::Range<usize> {
		0..self.capacity
	}

	/// Does not actually call `join` on a thread
	/// Instead breaks from internal loop
	#[inline]
	pub fn join(self) {
		drop(self)
	}

	/// Returns an amount of threads panicking
	#[inline(always)]
	pub fn check_panics(&self) -> usize {
		self.state.panicking.count()
	}

	/// Returns an amount of busy threads
	#[inline(always)]
	pub fn check_busy(&self) -> usize {
		self.state.busy.count()
	}
}

impl Drop for ThreadPool {
	fn drop(&mut self) {
		for _ in self.iter() {
			self.sender.send(Task::Break).unwrap()
		}
	}
}

impl Default for ThreadPool {
	#[inline(always)]
	fn default() -> Self {
		Self::new()
	}
}

mod state;
mod worker;
