// ownership_functions.rs

fn take_the_n(n: u8) { 
  println!("Dentro de n: {:?}", n); 
}

fn take_the_s(s: String) -> String {
    println!("inside function {}", s);
    s+" lo cual es una fuente de los deseos."
}

fn main() { 
    let n = 5; 
    let s = String::from("First string"); 

    take_the_n(n); 
    let s = take_the_s(s); 

    println!("n is {}", n); 
    println!("s is {}", s); 
} 