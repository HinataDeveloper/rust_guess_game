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

#[derive(Debug)]
pub enum GameResult {
    Loss,
    Win,
}
