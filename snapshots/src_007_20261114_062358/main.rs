// main.rs
// Date: Sat Nov 14 2026
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

use guess_game::{Counter, GameResult, Validation};
use guess_game::{check, obtain_number, secret_number, validate};

fn main() /*-> Result<(), Box<dyn std::error::Error>>*/
{
    println!("\n");

    let mut input_number: [u32; 5];
    let mut game_resultant = GameResult::Loss;
    let mut step = Counter::new(0);
    let secret_number = secret_number();

    // ---------------------------------------

    loop {
        step.plus();
        if step.check(11) {
            break;
        }
        input_number = loop {
            let input_number_result = obtain_number(step.status() as usize);
            if let Ok(inp) = input_number_result {
                break inp;
            }
            println!(" --->>> Invalid number. Try agian ...");
        };

        // ---------------------------------------

        let result = validate(&input_number);
        match result {
            Validation::LessThanFive => {
                println!(" --->>> Invalid Number: Less than five digit ...");
                step.minus();
                continue;
            }
            Validation::MoreThanFive => {
                println!(" --->>> Invalid Number: More than five digit ...");
                step.minus();
                continue;
            }
            Validation::ZeroFirst => {
                println!(" --->>> Invalid Number: First number is zero ...");
                step.minus();
                continue;
            }
            Validation::Repetitive => {
                println!(" --->>> Invalid Number: Repetitive digits ...");
                step.minus();
                continue;
            }
            Validation::Valid => (),
        }

        // ---------------------------------------

        let check = check(&secret_number, &input_number);
        println!(" -> {step} - In Place:{}, Exist:{}\n", check.0, check.1);
        if check.0 == 5 && check.1 == 5 {
            println!("Congratulation, You Win ...");
            game_resultant = GameResult::Win;
            break;
        }
    }

    if matches!(game_resultant, GameResult::Loss) {
        println!(" > You Loss the game...");
        println!(" > That’s fine; you can try again whenever you like ...");
        println!(" > The number was: {secret_number:?}");
    }

    println!("\n The End ...\n");
    // Ok(())
}
