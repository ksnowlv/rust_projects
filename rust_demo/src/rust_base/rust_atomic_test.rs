use std::sync::atomic::{AtomicI64, AtomicUsize, Ordering};

pub fn atomic_test() {
	println!("---atomic_test---");
	atomic_load_store();
	atomic_exchange();
	atomic_compare_swap();
	atomic_add_and_sub();
}

fn atomic_load_store() {
	println!("---atomic_load_store---");
	let atomic_usize = AtomicUsize::new(10);
	let v = atomic_usize.load(Ordering::SeqCst);
	println!("value:{}", v);
	atomic_usize.store(100, Ordering::SeqCst);
	let new_value = atomic_usize.load(Ordering::SeqCst);
	println!("atomic_int.store value:{}", new_value);
}

fn  atomic_exchange() {
	println!("---atomic_exchange---");
	let atomic_int = AtomicI64::new(100);
	let pre_value = atomic_int.swap(1000, Ordering::SeqCst);
	println!("pre value:{}", pre_value);
}

fn atomic_compare_swap() {
	println!("---atomic_compare_swap---");
	let atomic_int = AtomicI64::new(100);
	let pre_value = atomic_int.compare_exchange(100, 1000,  Ordering::SeqCst, Ordering::Relaxed);
	match pre_value {
		Ok(value) => println!("New value: {}", value),
		Err(value) => println!("Error, current value: {}", value),
	}
}

fn atomic_add_and_sub() {
	let atomic_num = AtomicUsize::new(5);
	atomic_num.fetch_add(10, Ordering::SeqCst);
	println!("value after add:{}", atomic_num.load(Ordering::SeqCst));

	atomic_num.fetch_sub(2, Ordering::SeqCst);
	println!("value after sub:{}", atomic_num.load(Ordering::SeqCst));
}
