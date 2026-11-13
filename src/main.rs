// main.rs
// Date: Fri Nov 13 2026
// Educational and Practice Rust Programming Language Code

// Project: Crate Guess Game
// Goal: ...
// Dependency: Depend on `rand` crate

// Kernel Version: 7.3.3-300.fc45.x86_64

// rustc 1.102.0-nightly (3ef0c22cd 2026-11-11)
// binary: rustc
// commit-hash: 3ef0c22cdb86ac42dc6b8f0f1ec36857daa6f2a1
// commit-date: 2026-11-11
// host: x86_64-unknown-linux-gnu
// release: 1.102.0-nightly
// LLVM version: 23.1.5

// cargo 1.102.0-nightly (c07ac2c4d 2026-11-06)
// release: 1.102.0-nightly
// commit-hash: c07ac2c4d12237effc4b94f4543769b421790045
// commit-date: 2026-11-06
// host: x86_64-unknown-linux-gnu
// libgit2: 1.9.6 (sys:0.21.0 vendored)
// libcurl: 8.21.0-DEV (sys:0.4.90+curl-8.21.0 vendored ssl:OpenSSL/3.6.3)
// ssl: OpenSSL 3.6.3 9 Jun 2026
// os: Fedora 45.0.0 [64-bit]

use std::cmp::Ordering;

use rand::{RngExt, seq::SliceRandom};

fn main() {
    println!("\n");

    for index in 0..1000000 {
        let array_number = secret_number();
        let result = validate(&array_number);
        println!("{index} {result:?}");
        match result {
            Validation::LessThanFive => break,
            Validation::MoreThanFive => break,
            Validation::Repetitive => break,
            Validation::ZeroFirst => break,
            Validation::Valid => (),
        }
    }

    println!("\nThe End ...\n");
}

#[allow(dead_code)]
#[derive(Debug)]
enum Validation {
    ZeroFirst,
    Repetitive,
    MoreThanFive,
    LessThanFive,
    Valid,
}

#[allow(dead_code)]
fn secret_number() -> [u32; 5] {
    let mut rng = rand::rng();
    let first: u32 = rng.random_range(1..=9);
    let mut rest: Vec<u32> = (0..=9).filter(|d| *d != first).collect();
    let (pick, _) = rest.partial_shuffle(&mut rng, 4);
    let mut secret: [u32; 5] = [first, 0, 0, 0, 0];
    secret[1..].copy_from_slice(pick);
    secret
}

#[allow(dead_code)]
fn validate(input_number: &[u32; 5]) -> Validation {
    let Some(1..=9) = input_number.first() else {
        return Validation::ZeroFirst;
    };

    match input_number.len().cmp(&5) {
        Ordering::Greater => return Validation::MoreThanFive,
        Ordering::Less => return Validation::LessThanFive,
        Ordering::Equal => (),
    }

    #[allow(clippy::indexing_slicing)]
    for index_outer in 0..input_number.len() {
        for index_inner in 0..input_number.len() {
            if index_inner == index_outer {
                continue;
            }
            if input_number[index_outer] == input_number[index_inner] {
                return Validation::Repetitive;
            }
        }
    }

    Validation::Valid
}
