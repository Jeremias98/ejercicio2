use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub struct FileReader;

impl FileReader {
    fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where
        P: AsRef<Path>,
    {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }

    pub fn read_file_lines(path: &str) -> Result<Vec<String>, String> {
        let mut lineas: Vec<String> = Vec::new();

        if let Ok(lines) = FileReader::read_lines(path) {
            for line in lines {
                lineas.push(line.unwrap());
            }
        } else {
            return Err(format!("No se ha encontrado la ruta '{}'", path));
        }

        Ok(lineas)
    }
}
