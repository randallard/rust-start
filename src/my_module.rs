use crate::{Result, Error};

pub fn my_function(num: u8) -> Result<u8> {

    if num % 3 == 0 {
        return Err(Error::ProjectError);
    }

    Ok(num)
}

// region:    --- Tests

#[cfg(test)]
mod tests {
    type Error = Box<dyn std::error::Error>;
    type Result<T> = core::result::Result<T, Error>; // For tests.

    use super::*;
                                        
    #[test]
    fn test_error() -> Result<()> {
        // -- Setup & Fixtures
        let num = 3;
        
        // -- Exec
        let result = my_function(num);
        
        // -- Check
        match result {
            Ok(_) => panic!("Expected an error, but got Ok"),
            Err(e) => {
                println!("Error: {:?}", e);
                // You can add more specific checks here, e.g.:
                assert!(e.to_string().contains("Error"));
            }
        }
        
        Ok(())
    }

    #[test]
    fn test_ok() -> Result<()> {
        // -- Setup & Fixtures
        let num = 1;
               
        // -- Exec
        let result = my_function(num);
        
        // -- Check
        match result {
            Ok(_) => {
                assert!(num % 3 != 0);
            },
            Err(e) => panic!("Expected Ok, but got error: {:?}", e),
        }
        
        Ok(())
    }
}

// endregion: --- Tests