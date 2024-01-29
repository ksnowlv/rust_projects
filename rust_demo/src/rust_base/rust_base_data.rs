pub fn base_data_test() {
	let a = 123;
	let b = 1.2345;
	let s = "abc";
	let s1 = s;
	println!("a = {}, b = {} ,s = {} s1 = {}", a, b, s, s1);

	let mut str = String::from("hello");
	str += " world";
	println!("str:hello + world = {}", str);
	//克隆,如何str_res 指向str，则str失效，需要 str.clone()才能保证str依然有效。
	let str_res = str.clone();
	println!("str:{} str_res {}", str, str_res);
	//引用（Reference）引用不会获得值的所有权。引用只能租借（Borrow）值的所有权
	let str_ref = &str;
	println!("str:{} str_ref {}", str, str_ref);

	let mut str_mut = String::from("Hello ");
	let str_mut_ref = & mut str_mut;
	str_mut_ref.push_str(" world!");
	println!("str_mut_ref:{}", str_mut_ref);

	let c :u64 = 123;
	println!("c= {}", c);
	let c = c + 100;
	println!("c + 100 = {}", c);

	let  x = 2.0;
	let y:f32 = 3.0;
	let sum = x + y;

	println!(" x + y = {}", sum);
	//元组，可以包含不同种类的数据
	let tup: (i32, f64, u8, char) = (100, 1.2, 1, 'a');
	let (mut a, mut b, c, d) = tup;
	println!("tup: {}, {}, {}, {}", tup.0, tup.1, tup.2, tup.3);
	println!("{:?}", (a, b, c, d));
	a = 3;
	b = 3.0;
	println!("{:?}", (a, b, c, d));

	//数组
	let int_array = [1, 2, 3, 4, 5, 6, 7, 8];
	println!("array:{:?}", int_array);

	//数组
	let city_array = ["beijing", "tianjin", "shanghai", "chongqing"];
	println!("city_array:{:?}", city_array);

	let int_array_with_fix_len:[i32;5] = [1,2, 3,4, 5];
	println!("int_array_with_fix_len:{:?}", int_array_with_fix_len);

	let repeat_array = [1; 10];
	println!("fix_array[1;10]:{:?}", repeat_array);

	let mut mut_array = [1, 2, 3];
	mut_array[0] = 2;
	mut_array[2] = 1;
	println!("mut_array:{:?}", mut_array);

}


