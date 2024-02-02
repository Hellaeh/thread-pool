#![feature(test)]
extern crate test;

use hel_thread_pool::ThreadPool;
use test::Bencher;

#[bench]
fn execute(b: &mut Bencher) {
	let pool = ThreadPool::new();

	b.iter(|| pool.execute(|| {}));
}
