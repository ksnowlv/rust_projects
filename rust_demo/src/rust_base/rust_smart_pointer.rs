use std::rc::Rc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::cell::{Cell, RefCell};

pub fn smart_pointer_test() {
	println!("---smart_pointer_test---");
	//Box
	box_pointer_test();
	//Rc
	rc_pointer_test();
	//arc
	arc_pointer_test();
	//
	refcell_pointer_test();
	//cell
	cell_test();

}

fn box_pointer_test() {
	//用于在堆上分配内存，并在编译时保证数据的所有权唯一。适用于在编译时无法确定大小或需要在多个位置共享所有权的情况。

	let x = Box::new(10.0);
	println!("x:{}", x);

	let y = Box::new("hello Box");
	println!("y:{}", y);

	let vec = Box::new(vec![1, 2, 3, 4, 5, 6]);

	println!("vec:{:?}", vec);
}

fn rc_pointer_test()  {
	//需要在 Rust 中进行引用计数共享所有权时，可以使用 Rc<T> 智能指针
	let data = Rc::new("hello");
	println!("data Rc count:{}", Rc::strong_count(&data));
	let clone_data = Rc::clone(&data);
	println!("clone_data Rc count:{}", Rc::strong_count(&clone_data));

	{
		let d = Rc::clone(&data);
		println!("data Rc count:{}, d1 rc count:{}", Rc::strong_count(&data), Rc::strong_count(&d));
	}

	println!("data res Rc count:{}", Rc::strong_count(&data));
}

fn arc_pointer_test() {
	//在 Rust 中进行跨线程共享所有权时，可以使用 Arc<T> 智能指针。
	// 示例代码，如何使用 Arc<T> 来创建引用计数的数据，以便在多线程环境中共享：
	// 同时需要使用 Mutex 或 RwLock 来实现内部数据的可变性控制。
	//let data = Arc::new( vec![1, 2, 3, 4, 5]);
	let data = Arc::new(Mutex::new(vec![1, 2, 3]));
	let data_clone = Arc::clone(&data);

	let handle = thread::spawn(move || {
		let mut data = data_clone.lock().unwrap();
		println!("child thread data {:?}", data);

		for i in data.iter_mut() {
			*i += 5;
		}

		println!("child thread update data {:?}", data);
	});

	handle.join().unwrap();
	let res = data.lock().unwrap();
	println!("main thread data:{:?} ", res);
}

fn refcell_pointer_test() {

	//RefCell: 提供了内部可变性，允许在不可变引用的同时对数据进行修改。适用于在编译时无法确定是否需要可变性的场景

	//let data = RefCell::new(vec![1, 2, 3]);
	let data = RefCell::new(vec![1, 2, 3, 4, 5, 6]);

	// 获取可变借用
	{
		let mut borrowed_data = data.borrow_mut();
		for i in 100..110  {
			borrowed_data.push(i);
		}
	}

	// 获取不可变借用
	let borrowed_data = data.borrow();
	for i in borrowed_data.iter() {
		println!("Data: {}", i);
	}
}

fn cell_test() {
	let data = Arc::new(Mutex::new(Cell::new(0)));

	let threads: Vec<_> = (0..6).map(|_| {
		let thread_data = Arc::clone(&data);
		thread::spawn(move || {
			for  _ in 0..10 {
				let value = thread_data.lock().unwrap();
				let cur_value = value.get();
				value.set(cur_value + 100);
			}
		})
	}).collect();

	for thread_item in threads  {
		thread_item.join().unwrap();
	}

	println!("Final data value: {}", data.lock().unwrap().get());
}
