use std::thread;

use crate::thread_pool::MAX_THREADS;

const MINIMAL_NUMBER_OF_THREADS: usize = 2;

pub fn optimal_number_of_threads(capacity: usize) -> usize {
	let available_threads = thread::available_parallelism()
		.expect("Could not get available logical cores")
		.get()
		- 1;

	match capacity.min(available_threads) {
		0 => MINIMAL_NUMBER_OF_THREADS,
		v => v.min(MAX_THREADS),
	}
}
