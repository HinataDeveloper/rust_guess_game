# guess_game

A command-line number guessing game written in Rust. The program picks a secret five-digit number, and you try to guess it within ten attempts, using feedback after each guess.

## How It Works

When the program starts, it generates a secret number with these properties:

- exactly **5 digits**
- **does not start with 0**
- **no repeated digits** (every digit is unique)

You then enter guesses one at a time. Your guess must follow the same rules as the secret number:

- exactly 5 digits
- does not start with 0
- no repeated digits

You have a maximum of **10 attempts**. An invalid guess is rejected with a message and does **not** use up an attempt.

## Requirements

- [Rust](https://www.rust-lang.org/tools/install) toolchain (`rustc` and `cargo`), version 1.102 or newer (edition 2024)

### Dependencies

| Crate    | Purpose                                                                                        |
|----------|------------------------------------------------------------------------------------------------|
| `rand`   | Generates the random secret number.                                                            |
| `chrono` | Timestamps for a logger function used only during development and testing, not in normal play. |

## Build

```sh
cargo build --release
```

The executable is created at `target/release/guess-game`.

## Usage

Run the executable:

```sh
./target/release/guess-game
```

Or build and run in one step:

```sh
cargo run --release
```

## Feedback

After each valid guess, the game reports two values:

```
 -> 1 - In Place:1, Exist:3
```

The first number is the attempt number, followed by:

| Value      | Meaning                                                                                      |
|------------|----------------------------------------------------------------------------------------------|
| `In Place` | How many digits of your guess are in the secret number **and** at exactly the same position. |
| `Exist`    | How many digits of your guess appear in the secret number, regardless of position.           |

`Exist` includes the digits that are also in place. Since every digit is unique, `In Place` can never be greater than `Exist`.

### Example

Suppose the secret number is `12345`:

| Guess   | Output                 | Explanation                                                        |
|---------|------------------------|--------------------------------------------------------------------|
| `12345` | `In Place:5, Exist:5`  | All digits are correct and in the right positions. You win!        |
| `13579` | `In Place:1, Exist:3`  | `1`, `3`, `5` exist in the secret; only `1` is in the right place. |
| `67890` | `In Place:0, Exist:0`  | None of the digits appear in the secret number.                    |

## Winning and Losing

- **Win:** your guess matches the secret number (`In Place:5`) within 10 attempts.
- **Lose:** you use all 10 attempts without finding the number. The game then reveals the secret number.

## Rules Summary

- Secret number: 5 digits, no leading zero, no repeated digits.
- Guesses: 5 digits, no leading zero, no repeated digits.
- Maximum attempts: 10 (invalid guesses are not counted).
- Output after each valid guess: `In Place:<n>, Exist:<m>`.

Good luck!