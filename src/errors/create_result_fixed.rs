// create_result_fixed.rs

fn main() {
	let _my_result: Result<_,()> = Ok(64);

	let _or_my_result = Ok::<_, ()>(64);

	// similarly we create Err variants

	let _my_err = Err::<(), f32>(435.3);
	let _other_err: Result<bool, String> = Err("Wait, what?".to_string());
}