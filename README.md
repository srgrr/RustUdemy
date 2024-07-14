# RustUdemy

This repo contains all the activities, exercises, notes, programs and scripts I wrote while learning Rust.

I followed the following course:

Ultimate Rust Crash Course

Through IBM's "IBM learning" platform, although the course itself is hosted, and most likely available, in vanilla Udemy too.

# Safety, Concurrency and Speed

The course presents a "three-options-pick-two" kind of triangle.

Python is Safe and Concurrent, but does not offer Speed
C/C++ offers Speed and Concurrency, but it is not Safe (due to wild memory management basically)

Rust intends to cover these three vertices at once.

# Repo with exercises

This course includes a repo with exercises. I've downloaded it and added it as a submodule here. The repo is named ultimate_rust_crash_course.git

`brew install rust`

This command also installs Cargo, a package manager. I'll give more details in further sections I guess.

# Hello folder

`hello` is the first Rust module I've ever created. You can find it in the very root of this repo.

It was created by writing the following command

`cargo new hello`

And it creates the following folder structure

```
hello
├── Cargo.toml
└── src
    └── main.rs
```

`Cargo.toml` (toml stands for Tom's Obvious Minimalistic Language) is an INI-like file whose role reminds me of that of a `pom.xml` file. It contains basic information about the project and it can also list dependencies (empty for now).

The main file contains a main function printing "Hello, World!".

Cargo can also compile and run Rust programs. For instance, running this command inside the `hello` folder

```
cargo run
```

Compiles (if needed) the program AND runs it. The tree now looks as follows:

```
.
├── Cargo.lock
├── Cargo.toml
├── src
│   └── main.rs
└── target
    ├── CACHEDIR.TAG
    └── debug
        ├── build
        ├── deps
        │   ├── hello-760989959768bd3c
        │   ├── hello-760989959768bd3c.15wrdtzekahee0y4.rcgu.o
        │   ├── hello-760989959768bd3c.1srq9xknetmxsoge.rcgu.o
        │   ├── hello-760989959768bd3c.2oxyq1qwb93fcvcq.rcgu.o
        │   ├── hello-760989959768bd3c.4c223uh7m9k38od0.rcgu.o
        │   ├── hello-760989959768bd3c.4ggg6grakxwhl4d7.rcgu.o
        │   ├── hello-760989959768bd3c.4k44nddejln98bws.rcgu.o
        │   └── hello-760989959768bd3c.d
        ├── examples
        ├── hello
        ├── hello.d
        └── incremental
            └── hello-1xdnuyrxy05td
                ├── s-gxtg6hxvbg-q8l7lm-v4gpdrzi9zzbjoqyxb6pq6lf
                │   ├── 15wrdtzekahee0y4.o
                │   ├── 1srq9xknetmxsoge.o
                │   ├── 2oxyq1qwb93fcvcq.o
                │   ├── 4c223uh7m9k38od0.o
                │   ├── 4ggg6grakxwhl4d7.o
                │   ├── 4k44nddejln98bws.o
                │   ├── dep-graph.bin
                │   ├── query-cache.bin
                │   └── work-products.bin
                └── s-gxtg6hxvbg-q8l7lm.lock
```

Check my `gitignore` file for regexs (asked chatGPT for it lol)

Cargo builds stuff with debug symbols by default. `cargo run --release` gets rid of such symbols.

# Variables

Find my experiments regarding variables in the `variables` folder, in the very root of this repo.

The very first concepts introduced by this course can be summarized with this piece of code

```
fn main() {
  // Rust will figure out that this variable is an integer
  let some_integer = 4;
  // Although the language will still be capable of figuring this one out, too,
  // I explicitly specified that this is supposed to be an integer
  let another_integer: i32 = 4;
  // The two previous variables are IMMUTABLE, uncommenting this line will cause
  // the program to crash
  // some_integer = 7;

  // However, a mutable variable won't cause us any trouble
  let mut mutable_integer = 4;
  mutable_integer = 89;

  // Variables can also be declared as tuples following a python-like LHS-RHS matching
  let (first_integer, second_integer) = (1, 2);

  // We can also declare constants...
  // We MUST specify their type in the code
  const BIGASS_CONSTANT: i32 = 1237;

  // Let's add a print here so the compiler won't throw us ugly warnings
  // (or not too many at least)
  let total = some_integer + another_integer + mutable_integer + first_integer + second_integer + BIGASS_CONSTANT;

  println!("{}", total);
}
```

# Scope
Like in other languages, Rust does have the concept of scope.

Like in C/C++, scopes are defined by code blocks e.g.

```
let x = 5;
{
  let x = 99;
  let y = 100;
  // x = 5 is shadowed by x = 99, and y is only visible within the block
  // this print statement would make the program to not compile if moved
  // outside the block
  println!("{} {}", x, y);
}
```

# Exercise A
The statement of this exercise is in the `README.md` file in the corresponding folder of the course's repo.

I'm naming this folder `exercise_a` instead of `variables` because I already used that name for a miniproject whose intention was to toy around with variables.

# Functions
Functions are defined using the `fn` keyword.

Functions can be declared in any order (unlike in C).

Type annotations for parameters and return type are required.

```
fn f(x: t1, y: t2) -> t3
```

Varargs are not allowed, nor variable type params.

NoneType or "void" is implemented by just not annotating any return type in the function's signature.

# Exercise B

This exercise can be found in the `b_functions` folder in the root of this repo.

The exercise itself is not very complex and can be done by learning the contents in this course. No SO/ChatGPT queries required.

It also mentions the `cargo clippy` feature, which throws a bunch of warnings and suggestions regarding your code.

# A deeper look into the module system

As we have seen before, `cargo new module_name` creates a module `module_name`.

```
module_name
├── Cargo.toml
└── src
    └── main.rs
```

`Cargo.toml` is the kind-of Rust's `pom.xml`. `main.rs` is where the code goes and where the entry point of the module is.

A `lib.rs` file can also be added

```
module_name
├── Cargo.toml
└── src
    ├── lib.rs
    └── main.rs
```

There we can define functions e.g.

```
pub fn compute_area(w: i32, h: i32) -> i32 {
  w * h
}
```

And use it in our `main.rs` file by using the scope operator `::`

```
fn main() {
    let total_area: i32 = module_name::compute_area(2, 2);
    println!("Total area of a 2x2 square is {}", total_area);
}
```

# Standard library and community libraries

Rust, like c++, contains an `std` module that can be imported by default (i.e. without having to explicitly add the dependency anywhere) e.g.

```
use std::collections::HashMap;
```

And, like Python's `pypi`, theres `crates.io` where community libraries are stored. Those libraries are called `crates` and can be retrieved via `cargo`.

For example, let's say we want to import the `rand` library (which contains various RNG related functionalities). It's enough to add a section in our `Cargo.toml` as follows

```
[dependencies]
rand = "0.6.5"
```

# Scalar Types

Like in C++, Rust has unsigned and signed integers. Types are specified as `(u|i)-(N|size)`. u means unsigned, i means signed (actually integer I guess). N can be 8, 16, 32, 64, 128. Supported types vary accordingly to the target architecture. Size is the size of a pointer in the target system/architecture.

Floats are defined with the f prefix and can be 32 o 64 bits. They're declared and operated like we do in C++, types can be enforced as follows

```
let a = 3.14f32;
let b = 3.14f64;
```

Booleans are `bool` and can be `true` or `false`.

There's also `char`. `char` are not ASCII but unicode, meaning that chars a 4-byte, unlike in C++.

Rust strings are UTF-8 and chars are not, so it's questionable how useful those actually are.

# Tuples and Arrays

## Tuples
Tuples are defined with parentheses

```
let info = (1, 3.3, 999);
```

Fields have no name so they're accessed via 0-based indexing

```
let jets = info.0;
```

Tuples have a maximum arity of 12. This seems to be an arbitrary limited implemented by the developers as tuples end up being "hardcoded" in the resulting binary.

Maybe the course is outdated though, I wrote this code

```
fn main() {
  let wrong = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20);
  println!("{}", wrong.18);
}
```

And could compile and run it with no problem whatsoever.

The module can be found in `tuple_limit` in the root of this repo.

## Arrays
Arrays can be defined by using square brackets in two different ways.

```
let arr = [1, 2, 3];
```

```
let arr = [initial_value; size];
```

Or with both

```
let arr: [type; size] = [...];
```

Arrays have its size limited to 32. They're optimized to be stored in the current function's stack and I guess this limitation comes from there too.

# Exercise C

This exercise, named `c_simple_types` in the root of this repo, is a simple exercise about types. It's easy to understand and follow.

# Control flow

## if-else

if-else statements are pretty much like in C++ or in Java except for the fact that here they ALWAYS return something

```
val = if num == 5 {
  "five"
} else if num == 4 {
  "four"
} else {
  "other"
};
```

The semicolon is only necessary when we're using the return value for something specific, otherwise Rust will allow us to write it as if it was a code block.

## Unconditional Loops

They're kind of self explanatory. They work as `while(1)` loops in C

```
'loop_label: loop {
  loop {
    break 'loop_label;
  }
}
```

They can also have labels. `break` and `continue` statements can include a label so we know which loop we're breaking from. In the example above, the `break` statement is breaking both loops.

## While loop

```
while cond {
  stuff();
}
```

## Foreach loop

```
for num in [1, 2, 3].iter() {
  stuff();
}
```

Like in Python, for loops support type unwrapping e.g.

```
let array = [(1, 2), (3, 4)];

for (x, y) in array.iter() {
  stuff(x, y);
}
```

Range-based loops can be written as follows

```
for num in 0..100 {

}
```

You can make the end inclusive by adding an = sign

```
for num in 0..=100 {

}
```

# Strings

Strings are NOT trivial at all in Rust.

The two most common types of strings are borrowed string slices and Strings (with capital S).

Borrowed string slices cannot be modified whereas Strings do not impose such limitation.

A borrowed string slice is a pointer+len array. A string is a pointer+len+capacity array.

String literals such as

```
let msg = "Hello";
```

Are string slices. Explicit conversion is required if what we want are strings

```
let msg = "Hello".to_string();
let msg2  = String::from("Hello");
```

Strings are UTF8. This means that byte-based access doesnt guarantee we're accessing the right character. Using `nth` is encouraged.

# Exercise D
Again, exercise D was super trivial and easy to follow. Find it `d_control_flow_strings` (no exercise about strings whatsoever lol).

# Ownership

Now here's when the tricky part begins. Rust's memory mgmt is quite unique. It's centered about what they call ownership.

Ownership is who owns a variable and Rust is crystal clear with this: there's only one valid owner at any time for a given value.

That is, the following code

```
let s1 = String::from("abc");
let s2 = s1;
```

Invalidates s1 and treats it as uninitialized garbage when the contents of s1 are moved to s2.

This principle allows us to have a clear, unambiguous notion of who owns what and where the changes on a variable might come from, as opposed to free memory access and pointer manipulation in C or the reference system from Python and Java.

# References and Borrowing

Since variables can have only one owner at any given time, we might need something else to work with in order for this language to be viable. This is what references are for.

References come in two kinds, mutable and immutable

```
x: &mut i32 = y; // mutable reference
z: &mut i32 = y; // immutable reference
```

There can only be *one* mutable reference pointing to a given object at any time, and infinitely many immutable references.

The reasons behind this are obvious: multiple mutable references pretty much ruin the concept of ownership and leave us in kind of the same spot we were in with C++, Java and Python.

References can always be manually dereferenced, although there's some exceptions

```
fn do_stuff(s: &mut String) {
  s.insert_str(0, "Hi, "); // No deref needed, handled by the language under the hood
  *s = String::from("Replacement"); // Deref is needed here
}
```

# Exercise E

Exercise E can be found in `e_ownership_references` in the root of this directory. It consists of a bunch of exercises regarding references, borrowing and manipulation of the contents of variables. No SO/prompting was required in order for us to complete this.

# Structs

They are the "classes" of Rust. Pretty much like in C++, they can have attributes and methods:

```
struct RedFox {
  enemy: bool,
  life: u8,
}

impl RedFox {
  fn new() -> Self {
    Self {
      enemy: true,
      life: 70,
    }
  }
  fn move(self) { ... }
  fn borrow(&self) { ... }
  fn mut_borrow(&mut self) { ... }
}
```

As our intuition tells us, the `new` "constructor" method can be called via scoping the struct first

```
let fox = RedFox::new();
```

And the move method with the class dot operator

```
fox.move();
```

Note that the `self` parameter needs to be specified (pretty much like in Python).

# Traits

Rust does not have inheritance. The most similar thing this language has are Traits.

A trait is nothing but a "feature" that, by including it on your struct signatures you guarantee that any instance of this particular object will have certain methods and behaviors e.g.

```
struct RedFox {
  enemy: bool,
  life: u32,
}

trait Noisy {
  fn get_noise(&self) -> &str;
}

impl Noisy for RedFox {
  fn get_noise(&self) -> &str { "Meow?" }
}
```

So you can create methods whose only assumption about a given input parameter is that they have such trait:

```
fn print_noise<T: Noisy>(item: T) {
  println!("{}", item.get_noise());
}
```
