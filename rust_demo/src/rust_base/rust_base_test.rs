use std::cmp::{max, min};
use crate::rust_base::rust_base_data::base_data_test;
use crate::rust_base::rust_control_condition::control_condition_test;
use crate::rust_base::rust_function::function_test;
use crate::rust_base::rust_struct::struct_test;
use std::f64::consts::PI;
use crate::rust_base::rust_atomic_test::atomic_test;
use crate::rust_base::rust_genericity::genericity_test;
use crate::rust_base::rust_muti_thread::muti_thread_test;
use crate::rust_base::rust_smart_pointer::smart_pointer_test;
use crate::rust_base::rust_thread_communication::thread_communication_test;
use crate::rust_base::rust_thread_pool::thread_pool_test;
use crate::rust_base::rust_trait::trait_test;

pub fn rust_base_test() {
	base_data_test();
	function_test();
	control_condition_test();
	struct_test();
	println!("PI:{}", (PI/2.0).sin());
	println!(" max(10, 2) = {}, min(10, 2) = {}", max(10, 2), min(10, 2));
	genericity_test();
	trait_test();
	smart_pointer_test();
	muti_thread_test();
	atomic_test();
	thread_communication_test();
	thread_pool_test();
}
