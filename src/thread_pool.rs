mod worker;

use std::sync::{
	mpsc::{self, Sender},
	Arc, Mutex,
};

use crate::utils::optimal_number_of_threads;

use self::worker::{Task, Worker};

///
#[derive(Debug)]
pub struct ThreadPool {
	_workers: Vec<Worker>,
	sender: Sender<Task>,
	capacity: usize,
}

impl ThreadPool {
	/// Will return [`ThreadPool`] with `capacity = num of cpu`
	pub fn new() -> Self {
		Self::with_capacity(optimal_number_of_threads(u16::MAX as usize))
	}

	/// Will return [`ThreadPool`] with user defined capacity
	pub fn with_capacity(capacity: usize) -> Self {
		let mut _workers = Vec::with_capacity(capacity);

		let (sender, receiver) = mpsc::channel();

		let receiver = Arc::new(Mutex::new(receiver));

		for _ in 0..capacity {
			_workers.push(Worker::new(receiver.clone()));
		}

		Self {
			_workers,
			sender,
			capacity,
		}
	}

	/// Returns [`ThreadPool`] capacity
	#[inline]
	pub fn capacity(&self) -> usize {
		self.capacity
	}

	/// Executes passed function in
	#[inline]
	pub fn execute<F: FnOnce() + Send + 'static>(&self, f: F) {
		self.sender.send(Task::New(Box::new(f))).unwrap()
	}

	/// Returns an iterator for capacity
	#[inline]
	pub fn iter(&self) -> std::ops::Range<usize> {
		0..self.capacity
	}

	/// Does not actually call `join` on a thread
	/// Instead breaks from internal loop
	pub fn join(self) {
		drop(self)
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
	fn default() -> Self {
		Self::new()
	}
}
