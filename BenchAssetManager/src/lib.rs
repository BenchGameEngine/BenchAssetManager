#[allow(non_snake_case)]
#[cfg(test)]
mod BenchAssetManager {
    use std::{
        fs::{self, File},
        process::Output,
    };

    struct AssetFileStruct {
        Data: Vec<u8>,
    }

    fn ReadFile<s: AsRef<str>>(FilePath: s) -> Result<Vec<u8>, String> {
        fs::read(FilePath.as_ref()).map_err(|e| e.to_string())
    }

    fn LoadAssetFile<s: AsRef<str>>(AssetPath: s) -> Result<AssetFileStruct, String> {
        match ReadFile(AssetPath) {
            Ok(data) => return Ok(AssetFileStruct { Data: data }),
            Err(error) => return Err(String::from("Error reading file.")),
        }
    }

    #[test]
    fn test_func() {
        let Output = LoadAssetFile("testfiles/test1");
        match Output {
            Ok(data) => {
                println!("Loaded fine.");
                for num in data.Data {
                    println!("{num}")
                }
            }
            Err(error) => {
                println!("Error loading file: {error}");
            }
        }
    }
}
