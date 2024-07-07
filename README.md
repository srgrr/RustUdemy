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
