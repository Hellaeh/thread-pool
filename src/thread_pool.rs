mod worker;

use std::sync::{
	mpsc::{self, Sender},
	Arc, Mutex,
};

use crate::utils::optimal_number_of_threads;

use self::worker::{Job, Task, Worker};

pub struct ThreadPool {
	_workers: Vec<Worker>,
	sender: Sender<Task>,
	capacity: u16,
}

impl ThreadPool {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn with_capacity(capacity: u16) -> Self {
		let mut _workers = Vec::with_capacity(capacity as usize);

		let (sender, receiver) = mpsc::channel();

		let receiver = Arc::new(Mutex::new(receiver));

		for id in 0..capacity {
			_workers.push(Worker::new(id, receiver.clone()));
		}

		Self {
			_workers,
			sender,
			capacity,
		}
	}

	pub fn capacity(&self) -> u16 {
		self.capacity
	}

	pub fn execute<T: Job>(&self, f: T) {
		self.sender.send(Task::New(Box::new(f))).unwrap()
	}
}

impl Drop for ThreadPool {
	fn drop(&mut self) {
		for _ in 0..self.capacity() {
			self.sender.send(Task::Exit).unwrap()
		}
	}
}

impl Default for ThreadPool {
	fn default() -> Self {
		Self::with_capacity(optimal_number_of_threads(u16::MAX))
	}
}

impl<'a> IntoIterator for &'a ThreadPool {
	type Item = u16;

	type IntoIter = std::ops::Range<u16>;

	fn into_iter(self) -> Self::IntoIter {
		0..self.capacity()
	}
}
