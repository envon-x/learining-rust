#[derive(Debug)]
struct SomeRef<'a, T> {
  part: &'a T, 
}

fn main() {
  let a = SomeRef{part: &43}; // fn get_a_borrowed_value() -> &u8 { 
  //||                            ^ expected named lifetime parameter
  //|

  println!("Probando: {:?}", a.part);


  let b: &'static str = "I live forever";

  println!("probando nuevamente: {:?}", b);

  let c = SomeRef{
    part: &b
  };

  println!("Esto es un string: {:?}", c)

}