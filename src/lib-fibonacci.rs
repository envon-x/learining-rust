/**
* let work_a = 4;
let work_b = 34;
let total_work = sum(work_a, work_b)
 */
fn sum(a: i8, b:i8) -> i8 {
  a + b
}

fn slow_fibonacci(nth: usize) -> u64 {
  if nth <= 1 {
      return nth as u64;
  } else {
  return slow_fibonacci(nth -1) + slow_fibonacci(nth - 2);
  
  }
}

fn fast_fibonacci(nth: usize) -> u64 {
  let mut a = 0;
  let mut b = 1;
  let mut c = 0;

  for _ in 1..nth {
    c = a + b;
    a=b;
    b=c;
  }
  c
}

#[cfg(test)] /* Generally used for conditional compilation and not just limited to test code */
mod tests {
    use crate::sum;

  
  fn sum_inputs_outputs() -> Vec<((i8,i8), i8)> {
    vec![((1,1), 2), ((0,0),0), ((2,-2),0)]
  }

  #[test]
  // #[should_panic]
  fn test_sums() {
    for(input, output) in sum_inputs_outputs() {
      assert_eq!(crate::sum(input.0, input.1), output);
    }
  }

  // #[test]
  // // #[ignore]
  // pub fn test_silly_loop() {
  //   silly_loop();
  // }

  #[test]
  fn sum_test() {
    assert_eq!(sum(6, 8), 14);
  }
}