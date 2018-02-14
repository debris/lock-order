use std::sync::{RwLock, Mutex, RwLockReadGuard, RwLockWriteGuard, MutexGuard};

pub type Xyz = Vec<u8>;
pub type Foo = Vec<u8>;
pub type Bar = Vec<u8>;

/// This structure creates a safe parallel access to it's fields
pub struct SafeContainer {
	pub foo: RwLock<Foo>,
	pub xyz: Mutex<Xyz>,
	pub bar: RwLock<Bar>,
}

pub fn main() {
}
