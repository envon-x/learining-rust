use std::fmt::Display;
use std::fmt::Debug;

struct Foo<T: Display> {
  bar: T
}

struct Bar<F> where F: Display {
  inner: F
}

trait Eatable {
    fn eat(&self);
}


#[derive(Debug)]
pub struct Food<T>(T);

#[derive(Debug)]
pub struct Apple;

impl<T> Eatable for Food<T> where T: Debug {
  fn eat(&self) {
    println!("Eating {:?}", self);
  }
}

fn eat<T>(val: T) where T: Eatable {
  val.eat();
}

