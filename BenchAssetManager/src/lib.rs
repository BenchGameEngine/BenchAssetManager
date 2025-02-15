#[allow(non_snake_case)]
#[cfg(test)]
mod BenchAssetManager {
    use std::fs::{self, File};

    fn LoadFile<s: AsRef<str>>(FilePath: s) -> Vec<u8> {
        return fs::read(FilePath);
    }

    #[test]
    fn test_func() {
        let Output = LoadFile("A");
    }
}
