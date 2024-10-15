use crate::{Result, Error};

use std::path::Path;

pub fn list_files(path: &str) -> Result<Vec<String>> {

    let path = Path::new(path);

    let files: Vec<String> = std::fs::read_dir(path)?
        .filter_map(|re| re.ok())
        .filter(|e| e.file_type().map(|ft| ft.is_file()).unwrap_or(false))
        .filter_map(|e| e.file_name().into_string().ok())
        .collect();

    if files.is_empty() {
        return Err(Error::EmptyFolder);
    }

    Ok(files)
}

// region:    --- Tests

#[cfg(test)]
mod tests {
    type Error = Box<dyn std::error::Error>;
    type Result<T> = core::result::Result<T, Error>; // For tests.

    use std::fs::File;

    use super::*;
                    
    #[test]
    fn test_empty() -> Result<()> {
        // -- Setup & Fixtures
        let temp_dir = tempfile::tempdir()?;
        let temp_path = temp_dir.path();
        
        // -- Exec
        let result = list_files(temp_path.to_str().unwrap());
        
        // -- Check
        match result {
            Ok(_) => panic!("Expected an error, but got Ok"),
            Err(e) => {
                println!("Error: {:?}", e);
                // You can add more specific checks here, e.g.:
                assert!(e.to_string().contains("EmptyFolder"));
            }
        }
        
        Ok(())
    }

    #[test]
    fn test_files() -> Result<()> {
        // -- Setup & Fixtures
        let temp_dir = tempfile::tempdir()?;
        let temp_path = temp_dir.path();
        
        // Create some files
        File::create(temp_path.join("file1.txt"))?;
        File::create(temp_path.join("file2.txt"))?;
        
        // -- Exec
        let result = list_files(temp_path.to_str().unwrap());
        
        // -- Check
        match result {
            Ok(files) => {
                assert_eq!(files.len(), 2);
                assert!(files.contains(&"file1.txt".to_string()));
                assert!(files.contains(&"file2.txt".to_string()));
            },
            Err(e) => panic!("Expected Ok, but got error: {:?}", e),
        }
        
        Ok(())
    }
}

// endregion: --- Tests