use std::fmt;

struct MyStruct(i32);

impl fmt::Display for MyStruct {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MyStruct contains: {}", self.0)
    }
}

fn main() {
    let data = MyStruct(42);
    println!("{}", data); // Output: MyStruct contains: 42
}
