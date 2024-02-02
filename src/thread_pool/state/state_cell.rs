use std::sync::atomic;

#[derive(Default)]
pub struct StateCell(atomic::AtomicU64);

impl StateCell {
	#[inline(always)]
	pub fn add(&self, bit: u64) {
		self.0.fetch_or(bit, atomic::Ordering::Relaxed);
	}

	#[inline(always)]
	pub fn remove(&self, bit: u64) {
		self.0.fetch_and(!bit, atomic::Ordering::Relaxed);
	}

	#[inline(always)]
	pub fn count(&self) -> usize {
		self.read_relax().count_ones() as usize
	}

	#[inline(always)]
	fn read_relax(&self) -> u64 {
		self.0.load(atomic::Ordering::Relaxed)
	}
}

impl std::fmt::Debug for StateCell {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{:b}", self.read_relax())
	}
}

unsafe impl Send for StateCell {}
unsafe impl Sync for StateCell {}
