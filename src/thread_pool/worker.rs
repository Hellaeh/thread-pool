use std::{
	sync::{mpsc::Receiver, Arc, Mutex},
	thread::{self, JoinHandle},
};

pub enum Task {
	New(Box<dyn FnOnce() + Send>),
	Break,
}

#[derive(Debug)]
pub(crate) struct Worker(JoinHandle<()>);

impl Worker {
	pub fn new(receiver: Arc<Mutex<Receiver<Task>>>) -> Self {
		let handle = thread::spawn(move || loop {
			let task = { receiver.lock().unwrap().recv().unwrap() };

			match task {
				Task::New(job) => job(),
				Task::Break => break,
			}
		});

		Self(handle)
	}
}
