use crate::{Counter, GameResult, Validation};
use crate::{check, obtain_number, secret_number, validate};

pub fn fire() {
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
            println!(" --->>> Invalid number. Try again ...");
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
}
