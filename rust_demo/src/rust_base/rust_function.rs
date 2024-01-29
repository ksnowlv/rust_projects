
/*fn <函数名> ( <参数> ) <函数体> */

fn add(a: i64, b: i64) -> i64 {
	return   a + b;
}

fn subtract(a: f64, b: f64) -> f64 {
	a - b
}

fn get_values() -> (i32, f64, bool) {
	(1, 2.1, true)
}

fn get_vectors() -> Vec<i64> {
	let mut v = Vec::new();

	for i  in 0..10  {
		v.push(i);
	}

	v
}

struct Person {
	name: String,
	age: u8,
}

impl Person {
	fn new(name: String, age: u8) -> Person {
		Person{
			name,
			age
		}
	}
	fn show_infomation(&self) {
		println!("name:{}, age:{}", self.name, self.age);
	}
}

pub fn function_test() {
	let c = add(1,2);
	println!("1 + 2 = {}", c);

	let d = subtract(21.0,1.1);
	println!("21.0 - 1.1 = {}", d);

	let (x, y, z) = get_values();
	println!("The values are: {}, {}, {}", x, y, z);

	let v = get_vectors();
	println!("vectors:{:?}", v);

	let p = Person::new(String::from("ksnowlv"), 10);
	p.show_infomation();
}
