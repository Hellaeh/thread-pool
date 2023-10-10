use std::sync::mpsc;

use hel_thread_pool::ThreadPool;

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

	let work = move |_| {
		for _ in 0..1_000_000 {}
	};

	for i in 0..thread_pool.capacity() {
		thread_pool.execute(move || work(i))
	}
}

#[test]
fn using_channels() {
	let thread_pool = ThreadPool::new();
	let cap = thread_pool.capacity();

	let (sender, receiver) = mpsc::channel();

	for _ in thread_pool.iter() {
		let sender = sender.clone();

		thread_pool.execute(move || {
			let mut result = 0usize;

			for i in 0..1_000_000 {
				result += i
			}

			sender.send(result).unwrap();
		})
	}

	drop(sender);

	let mut total = 0usize;
	while let Ok(result) = receiver.recv() {
		total += result;
	}

	assert_eq!(total, (1_000_000 * 999_999) / 2 * cap)
}
