// lib.rs
// Date: Sat Nov 14 2026
// Educational and Practice Rust Programming Language Code

// Dependency: Dependency: Depend on `rand` and `chrono` crate

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

use rand::{RngExt, seq::SliceRandom};
use std::{
    cmp::Ordering,
    fmt::{Debug, Display},
    io::Write,
};

// -----------------------------------------------------------

#[allow(dead_code)]
fn logger(msg: &str) {
    let now = chrono::offset::Local::now();
    println!(" >>> Logger: {now:?} {msg} ");
}

// -----------------------------------------------------------

#[derive(Debug)]
pub enum Validation {
    ZeroFirst,
    Repetitive,
    MoreThanFive,
    LessThanFive,
    Valid,
}

// -----------------------------------------------------------

#[allow(dead_code)]
#[derive(Debug)]
enum Exist {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
}

// -----------------------------------------------------------

#[allow(dead_code)]
#[derive(Debug)]
enum InPlace {
    Zero(Exist),
    One(Exist),
    Two(Exist),
    Three(Exist),
    Four(Exist),
    Five(Exist),
}

// -----------------------------------------------------------

#[must_use]
pub fn secret_number() -> [u32; 5] {
    let mut rng = rand::rng();
    let first: u32 = rng.random_range(1..=9);
    let mut rest: Vec<u32> = (0..=9).filter(|d| *d != first).collect();
    let (pick, _) = rest.partial_shuffle(&mut rng, 4);
    let mut secret: [u32; 5] = [first, 0, 0, 0, 0];
    secret[1..].copy_from_slice(pick);
    secret
}

// -----------------------------------------------------------

#[must_use]
pub fn validate(input_number: &[u32; 5]) -> Validation {
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

// -----------------------------------------------------------

#[must_use]
fn convert_number_to_array(mut number: u32, length: u32) -> [u32; 5] {
    let mut array_number: [u32; 5] = [0_u32; 5];

    for (counter, item) in (1..=length).rev().enumerate() {
        let z: u32 = 10_u32.pow(item - 1);
        let unit = number / z;
        if let Some(n) = array_number.get_mut(counter) {
            *n = unit;
        }
        number -= unit * z;
    }

    array_number
}

// -----------------------------------------------------------

/// # Errors
fn read_input_str_number(time: usize) -> Result<String, Box<dyn std::error::Error>> {
    print!(" => {time} - input five digit number: ");
    std::io::stdout().flush()?;
    let mut input_data = String::new();
    std::io::stdin().read_line(&mut input_data)?;
    input_data = input_data.trim().to_string();
    if input_data.len() != 5 {
        return Err(Box::new(LengthError));
    }
    Ok(input_data)
}

// -----------------------------------------------------------

/// # Errors
pub fn obtain_number(time: usize) -> Result<[u32; 5], Box<dyn std::error::Error>> {
    let input_data = read_input_str_number(time)?;
    let length = u32::try_from(input_data.len())?;
    let number = input_data.parse::<u32>()?;
    let array_number = convert_number_to_array(number, length);
    Ok(array_number)
}

// -----------------------------------------------------------

#[must_use]
pub fn check(secret_number: &[u32; 5], input_number: &[u32; 5]) -> (u32, u32) {
    let mut in_place: u32 = 0;
    let mut exist: u32 = 0;

    for index in 0..5 {
        if let Some(sec_num) = secret_number.get(index)
            && let Some(inp_num) = input_number.get(index)
            && sec_num == inp_num
        {
            in_place += 1;
        }
    }

    for item_inp in input_number {
        for item_sec in secret_number {
            if item_inp == item_sec {
                exist += 1;
            }
        }
    }

    (in_place, exist)
}

// -----------------------------------------------------------

pub struct Counter {
    counter: u32,
}

impl Counter {
    #[must_use]
    pub const fn new(start: u32) -> Self {
        Self { counter: start }
    }

    #[must_use]
    pub const fn status(&self) -> u32 {
        self.counter
    }

    pub const fn plus(&mut self) -> u32 {
        self.counter += 1;
        self.counter
    }

    pub const fn minus(&mut self) -> u32 {
        self.counter -= 1;
        self.counter
    }

    #[must_use]
    pub const fn check(&mut self, with: u32) -> bool {
        self.counter == with
    }

    pub const fn reset(&mut self) {
        self.counter = 0;
    }

    pub const fn reset_by(&mut self, by: u32) {
        self.counter = by;
    }
}

impl Display for Counter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.counter)
    }
}

impl Debug for Counter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Counter")
            .field("counter", &self.counter)
            .finish()
    }
}

#[derive(Debug)]
pub enum GameResult {
    Loss,
    Win,
}

// -----------------------------------------------------------

struct LengthError;

impl std::error::Error for LengthError {}

impl Debug for LengthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Length error")
    }
}

impl Display for LengthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Length error")
    }
}
