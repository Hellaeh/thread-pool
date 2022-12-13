use std::{
	sync::{mpsc::Receiver, Arc, Mutex},
	thread::{self, JoinHandle},
};

pub trait Job = FnOnce() + Send + 'static;
pub type BoxedJob = Box<dyn Job>;

pub enum Task {
	New(BoxedJob),
	Exit,
}

pub struct Worker {
	id: u16,
	handle: Option<JoinHandle<()>>,
}

impl Worker {
	pub fn new(id: u16, receiver: Arc<Mutex<Receiver<Task>>>) -> Self {
		let handle = thread::spawn(move || {
			while let Task::New(job) = receiver.lock().unwrap().recv().unwrap() {
				job();
			}
		});

		Self {
			id,
			handle: Some(handle),
		}
	}
}

impl Drop for Worker {
	fn drop(&mut self) {
		self
			.handle
			.take()
			.unwrap()
			.join()
			.unwrap_or_else(|_| panic!("Failed to join worker with ID: {}", self.id));
	}
}
