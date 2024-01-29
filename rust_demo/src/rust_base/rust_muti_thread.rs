use std::pin::Pin;
use std::sync::{Arc, Condvar, Mutex, RwLock};
use std::thread;
use std::time::Duration;

pub fn muti_thread_test() {
	mutex_test();
	rwlock_test();
	pin_test();
	cond_var_test();
}

fn mutex_test() {
	println!("---mutex_test---");
	//当使用 Rust 的 Mutex 智能指针时，一般情况下会将内部数据封装在 Arc 和 Mutex 中，以确保多线程下的安全性
	let shared_data = Arc::new(Mutex::new(0));
	let threads: Vec<_> = (0..6).map(|i| {
		let thread_data = Arc::clone(&shared_data);
		thread::spawn(move || {
			for j in 0..10  {
				let mut data = thread_data.lock().unwrap();
				*data += i *  j;
			}
		})

	}).collect();



	for t in threads  {
		t.join().unwrap();
	}

	println!("Final value: {}", *shared_data.lock().unwrap());
}

fn  rwlock_test() {
	// Rust 中的 RwLock（读写锁），可以使用 Arc 和 RwLock 结合起来，以实现多线程下的读写安全。
	println!("---rwlock_test---");
	let shared_data = Arc::new(RwLock::new(0));
	let exit_flag = Arc::new(Mutex::new(false));

	let read_threads:Vec<_> = (0..5).map(|_x| {
		let thread_data = Arc::clone(&shared_data);
		//let thread_flag = Arc::clone(&exit_flag);
		let thread_flag = Arc::clone(&exit_flag);
		thread::spawn(move || {
			loop {
				let  data = thread_data.read().unwrap();
				println!("read thread {:?}: data = {}", thread::current().id(), *data);
				let flag = thread_flag.lock().unwrap();
				if *flag {
					println!("read thread {:?}: flag = {}", thread::current().id(), *flag);
					break;
				}
			}

			println!("read thread end!");
		})
	}).collect();

	let write_threads: Vec<_> = (0..3).map(|_x| {
		let thread_data = Arc::clone(&shared_data);
		let thread_flag = Arc::clone(&exit_flag);
		thread::spawn(move || {
			for i in 0..3 {
				let  mut  data = thread_data.write().unwrap();
				*data = i;
				println!("write thread {:?}: data = {}", thread::current().id(), *data);
				thread::sleep(Duration::from_secs(2));
			}

			let mut flag_data = thread_flag.lock().unwrap();
			*flag_data = true;

			println!("write thread end!");
		})
	}).collect();


	//wait for write
	for write_t in write_threads  {
		write_t.join().unwrap();
	}

	//read
	for read_thread in read_threads  {
		read_thread.join().unwrap();
	}

	let final_value = shared_data.read().unwrap();
	println!("rwlock_test Final value: {}", *final_value);
}


struct XData {
	value: i64,
}

impl XData {
	fn new(value: i64) -> Self {
		XData{value}
	}
}

fn pin_test() {
	//Pin<Box<T>> 主要用于将值固定在内存中的堆上，以防止其被移动。这在处理涉及异步操作或存在自引用数据结构的情况下特别有用
	//使用 as_ref 方法获取指向固定数据的引用。值得注意的是，一旦数据被固定，就无法再次移动它。
	println!("---pin_test---");
	let data = XData::new(100);
	let pin_data: Pin<Box<XData>> = Box::pin(data);
	let pin_data_ref = pin_data.as_ref();
	println!("pin data: {}", pin_data_ref.value);
}

fn cond_var_test() {
	println!("---cond_var_test---");
	let pair = Arc::new((Mutex::new(false), Condvar::new()));
	let pair_clone_data = Arc::clone(&pair);
	thread::spawn(move || {
		let (lock, cvar) = &*pair_clone_data;
		let mut exit_flag = lock.lock().unwrap();

		while !*exit_flag {
			println!("exit_flag = {}", *exit_flag);
			exit_flag = cvar.wait(exit_flag).unwrap();
		}
		println!("Thread end, exit_flag = {:?}", *exit_flag);
		thread::sleep(Duration::from_secs(1))
	});

	thread::sleep(Duration::from_micros(500));
	let (lock, cvar) = &*pair;
	let mut exit_flag = lock.lock().unwrap();
	*exit_flag = true;
	cvar.notify_one();
	println!("main thread exit_flag = {}", *exit_flag);
	// 等待子线程执行完毕，并添加一些延迟等待的时间
	//handle.join().unwrap();
	thread::sleep(Duration::from_secs(1));
	println!("cond_var_test end");
}
