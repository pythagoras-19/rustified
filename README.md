# rustified
# Foundational Rust Principles
### Rust solves memory leak problems with ease
- **memory leak**: The situation where a computer program incorrectly manages memory allocations. 
  - In essence, it happens when an **application allocates memory for temporary use and then, due to logical errors**, fails to release it when it's no longer needed. 
- Over time, a memory leak can cause an application to slow down or crash as it consumes more and more of the system's available memory.  
- In languages that do not automatically manage memory, like C and C++, developers are responsible for allocating and deallocating memory. 
- If they forget to deallocate memory that's no longer needed, a memory leak occurs.  
- However, in languages with automatic memory management, such as Java, Python, or Rust, the garbage collector or ownership system (in Rust) is designed to prevent memory leaks by automatically deallocating memory that's no longer in use. 
- In **Rust** specifically, the ownership model ensures that memory is deallocated when there are no more references to it, which helps prevent memory leaks.


- **dangling pointer**:  pointer that doesn't point to a valid object of the appropriate type
    - usually occurs when an object is deleted or deallocated, without modifying the value of the pointer, so it still points to the memory location of the deallocated memory.
    - As the system may have reallocated the memory to another object, using the dangling pointer can lead to _unpredictable results_ or program crashes.
    - This is a common issue in languages that allow direct memory management, such as C and C++.
    - However, in **Rust**, the ownership system prevents this problem at compile time.

## Ownership and Borrowing
- Rust uses a unique system of Ownership with rules that the compiler checks at compile time. This system ensures that there are no dangling pointers or memory leaks, and all memory is cleaned up once an object goes out of scope.

## The Borrow Checker
- Portion of the compiler that enforces the rules of borrowing
  - it checks the code to ensure that references to data obey the following rules
  1. Any borrow **must** last for a scope no greater than that of the owner
  2. You may have one or the other these 2 kinds of borrows, BUT NOT BOTH at the same time
    - One or more references (`&T`) to a resource
    - Exactly one mutable reference (`&mut T`)
- example of correct use of borrowing: 
```rust
fn main() {
    let mut s = String::from("hello");

    change(&mut s);
    println!("{}", s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```
- example of incorrect use of borrowing
```rust
fn main() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s; // 2 mutable references to s in the same scope! ERROR

    println!("{}, {}", r1, r2);
}
```
## Ownership (in more detail)
- Ownership is a key feature of Rust that makes it possible to manage memory safely and efficiently, without needing a garbage collector. 
- In Rust, each value has a variable that's called its owner. There can only be one owner at a time. 
- When the owner goes out of scope, the value will be dropped, and the memory it was using is freed.
- example:
```rust
fn main() {
    let s1 = String::from("Hello"); // s1 owns the data
    let s2 = s1; // s1 is no longer valid here, s2 owns the data; s1 is NO LONGER VALID
    println!("{}", s2); // Works fine, s2 owns the data
}
```
- another example:
```rust
fn s() {
    let s = String::from("hello"); // s is valid from this point forward
    // do stuff with s
} // this scope is now over, and s is no longer valid, so the memory is freed
```
- In this example, s is the owner of the string "hello". When s goes out of scope at the end of the block, the string is automatically deallocated.

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
## No data races 
- The compiler ensures that you don't have multiple threads with mutable access to the same data.

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
- a `'static` is a special lifetime which represents the entire duration of the program
  - most often seen on string literals

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

# Programming Philosophy with Rust
## Focus
- Rust's focus is on speed, safety, and concurrency.
- **Rust's philosophy is about giving the developer low-level control over the system, while providing high-level abstractions and strong safety guarantees**
- It aims to provide the ability to build reliable and efficient software while minimizing common programming errors like:
    - null pointer dereferencing
    - memory leaks
    - data races
- Rust is statically typed and compiled with a focus on safety and performance
- Rust does NOT use classes!
  - Instead, it uses a combination of `structs` and `impl` blocks to create types and define behavior.
- `struct`: custom data type that lets the programer name and package together multiple related values
  - NO INHERITANCE
- example:
```rust
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    fn distance_to_origin(&self) -> f64 {
        let x2 = self.x.pow(2) as f64;
        let y2 = self.y.pow(2) as f64;
        (x2 + y2).sqrt()
    }
}
```
- `new()`: considered the constructor of a new Point object

## Types
- a type defines the kind of data that a value can hold and how the data behaves
- Categories:
1. Scalar types: represent single value; like integer, floating point numbers, booleans, or characters
2. Compound types: These group multiple values into 1 type; like tuples and arrays
3. Custom types: `struct`, `enum`, `union`
4. Function types: Types of functions, defined by their inputs and output types
5. Pointer types: these include references: `&T`, `&mut T`. These also include raw pointers: `*const T`, `*mut T`
6. Trait types: implement a specific **trait**
7. The Unit type: this indicates an absence of a value or state denoted by `()`
8. The Never type: indicates a computation that never returns, denoted by `!`

Example of the types:
```rust
fn main() {
    let logical: bool = true; // Boolean type

    let a_float: f64 = 1.0;  // Floating-point type

    let an_integer   = 5i32; // Integer type

    let default_float   = 3.0; // `f64` (default floating-point type)

    let default_integer = 7;   // `i32` (default integer type)

    let c = 'z'; // Character type

    let z = 'ℤ'; // Unicode scalar value

    let heart_eyed_cat = '😻'; // Unicode scalar value

    let tup: (i32, f64, u8) = (500, 6.4, 1); // Tuple type

    let arr = [1, 2, 3, 4, 5]; // Array type
}

```

## Traits
- What is it: a way to define shared behavior across types
- A trait in Rust defines a set of methods that other types can implement.
- If a type chooses to implement a `trait` it **must** implement all the trait's functions!!
- example:
```rust
trait Speak {
    fn speak(&self);
    
    fn print_one(&self);
}

struct Human;
struct Dog;

impl Speak for Human {
    fn speak(&self) {
        println!("Hello!");
    }

    fn print_one(&self) {
        println!("1");
    }
}

impl Speak for Dog {
    fn speak(&self) {
        println!("Woof!");
    }

    fn print_one(&self) {
        println!("One");
    }
}
```
- in this example: `Speak` is a trait that has a method `speak()`. 
- Both `Human` and `Dog` are structs (custom types) that implement their own versions of `speak()`

### A parallel in Java with interfaces
```java
interface Speak {
    void speak();
}

class Human implements Speak {
    public void speak() {
        System.out.println("Hello!");
    }
}

class Dog implements Speak {
    public void speak() {
        System.out.println("Woof!");
    }
}
```

## Closures
- small anonymous functions like lambdas
```
|<parameter-name>: <type>| -> <return-type> { <body> };
```
```rust
fn main() {
    let f = |x: i32| -> i32 { x * 2 };
    let f = || println!("Hello, world!"); // with no input parameter
}
```
- closures can be passed as parameters
```rust
// A function which takes a function pointer as an argument and calls it with
// the value `5`.
fn apply(f: fn(i32) -> i32) -> i32 {
    // No semicolon, to indicate an implicit return
    f(5)
}

fn main() {
    // Defining the closure
    let f = |x| x * 2;

    println!("{}", apply(f));  // 10
    println!("{}", f(5));      // 10
}
// can be shortened to:
fn apply_by_ref(f: impl Fn(i32) -> i32) -> i32 {
    f(5)
}
```

## Crates
- Primary bulding blocks of Rust. 
- Equivalent to library in JavaScript
- A crate can produce an .exe or a library
- Useful for: managing and organizing code in a modular way
- Why use crates?: They provide a way to group related functionality together
  - makes it easier to share and distribute code
- example:
- dependency in `Cargo.toml`
```toml
[dependencies]
rand = "0.8.4"
```
- then implemented in Rust: 
```rust
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let n: u32 = rng.gen();
    println!("Random number: {}", n);
}
```
- In this example, we're using the `rand::Rng` **trait**, which provides methods for generating random numbers. 
- The `gen` method generates a random number that can be any value that is valid for the type.

## Pointers
| Type                             | Description                                          | Examples                                                         |
|----------------------------------|------------------------------------------------------|------------------------------------------------------------------|
| `&T`<br/>`&mut T`                | References (mutable or immutable)                    | `let x_ref = &x;`<br/>`let x_ref = &mut x;`                      |
| `Option<&T>`<br/>`Option<&mut T>` | Option wrapped reference<br/>Possibly null reference | `None`<br/>`let x_ref = Some(&x)`<br/>`let x_ref = Some(&mut x);` |
| Row 3                            | Row 3                                                | Row 3                                                            |
| Row 4                            | Row 4                                                | Row 4                                                            |
| Row 5                            | Row 5                                                | Row 5                                                            |