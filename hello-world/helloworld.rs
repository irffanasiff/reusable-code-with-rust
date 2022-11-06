use std::collections::LinkedList;
use hello::say_hello;

fn main() {
    let a = 1;
    let b = 2;
    println!("{} + {} = {}",a, b, a+b );// ! in prinln means it is a macro and this means the compiler actually does some work without compiling to generate some more amount of code so that it can figure out the type of value it is meant to format into the string at compile time without being told

    // type casting
    let c: i64 = 2;
    add(a, c as i32);

    let mut ll = LinkedList::new();
    ll.push_back(1);
    ll.push_back(2);
    ll.push_back(3);

    for item in ll {
        println!("{}", item);
    }

    let mut vec = Vec::new();
    vec.push("x");
    vec.push("y");
    vec.push("z");

    for item in vec {
        println!("{}", item);
    }

    say_hello();


}

fn add (a: i32, b: i32) -> i32 { // a is a binding of i32 integer and same for b
    a+b
}