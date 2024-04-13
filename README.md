# rustified
# Foundational Rust Principles

## Ownership and Borrowing

Rust uses a unique system of Ownership with rules that the compiler checks at compile time. This system ensures that there are no dangling pointers or memory leaks, and all memory is cleaned up once an object goes out of scope.

```rust
fn main() {
    let s1 = String::from("Hello");
    let s2 = s1; // s1 is no longer valid here
    println!("{}", s2); // Works fine, s2 owns the data
}
```
## Borrowing
- Borrowing is a feature in Rust that allows you to access the data of a value without taking ownership over it. This is useful when you want to use a value but don't want to take ownership, which would involve copying or moving the value.
```rust
fn main() {
    let s = String::from("hello");
    let len = calculate_length(&s);
    println!("The length of '{}' is {}.", s, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```
# Safety
- Rust's commitment to safety eliminates entire classes of bugs, ensuring memory safety without using a garbage collector.
```rust
fn main() {
    let v = vec![1, 2, 3];
    let v_index = 3;
    // The line below would compile fail, preventing access to an invalid array index
    // println!("{}", v[v_index]);
}
```
# Concurrency without Fear
- Rust's ownership and type system ensure that mutable data cannot be simultaneously accessed from different threads, thus preventing data races at compile time.
```rust
use std::thread;

fn main() {
    let mut data = vec![1, 2, 3];

    thread::spawn(move || {
        data[0] += 1;
    }).join().unwrap();

    println!("{:?}", data);
}
```
- Rust has a number of features that help you write safe and efficient multithreaded code. The `std::thread` library provides support for spawning new threads. Rust's type system and ownership rules greatly assist in getting concurrency right.
```rust
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}
```
# Lifetimes
- Lifetimes are a way of ensuring that **all references are valid**. They are annotated with a syntax that looks like this: `<'a>`. Lifetimes are part of type annotations, which let the compiler know how generic lifetime parameters should be substituted.
```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```
# Error Handling
- Rust encourages the use of error handling to deal with exceptional situations. The `Result` and `Option`types are used for functions that can fail, and the `?` operator can be used to propagate errors up the call stack.
```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}
```