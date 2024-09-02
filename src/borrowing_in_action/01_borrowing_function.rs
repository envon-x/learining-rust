fn take_the_n(n: &mut u8) {
  *n += 3;
}

fn take_the_s(s: &mut String) {
  s.push_str("ing");
}

fn take_and_return_s(ss: &mut String) ->&mut String {
  ss.push_str("_texto adicionado");
  ss
}

fn main() {
  let mut n = 5;
  let mut s = String::from("Texto prestado");

  take_the_n(&mut n);
  take_the_s(&mut s);

  println!("Valor original 5, nuevo valor de n: {}", n);
  println!("s fue cambiado al siguiente texto: {}", s);

  // take_and_return_s(&mut s);

  // println!("Valor retornado: {}", s);

}

























