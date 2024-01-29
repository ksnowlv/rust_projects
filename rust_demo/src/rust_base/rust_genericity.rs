
#[derive(Debug)]
struct Point<T> {
	x: T,
	y: T
}

impl <T> Point<T> {
	fn x(&self) -> &T {
		&self.x
	}

	fn y(&self) -> &T {
		&self.y
	}
}

pub fn genericity_test() {
	println!("genericity_test()");
	let p1 = Point{x:1, y:2};
	println!("p1:{:?}, x:{}, y:{}", p1, p1.x(), p1.y());
	let p2 = Point{x:10.0, y: 20.0};
	println!("p2:{:?}", p2);

}
