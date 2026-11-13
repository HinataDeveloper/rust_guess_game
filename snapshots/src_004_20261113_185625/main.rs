// main.rs
// Date: Fri Nov 13 2026
// Educational and Practice Rust Programming Language Code

// Project: Crate Guess Game
// Goal: ...
// Dependency: Depend on `rand` and `chrono` crate

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
use std::{cmp::Ordering, io::Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n");

    let mut win = false;
    let secret_number = secret_number();

    // ---------------------------------------

    for item in 0..10 {
        let input_number = obtain_number(item + 1)?;

        // ---------------------------------------

        let result = validate(&input_number);
        match result {
            Validation::LessThanFive => println!("Invalid Number: Less than five digit ..."),
            Validation::MoreThanFive => println!("Invalid Number: More than five digit ..."),
            Validation::ZeroFirst => println!("Invalid Number: First number is zero ..."),
            Validation::Repetitive => println!("Invalid Number: Repetitive digits ..."),
            Validation::Valid => (),
        }

        // ---------------------------------------

        let check = check(&secret_number, &input_number);
        println!(" -> {item} - In Place:{}, Exist:{}\n", check.0, check.1);
        if check.0 == 5 && check.1 == 5 {
            println!("Congratulation, You Win ...");
            win = true;
            break;
        }
    }

    if !win {
        println!(
            "You Loss the game...\n
        That’s fine; you can try again whenever you like ..."
        );
    }

    println!("\n The End ...\n");
    Ok(())
}

//-------------------------

#[allow(dead_code)]
fn logger(msg: &str) {
    let now = chrono::offset::Local::now();
    println!(" >>> Logger: {now:?} {msg} ");
}

//-------------------------

#[allow(dead_code)]
#[derive(Debug)]
enum Validation {
    ZeroFirst,
    Repetitive,
    MoreThanFive,
    LessThanFive,
    Valid,
}

//-------------------------

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

//-------------------------
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

//-------------------------

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

//-------------------------

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

//-------------------------

fn convert_number_to_array(mut number: u32, length: u32) -> [u32; 5] {
    let mut array_number: [u32; 5] = [0_u32; 5];

    // let mut counter: usize = 0;

    for (counter, item) in (1..=length).rev().enumerate() {
        let z: u32 = 10_u32.pow(item - 1);
        let unit = number / z;
        if let Some(n) = array_number.get_mut(counter) {
            *n = unit;
        }
        number -= unit * z;
        // counter += 1;
    }

    array_number
}

//-------------------------

fn read_input_str_number(time: usize) -> Result<String, std::io::Error> {
    print!(" => {time} - input five digit number: ");
    std::io::stdout().flush()?;
    let mut input_data = String::new();
    std::io::stdin().read_line(&mut input_data)?;
    Ok(input_data.trim().to_string())
}

//-------------------------

#[allow(dead_code)]
fn obtain_number(time: usize) -> Result<[u32; 5], Box<dyn std::error::Error>> {
    let input_data = read_input_str_number(time)?;
    let length = u32::try_from(input_data.len())?;
    let number = input_data.parse::<u32>()?;
    let array_number = convert_number_to_array(number, length);
    Ok(array_number)
}

fn check(secret_number: &[u32; 5], input_number: &[u32; 5]) -> (u32, u32) {
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
