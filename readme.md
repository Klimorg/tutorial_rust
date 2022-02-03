# Rust + Cargo tutorial

**Main resource** : https://doc.rust-lang.org/book/title-page.html


Let’s recap what we’ve learned so far about Cargo:

* We can build a project using `cargo build`.
* We can build and run a project in one step using `cargo run`.
* We can build a project without producing a binary to check for errors using `cargo check`.
* Instead of saving the result of the build in the same directory as our code, Cargo stores it in the target/debug directory.

## Building for Release

When your project is finally ready for release, you can use `cargo build --release` to compile it with optimizations. This command will create an executable in **target/release instead of target/debug**.

The optimizations make your Rust code run faster, but turning them on lengthens the time it takes for your program to compile. This is why there are two different profiles: one for development, when you want to rebuild quickly and often, and another for building the final program you’ll give to a user that won’t be rebuilt repeatedly and that will run as fast as possible. If you’re benchmarking your code’s running time, be sure to run `cargo build --release` and benchmark with the executable in target/release.

## Shadowing, variables and mutability

You can do variable shadowing with limited scope.

```rust
fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);
}
```

This program first binds `x` to a value of $5$. Then it shadows `x` by repeating let `x =`, taking the original value and adding $1$ so the value of `x` is then $6$.

Then, within an inner scope, the third let statement also shadows `x`, multiplying the previous value by $2$ to give `x` a value of $12$.

When that scope is over, the inner shadowing ends and `x` returns to being $6$.

### Difference between `mut` and `let mut`

1. Shadowing is different from marking a variable as `mut`, because **we’ll get a compile-time error if we accidentally try to reassign to this variable without using the `let` keyword**. By using let, we can perform a few transformations on a value but have the variable be immutable after those transformations have been completed.

Eg :

```rust
fn main() {
    let x = 5;

    x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);
}
```
will gives the error

```shell
(base) vorph@Laptop3080:~/work/rust/concepts$ cargo check
    Checking concepts v0.1.0 (/home/vorph/work/rust/concepts)
error[E0384]: cannot assign twice to immutable variable `x`
 --> src/main.rs:4:5
  |
2 |     let x = 5;
  |         -
  |         |
  |         first assignment to `x`
  |         help: consider making this binding mutable: `mut x`
3 |
4 |     x = x + 1;
  |     ^^^^^^^^^ cannot assign twice to immutable variable

For more information about this error, try `rustc --explain E0384`.
error: could not compile `concepts` due to previous error
```

Meaning that **shadowing a variable doesn't have anything to do with it to be immutable**, in the following script

```rust
fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);
}
```

**you are indeed shadowing immutable variables, and shadowing an immutable variable still gives an immutable variable.**

The following script isn't shadowing `x`, it's just using its mutable trait.
```rust
fn main() {
    let mut x = 5;

    x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);
}
```

2. The other difference between `mut` and shadowing is that because we’re effectively creating a new variable when we use the `let` keyword again, we can change the type of the value but reuse the same name.

**Meaning that even if your variable is mutable, you can't change its type.But you can change the type of the variable by shadowing it.** The following script

```rust
fn main() {
    let mut x = 5;

    x = "hello";

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);
}
```

will gives the following error.

```shell
(base) vorph@Laptop3080:~/work/rust/concepts$ cargo check
    Checking concepts v0.1.0 (/home/vorph/work/rust/concepts)
error[E0308]: mismatched types
 --> src/main.rs:4:9
  |
2 |     let mut x = 5;
  |                 - expected due to this value
3 |
4 |     x = "hello";
  |         ^^^^^^^ expected integer, found `&str`

For more information about this error, try `rustc --explain E0308`.
error: could not compile `concepts` due to previous error
```

## Data types

Keep in mind that Rust is a *statically typed* language, which means that **it must know the types of all variables at compile time**. The compiler can usually infer what type we want to use based on the value and how we use it.

### Numeric operations

Be careful with the type.

```rust
fn main() {
    let guess: u32 = "42".parse().expect("Not a number!");

    let div1 = 2 / 3;

    let div2 = 2.0 / 3.0;

    println!("The value of x is: {}, {}, {}", guess, div1, div2);
}
```

gives

```shell
(base) vorph@Laptop3080:~/work/rust/concepts$ cargo run
   Compiling concepts v0.1.0 (/home/vorph/work/rust/concepts)
    Finished dev [unoptimized + debuginfo] target(s) in 0.18s
     Running `target/debug/concepts`
The value of x is: 42, 0, 0.6666666666666666
```

because `2 / 3` is a division with two integers. Integer division rounds down to the nearest integer. While `2.0 / 3.0` gives the result you would expect because they are typed as float here.

### Tuples

https://doc.rust-lang.org/book/ch03-02-data-types.html#the-tuple-type

### Arrays

Another way to have a collection of multiple values is with an array. **Unlike a tuple, every element of an array must have the same type. Unlike arrays in some other languages, arrays in Rust have a fixed length.**

**SAME TYPE**.

**FIXED LENGTH**.

A **vector** is a similar collection type provided by the standard library that is allowed to grow or shrink in size.

**A list as we understand it in Python = a vector in Rust.**

Defining and typing arrays in Rust.

```rust
let a: [i32; 5] = [1, 2, 3, 4, 5];
```

```rust
use std::cmp::Ordering;
use std::io;

fn main() {
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    let len_a = a.len();

    println!("Length of a : {}", len_a);

    loop {
        let mut index = String::new();
        println!("Index you want to know");

        io::stdin()
            .read_line(&mut index)
            .expect("Nothing to read !");

        let index: usize = match index.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match index.cmp(&len_a) {
            Ordering::Less => println!("Here's your number {}", a[index]),
            Ordering::Equal => {
                println!("Too big !");
                break;
            },
            Ordering::Greater => {
                println!("Too big !");
                break;
            }
        }
    }
}
```

## Functions

Rust code uses **snake case as the conventional style** for function and variable names, in which all letters are lowercase and underscores separate words.

In function signatures, you must declare the type of each parameter. This is a deliberate decision in Rust’s design: requiring type annotations in function definitions means the compiler almost never needs you to use them elsewhere in the code to figure out what type you mean.

### Statement and expressions

Function bodies are made up of a series of statements optionally ending in an expression.

So far, the functions we’ve covered haven’t included an ending expression, but you have seen an expression as part of a statement. Because Rust is an expression-based language, this is an important distinction to understand. Other languages don’t have the same distinctions, so let’s look at what statements and expressions are and how their differences affect the bodies of functions.

**Statements are instructions that perform some action and do not return a value.**

**Expressions evaluate to a resulting value. Expressions do not include ending semicolons.**

## Ownership

**Stack** = **LIFO** : The stack stores values in the order it gets them and removes the values in the opposite order. This is referred to as **last in, first out**. Adding data is called *pushing onto the stack*, and removing data is called *popping off the stack*. **All data stored on the stack must have a known, fixed size**. Data with an unknown size at compile time or a size that might change must be stored on the heap instead.

**The heap is less organized**: when you put data on the heap, you request a certain amount of space. *The memory allocator finds an empty spot in the heap that is big enough, marks it as being in use, and returns a pointer*, which is the address of that location. This process is called allocating on the heap and is sometimes abbreviated as just allocating.

Ownership Rules

First, let’s take a look at the ownership rules. Keep these rules in mind as we work through the examples that illustrate them:

* Each value in Rust has a variable that’s called its owner.
* There can only be one owner at a time.
* When the owner goes out of scope, the value will be dropped.

```rust
fn main() {
    {
        // s is not valid here, it’s not yet declared
        let s = "hello"; // s is valid from this point forward

        // do stuff with s
    } // this scope is now over, and s is no longer valid
    println!("{}", s)
}
```

will gives

```shell
❯ cargo check
    Checking ownership v0.1.0 (/media/vorph/datas/tutorial_rust/ownership)
error[E0425]: cannot find value `s` in this scope
 --> src/main.rs:8:20
  |
8 |     println!("{}", s)
  |                    ^ not found in this scope

For more information about this error, try `rustc --explain E0425`.
error: could not compile `ownership` due to previous error
```
You can create a String from a string literal as a strating point using the from function, like so:

```rust
    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() appends a literal to a String
```

### Ownership and Functions

One main difference with Python is how Rust cleans its variables. In Python, the following code is acceptable.

```python
var1 = 2

var2 = some_function(var1)

print(f"{var1}")
```
While in Rust, doing something similar

```rust
let var1 = 2;

let var2 = some_fn(var1);

println!("{}", var1);
```
will ends with an error, `var1` is not accessible anymore, being consumed by the function `some_fn` made it change of scope, it now belongs to the local scope of the function `some_fn` and can no longer be accessed in the main scope.

We say that `some_fn` takes ownership of the varriable `var1`.

Unless you explicitely return it.

```rust
let var1 = 2;

let (var1, var2) = some_fn(var1);

println!("{}", var1);

fn some_fn(var1:i32)-> (i32,i32) {
    //your code here

    return (var1, var2);
}
```

### References and Borrowing

```rust
fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}
```

The issue with the tuple code above is that we have to return the String to the calling function so we can still use the String after the call to calculate_length, because the String was moved into calculate_length. Instead, we can provide a reference to the String value. A reference is like a pointer in that it’s an address we can follow to access data stored at that address that is owned by some other variable. Unlike a pointer, a reference is guaranteed to point to a valid value of a particular type. Here is how you would define and use a calculate_length function that has a reference to an object as a parameter instead of taking ownership of the value:

```rust
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

First, notice that all the tuple code in the variable declaration and the function return value is gone. Second, note that we pass &s1 into calculate_length and, in its definition, we take &String rather than String. These ampersands represent references, and they allow you to refer to some value without taking ownership of it.

```rust
fn main() {
    let s = String::from("hello");

    change(&s);
}

fn change(some_string: &String) {
    some_string.push_str(", world");
}
```

```shell
$ cargo run
   Compiling ownership v0.1.0 (file:///projects/ownership)
error[E0596]: cannot borrow `*some_string` as mutable, as it is behind a `&` reference
 --> src/main.rs:8:5
  |
7 | fn change(some_string: &String) {
  |                        ------- help: consider changing this to be a mutable reference: `&mut String`
8 |     some_string.push_str(", world");
  |     ^^^^^^^^^^^ `some_string` is a `&` reference, so the data it refers to cannot be borrowed as mutable

For more information about this error, try `rustc --explain E0596`.
error: could not compile `ownership` due to previous error
```

Just as variables are immutable by default, so are references. We’re not allowed to modify something we have a reference to.

```rust
fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

Mutable references have one big restriction: you can have only one mutable reference to a particular piece of data at a time.

**The Rules of References**

Let’s recap what we’ve discussed about references:

* At any given time, you can have either one mutable reference or any number of immutable references.
* References must always be valid.
