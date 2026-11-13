# guess_game

A command-line number guessing game. The program picks a secret five-digit number, and you try to guess it within ten attempts, using feedback after each guess.

## How It Works

When the program starts, it generates a secret number with these properties:

- exactly **5 digits**
- **does not start with 0**
- **no repeated digits** (every digit is unique)

You then enter guesses one at a time. Your guess must follow the same rules as the secret number:

- exactly 5 digits
- does not start with 0
- no repeated digits

You have a maximum of **10 attempts**.

## Requirements

- [Rust](https://www.rust-lang.org/tools/install) toolchain (`rustc` and `cargo`)

### Dependencies

| Crate    | Purpose                                                                                          |
|----------|--------------------------------------------------------------------------------------------------|
| `rand`   | Generates the random secret number.                                                              |
| `chrono` | Timestamps for a logger function used only during development and testing, not in normal play.   |

## Build

```sh
cargo build --release
```

The executable is created at `target/release/guess_game`.

## Usage

Run the executable:

```sh
./target/release/guess_game
```

Or build and run in one step:

```sh
cargo run --release
```

## Feedback

After each valid guess, the game reports two values:

```
in place: 1 exist: 3
```

| Value      | Meaning                                                                                      |
|------------|----------------------------------------------------------------------------------------------|
| `in place` | How many digits of your guess are in the secret number **and** at exactly the same position. |
| `exist`    | How many digits of your guess appear in the secret number, regardless of position.           |

Since every digit is unique, `in place` can never be greater than `exist`.

### Example

Suppose the secret number is `12345`:

| Guess   | Output                    | Explanation                                                        |
|---------|---------------------------|--------------------------------------------------------------------|
| `12345` | `in place: 5 exist: 5`    | All digits are correct and in the right positions. You win!        |
| `13579` | `in place: 1 exist: 3`    | `1`, `3`, `5` exist in the secret; only `1` is in the right place. |
| `67890` | `in place: 0 exist: 0`    | None of the digits appear in the secret number.                    |

## Winning and Losing

- **Win:** your guess matches the secret number (`in place: 5`) within 10 attempts.
- **Lose:** you use all 10 attempts without finding the number.

## Rules Summary

- Secret number: 5 digits, no leading zero, no repeated digits.
- Guesses: 5 digits, no leading zero, no repeated digits.
- Maximum attempts: 10.
- Output after each guess: `in place: <n> exist: <m>`.

Good luck!