// multiple_lifetimes.rs

#[derive(Debug)]
struct Decoder<'a, 'b, S, R> {
	schema: &'a S,
	reader: &'b R,
}

fn main() {
	unimplemented!();
}