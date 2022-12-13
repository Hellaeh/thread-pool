mod worker;

use std::sync::{
	mpsc::{channel, Sender},
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
		Self::with_capacity(optimal_number_of_threads(u16::MAX))
	}

	pub fn with_capacity(capacity: u16) -> Self {
		let mut _workers = Vec::with_capacity(capacity as usize);

		let (sender, receiver) = channel();

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

	pub fn execute<T>(&self, f: T)
	where
		T: Job,
	{
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

#[cfg(test)]
mod test {
	use super::ThreadPool;

	#[test]
	fn graceful_shutdown() {
		let thread_pool = ThreadPool::with_capacity(4);

		for i in 0..thread_pool.capacity() {
			thread_pool.execute(move || assert_eq!(i, i));
		}
	}

	#[test]
	fn loops() {
		let thread_pool = ThreadPool::new();

		let mut i = 0;
		let work = move || loop {
			if i >= 1e6 as u32 {
				break;
			}

			i += 1;
		};

		for _ in 0..thread_pool.capacity() {
			thread_pool.execute(work)
		}

		thread_pool.execute(|| assert!(true))
	}
}
