//要导入调试库 #[derive(Debug)] ，之后在 println 和 print 宏中就可以用 {:?} 占位符输出一整个结构体
#[derive(Debug)]
struct Person {
	name: String,
	phone: String,
	address: String,
	age: i64,
	sex: bool
}

impl Person {
	fn show_person(&self) {
		println!("name:{}, phone:{}, address:{}, age:{} sex:{}", self.name,
		self.phone, self.address, self.age, self.sex);
	}
}

pub fn  struct_test() {

	//结构体
	let  person = Person{
		name:String::from("ksnowlv"),
		phone: String::from("15210111111"),
		address: String::from("Beijing"),
		age: 30,
		sex:true
	};

	person.show_person();
	println!("person:{:?}", person);

	//元组结构体

	#[derive(Debug)]
	struct Point(f64, f64);

	let pos = Point(10.0, 11.0);
	println!("point:{{x:{},y:{}}}", pos.0, pos.1);
	println!("point:{:?}", pos);
}
