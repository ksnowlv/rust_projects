use rayon::prelude::*;
pub fn thread_pool_test() {

	let mut  v = Box::new(Vec::new());
	let v_size: u64 = 10000000;
	v.reserve(v_size.try_into().unwrap());
	for i in 0..v_size  {
		v.push(i);
	}

	let sum:  u64 = v.into_par_iter().map(|i| {
			i
		})
		.sum();

	println!("v sum:{}", sum);
}
