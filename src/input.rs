use std::{fmt::Debug, io, io::Write, str::FromStr};

// Gets input from the user
// Similar to Python's `input` function
pub fn get<T>(question: &str) -> T
where
    T: FromStr,
    T::Err: Debug,
{
    print!("{}", question);
    io::stdout().flush().expect("Failed to flush stdout!");

    let mut answer = String::new();
    io::stdin()
        .read_line(&mut answer)
        .expect("Failed to read from stdin!");

    answer.trim().parse().expect("Bad input")
}
