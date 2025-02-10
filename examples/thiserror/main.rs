//! This example used to test the cargo expand command.
//! cargo expand --example thiserror

use anyhow::anyhow;
// use std::fs;
use thiserror::Error;

/// Error propogation can lose information when
/// using thiserror and anyhow together,
/// below example try to explain pros and cons.
#[derive(Debug, Error)]
pub enum MyError {
    /// When propogate up to `main() -> anyhow::Error`, `Display` output is used
    #[error("An error occurred")] // format `Display` and `to_string()`
    AnError,

    /// Bad practice:
    /// because Display output will contains only `String` info.
    /// Lose context about it is `AnErrorWithMessage` variant.
    #[error("{0}")]
    AnErrorWithMessage(String),

    /// Good practice:
    /// This make sure when `to_string()` it will include the source error.
    /// when propogating this error to anyhow::Error, the source error printed twice,
    /// but better than not having it at all.
    ///
    /// ## Output from println!("{}")
    /// ```
    /// AnErrorWithSource: Custom { kind: Other, error: "An i/o error occurred" }
    ///
    /// ```
    ///
    /// ## Ouput from anyhow::Error
    /// ```
    /// Error: AnErrorWithSource: Custom { kind: Other, error: "An i/o error occurred" }
    ///
    /// Caused by:
    ///     An i/o error occurred
    /// ```
    #[error("AnErrorWithSource: {0:?}")]
    // NOTE: {0:?} is used to print the source error as debug output
    AnErrorWithSource(#[from] std::io::Error),

    /// Bad practice:
    /// When propogate to `main() -> anyhow::Error`, only inner information is
    /// printed, lose the context info.
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

#[test]
fn test_1() -> anyhow::Result<()> {
    // display error
    println!("{}", MyError::AnError);
    // debug error
    println!("{:?}", MyError::AnError);

    return Err(MyError::AnError.into());
}

#[test]
fn test_2() -> anyhow::Result<()> {
    // Display error
    println!(
        "{}",
        MyError::AnErrorWithMessage("An error with message".to_string())
    );
    // Debug error
    println!(
        "{:?}",
        MyError::AnErrorWithMessage("An error with message".to_string())
    );

    return Err(MyError::AnErrorWithMessage("An error with message".to_string()).into());
}

#[test]
fn test_3() -> anyhow::Result<()> {
    // Display error
    println!(
        "{}",
        MyError::from(std::io::Error::new(
            std::io::ErrorKind::Other,
            "An i/o error occurred"
        ))
    );
    // Debug error
    println!(
        "{:?}",
        MyError::from(std::io::Error::new(
            std::io::ErrorKind::Other,
            "An i/o error occurred"
        ))
    );

    return Err(MyError::from(std::io::Error::new(
        std::io::ErrorKind::Other,
        "An i/o error occurred",
    ))
    .into());
}

#[test]
fn test_4() -> anyhow::Result<()> {
    // Display error
    println!("{}", MyError::from(anyhow!("An anyhow error occurred")));
    // Debug error
    println!("{:?}", MyError::from(anyhow!("An anyhow error occurred!")));

    return Err(MyError::from(anyhow!("An anyhow error occurred")).into());
}

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("io error: {0}")]
    IoError(#[from] std::io::Error),
}

// fn read_file() -> Result<String, AppError> {
//     Ok(fs::read_to_string("README.md")?)
// }

// fn main() -> Result<(), AppError> {
//     let content = read_file()?;
//     println!("{}", content);

//     Ok(())
// }

fn main() -> anyhow::Result<()> {
    // Does NOT tell this is MyError::Other variant
    return Err(MyError::from(anyhow!("An anyhow error occurred")).into());
}
