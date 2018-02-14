use std::sync::{RwLock, Mutex};

pub type Xyz = Vec<u8>;
pub type Foo = Vec<u8>;
pub type Bar = Vec<u8>;

/// This structure creates a safe parallel access to it's fields
#[derive(Default)]
pub struct SafeContainer {
	foo: RwLock<Foo>,
	xyz: Mutex<Xyz>,
	bar: RwLock<Bar>,
}

pub mod safe_container_lo {
	use std::sync::{RwLockReadGuard, RwLockWriteGuard, MutexGuard};
	use super::{Xyz, Foo, Bar};
	
	pub struct ReadUntilFoo<'a> {
		pub foo: RwLockReadGuard<'a, Foo>,
	}

	pub struct WriteUntilFoo<'a> {
		pub foo: RwLockWriteGuard<'a, Foo>,
	}

	pub struct LockUntilXyz<'a> {
		pub foo: RwLockWriteGuard<'a, Foo>,
		pub xyz: MutexGuard<'a, Xyz>,
	}

	pub struct ReadUntilBar<'a> {
		pub foo: RwLockReadGuard<'a, Foo>,
		pub bar: RwLockReadGuard<'a, Bar>,
	}

	pub struct WriteUntilBar<'a> {
		pub foo: RwLockWriteGuard<'a, Foo>,
		pub xyz: MutexGuard<'a, Xyz>,
		pub bar: RwLockWriteGuard<'a, Bar>,
	}
}

impl SafeContainer {
	pub fn read_until_foo(&self) -> safe_container_lo::ReadUntilFoo {
		safe_container_lo::ReadUntilFoo {
			foo: self.foo.read().unwrap(),
		}
	}

	pub fn write_until_foo(&self) -> safe_container_lo::WriteUntilFoo {
		safe_container_lo::WriteUntilFoo {
			foo: self.foo.write().unwrap(),
		}
	}

	pub fn lock_until_xyz(&self) -> safe_container_lo::LockUntilXyz {
		safe_container_lo::LockUntilXyz {
			foo: self.foo.write().unwrap(),
			xyz: self.xyz.lock().unwrap(),
		}
	}


	pub fn read_until_bar(&self) -> safe_container_lo::ReadUntilBar {
		safe_container_lo::ReadUntilBar {
			foo: self.foo.read().unwrap(),
			bar: self.bar.read().unwrap(),
		}
	}

	pub fn write_until_bar(&self) -> safe_container_lo::WriteUntilBar {
		safe_container_lo::WriteUntilBar {
			foo: self.foo.write().unwrap(),
			xyz: self.xyz.lock().unwrap(),
			bar: self.bar.write().unwrap(),
		}
	}
}

pub fn main() {
	let safe_container = SafeContainer::default();
	let _ = safe_container.read_until_foo();
	let _ = safe_container.read_until_bar();
	let _ = safe_container.lock_until_xyz();
	let _ = safe_container.write_until_foo();
	let _ = safe_container.write_until_bar();
}
