// box_basics.rs

fn box_ref<T>(b: T) -> Box<T> {
    let a = b;
    Box::new(a)
}

struct Foo;

#[derive(Debug)]
struct Value(u8);

fn main() {
    let boxed_one = Box::new(Foo); // we created a heap allocated value in boxed_one
    let unboxed_one = *boxed_one;
    box_ref(unboxed_one);


    let boxed_two = Box::new(Value(3));
    let unboxed_two = *boxed_two;
    box_ref(&unboxed_two);

    println!("{:?}", unboxed_two);
}