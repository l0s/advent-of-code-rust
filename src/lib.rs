pub mod lib {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use std::path::Path;

    pub fn get_lines(file: &str) -> impl Iterator<Item=String> {
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let file_name = String::from(manifest_dir) + file;
        let path = Path::new(&file_name);
        let file = File::open(path).expect("file not found");
        let reader = BufReader::new(file);
        reader.lines().map(Result::unwrap)
    }
}