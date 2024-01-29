use std::sync::{Arc, Condvar, mpsc, Mutex};
use std::thread;
use std::time::Duration;

pub fn thread_communication_test() {
	println!("---thread_communication_test---");
	mutex_test();
	channel_communication_test();
	condvar_mutex_test();
	shared_memory_test();
}

fn mutex_test() {
	//Mutex 进行线程间共享数据访问
	println!("---mutex_test---");
	let shared_data = Arc::new(Mutex::new(0));
	let shared_data_clone = Arc::clone(&shared_data);

	let handle = thread::spawn(move || {
		let mut thread_data = shared_data_clone.lock().unwrap();
		*thread_data = 1000;
		println!("thread id= {:?}, thread_data: {}", thread::current().id(), *thread_data);
	});

	// wait for child thread
	handle.join().unwrap();

	let res = shared_data.lock().unwrap();
	println!("Final data value:{}", *res);
}

fn channel_communication_test() {
	let (tx, rx) = mpsc::channel();
	let handle = thread::spawn(move || {
		let data = "I am  a child thread";
		tx.send(data).unwrap();
	});

	let received_data = rx.recv().unwrap();
	println!("main thread received data value:{}", received_data);
	handle.join().unwrap();
}

fn condvar_mutex_test() {
	//使用 Condvar 和 Mutex 进行线程间同步和通知
	println!("---condvar_mutex_test---");
	let pair = Arc::new((Mutex::new(false), Condvar::new()));
	let pair_clone = Arc::clone(&pair);

	let handle = thread::spawn(move || {
		thread::sleep(Duration::from_secs(2));
		let (lock, cvar) = &*pair_clone;
		let mut exit_flag = lock.lock().unwrap();

		*exit_flag = true;
		cvar.notify_one();
	});

	let (lock, cvar) = &*pair;
	let mut exit_flag = lock.lock().unwrap();
	while !*exit_flag {
		println!("main thread exit_flag:{}", *exit_flag);
		exit_flag = cvar.wait(exit_flag).unwrap();
	}

	println!("main thread receive exit_flag status:{}", *exit_flag);
	handle.join().unwrap();
	println!("---condvar_mutex_test---end ");
}

fn shared_memory_test() {
	//在 Rust 中，可以使用共享内存进行消息传递，一种常见的方式是使用 Arc (原子引用计数) 和 Mutex 来实现共享内存和线程安全的消息传递
	println!("---shared_memory_test---");
	let message_queue = Arc::new(Mutex::new(Vec::new()));

	let message_queue_producer = Arc::clone(&message_queue);

	let producer_handle = thread::spawn(move || {
		for i in 0..10 {
			let mut queue = message_queue_producer.lock().unwrap();
			queue.push(format!(" producer handle message:{}", i));
			println!("producer thread Message {}", i);
		}
	});

	let message_queue_consumer = Arc::clone(&message_queue);
	let consumer_handle = thread::spawn(move || {
		for _i in 0..10 {

			let message = {
				let mut queue = message_queue_consumer.lock().unwrap();
				queue.pop()
			};

			if let Some (msg) = message {
				println!("consumer_handle consumer:{}", msg);
			}
		}
	});

	producer_handle.join().unwrap();
	consumer_handle.join().unwrap();
}

