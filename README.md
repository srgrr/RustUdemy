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
