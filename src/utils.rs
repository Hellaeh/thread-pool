use std::{num::NonZeroUsize, thread};

const DEFAULT_NUMBER_OF_THREADS: u16 = 4;

pub fn optimal_number_of_threads(capacity: u16) -> u16 {
	let total_max_threads = thread::available_parallelism()
		.unwrap_or(NonZeroUsize::new(4).unwrap())
		.get() as u16;

	match capacity.min(total_max_threads) {
		0 => DEFAULT_NUMBER_OF_THREADS,
		v => v,
	}
}
