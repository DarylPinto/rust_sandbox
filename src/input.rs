use std::{fmt::Debug, io, str::FromStr};

// Gets input from the user
// Similar to Python's `input` function
pub fn get<T>(question: &str) -> T
where
    T: FromStr,
    T::Err: Debug,
{
    println!("{}", question);

    let mut answer = String::new();
    io::stdin()
        .read_line(&mut answer)
        .expect("Failed to read from stdin!");

    answer.trim().parse().expect("Bad input")
}
