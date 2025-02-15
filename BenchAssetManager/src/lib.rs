#[allow(non_snake_case)]
#[cfg(test)]
mod BenchAssetManager {
    use std::{
        fs::{self, File},
        process::Output,
    };

    fn LoadFile<s: AsRef<str>>(FilePath: s) -> Result<Vec<u8>, String> {
        fs::read(FilePath.as_ref()).map_err(|e| e.to_string())
    }

    #[test]
    fn test_func() {
        let Output = LoadFile("testfiles/test1");
        match Output {
            Ok(data) => {
                println!("Loaded fine.");
            }
            Err(error) => {
                println!("Error loading file: {error}");
            }
        }
    }
}
