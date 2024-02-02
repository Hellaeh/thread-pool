use std::{sync::mpsc, time::Duration};

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

#[test]
fn busy_count() {
	let thread_pool = ThreadPool::new();

	let cap = thread_pool.capacity();

	let test_cap = cap * 8;

	for _ in 0..test_cap {
		thread_pool.execute(|| std::thread::sleep(Duration::from_secs(1)));
	}

	std::thread::sleep(Duration::from_millis(100));

	assert_eq!(thread_pool.check_busy(), cap.min(test_cap));
}

#[test]
fn panic_count() {
	let thread_pool = ThreadPool::with_capacity(20);

	thread_pool.execute(|| panic!("Panic"));
	thread_pool.execute(|| println!("Work"));
	thread_pool.execute(|| println!("Work"));
	thread_pool.execute(|| panic!("Panic"));

	std::thread::sleep(Duration::from_millis(100));

	let panics = thread_pool.check_panics();

	assert_eq!(panics, 2);
}
