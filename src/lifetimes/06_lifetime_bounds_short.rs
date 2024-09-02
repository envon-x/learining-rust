// lifetime_bounds_short.rs

enum Level {
    Error
}

struct Logger<'a>(&'a str, Level);

fn configure_logger<T>(_t: T) where T: Send + 'static {
    // configure the logger here
}

fn main() {
    let other = String::from("Local");
    let log2 = Logger(&other, Level::Error);
    configure_logger(&log2);
}
/*

error[E0597]: `other` does not live long enough
  --> ./06_lifetime_bounds_short.rs:15:23
   |
14 |     let other = String::from("Local");
   |         ----- binding `other` declared here
15 |     let log2 = Logger(&other, Level::Error);
   |                       ^^^^^^ borrowed value does not live long enough
16 |     configure_logger(&log2);
   |     ----------------------- argument requires that `other` is borrowed for `'static`
17 | }
   | - `other` dropped here while still borrowed

error[E0597]: `log2` does not live long enough
  --> ./06_lifetime_bounds_short.rs:16:22
   |
15 |     let log2 = Logger(&other, Level::Error);
   |         ---- binding `log2` declared here
16 |     configure_logger(&log2);
   |     -----------------^^^^^-
   |     |                |
   |     |                borrowed value does not live long enough
   |     argument requires that `log2` is borrowed for `'static`
17 | }
   | - `log2` dropped here while still borrowed

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0597`.

*/