# Cormen Algorithms in Rust
[![Travis Build Status](https://travis-ci.org/sjsyrek/Cormen_Rust.svg?branch=master)](https://travis-ci.org/sjsyrek/Cormen_Rust)
[![Apache 2.0 licensed](https://img.shields.io/badge/license-Apache2.0-blue.svg)](./LICENSE.txt)

This project contains Rust implementations of some of the algorithms from [_Introduction to Algorithms_](http://www.amazon.com/Introduction-Algorithms-3rd-Thomas-Cormen/dp/0262033844/ref=sr_1_fkmr0_1?ie=UTF8&qid=1453748746&sr=8-1-fkmr0&keywords=cormel+introduction+to+algorithms) by Thomas Cormen, et al.

I am working on these in order to learn the [Rust](https://www.rust-lang.org/index.html) programming language. I have found that they mostly work verbatim but do require an understanding of the type system and of the concepts of [ownership](http://doc.rust-lang.org/1.6.0/book/ownership.html) and [borrowing](http://doc.rust-lang.org/1.6.0/book/references-and-borrowing.html). They may therefore be useful for other Rust novices familiar with Cormen's book.

I will do my best to document these functions in accordance with `rustdoc` conventions. Enjoy.

## Installation

- Make sure you have [Cargo](http://doc.crates.io/index.html), the Rust package manager, installed on your machine.
- Clone repository locally.
- In your project directory, run `cargo build` or `cargo build --release` to enable optimizations.
- Run `cargo test` to run the comment-embedded test code and compile examples into executables.
- Run `cargo doc` to generate HTML documentation

## Usage

Put `use cormen_rust::*;` at the top of your source files to bring the entire library into scope or `use cormen_rust` to maintain it as a separate namespace. 