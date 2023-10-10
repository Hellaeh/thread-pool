use std::thread;

const MINIMAL_NUMBER_OF_THREADS: usize = 2;

pub fn optimal_number_of_threads(capacity: usize) -> usize {
	let total_max_threads = thread::available_parallelism()
		.expect("Could not get available logical cores")
		.get()
		- 1;

	match capacity.min(total_max_threads) {
		0 => MINIMAL_NUMBER_OF_THREADS,
		v => v,
	}
}
