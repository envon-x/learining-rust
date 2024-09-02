// multiple_lifetimes.rs

#[derive(Debug)]
struct Decoder<'a, 'b, S, R> {
	schema: &'a S,
	reader: &'b R,
}

impl<'a, 'b, S, R> Decoder<'a, 'b, S, R> where 'a: 'b {

}

fn main() {
	let a: Vec<u8> = vec![1,2,3,4,5];
	let b: Vec<u8> = vec![1,3,5];

	let decoder = Decoder{schema: &a, reader: &b};

	println!("{:?}", decoder);
}