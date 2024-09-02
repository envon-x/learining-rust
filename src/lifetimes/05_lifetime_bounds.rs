// 05_lifetime_bounds.rs
enum Level {
    Error
}

struct Logger<'a>(&'a str, Level);

fn configure_logger<T>(_t: T) where T: Send + 'static {
    // configure the logger here
}

fn main() {
    let name = "Global";
    let log1 = Logger(name, Level::Error);
    configure_logger(log1);
}

/*
warning: field `0` is never read
 --> ./05_lifetime_bounds.rs:6:19     
  |                                         
6 | struct Logger<'a>(&'a str, Level);                                                                       
  |        ------     ^^^^^^^  
  |        |
  |        field in this struct             
  |                                                                                                          
  = note: `#[warn(dead_code)]` on by default
help: consider changing the field to be of unit type to suppress this warning while preserving the field numbering, or remove the field       
  |                                                                                                          
6 | struct Logger<'a>((), Level);
  |                   ~~  

warning: 1 warning emitted   

*/
