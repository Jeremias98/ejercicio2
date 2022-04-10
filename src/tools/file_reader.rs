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

    /// Lee un archivo de texto y devuelve un vector de strings con el
    /// contenido de cada línea.
    ///
    /// # Argumentos
    ///
    /// * `path` - Ruta relativa al archivo
    ///
    /// # Ejemplo
    /// ```
    /// use crate::ejercicio2::tools::file_reader::FileReader;
    /// let lineas_archivo = match FileReader::read_file_lines("ruta") {
    ///    Ok(lines) => lines,
    ///    Err(msj) => [].to_vec(),
    /// };
    /// ```
    pub fn read_file_lines(path: &str) -> Result<Vec<String>, String> {
        let mut result: Vec<String> = Vec::new();

        if let Ok(lines) = FileReader::read_lines(path) {
            for line in lines {
                result.push(line.unwrap());
            }
        } else {
            return Err(format!("No se ha encontrado la ruta '{}'", path));
        }

        Ok(result)
    }
}
