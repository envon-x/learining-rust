use std::fmt::Debug;

#[derive(Debug)]
pub struct Square(f32);

#[derive(Debug)]
pub struct Rectangle(f32, f32);


trait Area: Debug {
  fn calculate(&self) -> f32;
}

impl Area for Square {
    fn calculate(&self) -> f32 {
        self.0 * self.0
    }
}

impl Area for Rectangle {
    fn calculate(&self) -> f32 {
        self.0 * self.1
    }
}

fn main() {
  //
  let shapes: Vec<&dyn Area> = vec![&Square(3f32), &Rectangle(4f32, 2f32)];
  for s in shapes {
    println!("Area calculada: {:?}",s.calculate());
  }
}

// /**
// $ rustc src/trait_objects.rs
// $ ./trait_objects
// Area calculada: 9.0
// Area calculada: 8.0
//  */