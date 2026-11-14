use rand::{RngExt, seq::SliceRandom};
use std::{cmp::Ordering, io::Write};

use crate::{LengthError, Validation};



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
