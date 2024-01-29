
pub fn control_condition_test() {

	//if-else if
	let n = 10;

	if n > 20  {
		println!("{} > 20", n);
	} else if n > 10 {
		println!("{} > 10", n);
	} else  if n == 10 {
		println!("{} == 10", n);
	} else {
		println!("{} < 10", n);
	}

	let res = if n > 10 {1} else { -1 };
	println!(" res:{}", res);

	//类似switch case的match
	let  mut color = Color::Blue;

	match color {
		Color::Red => println!("The color is red"),
		Color::Green => println!("The color is green"),
		Color::Blue => println!("The color is blue"),
	}

	color = Color::Red;

	match color {
		Color::Red => println!("The color is red"),
		Color::Green => println!("The color is green"),
		Color::Blue => println!("The color is blue"),
	}

	color = Color::Green;
	match color {
		Color::Red => println!("The color is red"),
		Color::Green => println!("The color is green"),
		Color::Blue => println!("The color is blue"),
	}

	// while

	let mut count = 0;

	while count != 10 {
		println!("count:{}", count);
		count += 1;
	}

	//for
	for i in 0..10  {
		println!("i:{}", i);
	}

	// vec
	let v = vec![1, 2, 3,4,5, 6];
	for i in v  {
		println!("i:{}", i);
	}

	// iter
	let array = [1, 2, 3, 4, 5, 6];

	for x in array.iter() {
		println!("x:{}", x);
	}

	// rev
	for i in (0..10).rev() {
		println!("i:{}", i);
	}

}

enum Color {
	Red,
	Green,
	Blue,
}

