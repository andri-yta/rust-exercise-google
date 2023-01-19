use thiserror::Error;
mod unsafe_rust;

#[derive(Error, Debug, PartialEq)]
enum NumberError {
    #[error("{0} is too small")]
    TooSmall(i32),
    #[error("{0} is too big")]
    TooBig(i32),
}

/// Checks whether the number is between 10 - 100.
///
/// ```
/// use playground::check;
/// assert_eq!(check(30), Ok(30));
/// ```
fn check(number: i32) -> Result<i32, NumberError> {
    if number >= 100 {
        return Err(NumberError::TooBig(number))?;
    } else if number < 10 {
        return Err(NumberError::TooSmall(number))?;
    }
    Ok(number)
}

fn main() {
    unsafe_rust::unsafe_rust::data_race();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check() {
        assert_eq!(check(177), Err(NumberError::TooBig(177)));
    }
    
}