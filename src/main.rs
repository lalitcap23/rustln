// fn main() {
//     let x = 5;
//     println!("Value of x: {}", x);

//     let s1 = String::from("Hello, Rust!");
//     let s2 = s1;
//     println!("s2: {}", s2);

//     let s3 = String::from("Ownership to function");
//     take_ownership(s3);

//     let s4 = String::from("Returned from function");
//     let returned_string = give_ownership(s4);
//     println!("Returned string: {}", returned_string);

//     let s5 = String::from("Borrowed string");
//     borrow_string(&s5);
//     println!("s5 after borrowing: {}", s5);

//     let mut s6 = String::from("Mutable borrowing");
//     mutable_borrow(&mut s6);
//     println!("s6 after mutable borrow: {}", s6);
// }

// fn take_ownership(s: String) {
//     println!("Taking ownership of: {}", s);
// }

// fn give_ownership(s: String) -> String {
//     println!("Giving ownership of: {}", s);
//     s
// }

// fn borrow_string(s: &String) {
//     println!("Borrowed string: {}", s);
// }

// fn mutable_borrow(s: &mut String) {
//     s.push_str(" - Modified!");
// }

//Enum

enum direction{
    east,
    west,
    north ,
    south
}
fn print_direction(direction: Direction) {
    match direction {
        Direction::North => println!("Heading North!"),
        Direction::South => println!("Heading South!"),
        Direction::East => println!("Heading East!"),
    }
}

fn main() {
    let dir = Direction::North;
    print_direction(dir);
}
