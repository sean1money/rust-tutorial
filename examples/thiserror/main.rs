//! This example used to test the cargo expand command.
//! cargo expand --example thiserror

use std::fs;

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("io error: {0}")]
    IoError(#[from] std::io::Error),
}

fn main() -> Result<(), AppError> {
    let content = read_file()?;
    println!("{}", content);
    Ok(())
}

fn read_file() -> Result<String, AppError> {
    Ok(fs::read_to_string("README.md")?)
}
