struct Complex<T> {
  re:T,
  im:T,
}

impl<T> Complex<T> {
  fn new(re: T, im: T) -> Self {
    Complex {re, im}
  }
}

#[cfg(test)]
mod tests {
  use Complex;
  #[test]
  fn complex_basic() {
    let first = Complex::new(2,5);
    let second: Complex::default();
    assert_eq!(first.re, 3);
    assert_eq!(first.im, 5);
    assert!(second.re == second.im);
  }
}