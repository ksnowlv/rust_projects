
//特性（trait）概念接近于 Java 中的接口（Interface），但两者不完全相同。特性与接口相同的地方在于它们都是一种行为规范，可以用于标识哪些类有哪些方法。


trait BaseClassInterface {
	fn description(&self) -> String;
}

struct Student {
	id: String,
	name: String,
	age:u64
}

impl BaseClassInterface for Student {
	fn description(&self) -> String {
		format!("id:{}, name:{}, age:{}", self.id, self.name, self.age)
	}
}

//特性做参数
//我们需要传递一个函数做参数，例如回调函数、设置按钮事件等;在 Rust 中可以通过传递特性参数来实现：
fn output_infomation(object: impl BaseClassInterface) {
	println!("{}", object.description());
}

//特性做返回值
fn student_info() -> Box<dyn  BaseClassInterface> {
	Box::new(Student {
		id: String::from("111"),
		name: String::from("ksnowlv"),
		age:10
	})
}

pub fn trait_test() {
	println!("---trait_test---");
	let s = Student{
		id: String::from("111"),
		name: String::from("ksnowlv"),
		age:10
	};
	println!("s:{}", s.description());

	//特性做参数
	output_infomation(s);

	let s1 = student_info();
	println!("s1:{}", s1.description());
}
