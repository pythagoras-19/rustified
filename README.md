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