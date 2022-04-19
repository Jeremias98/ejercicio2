pub mod tools;

use tools::arg_parser::ArgParser;
use tools::diff::Diff;
use tools::file_reader::FileReader;

pub fn run() -> Result<(), String> {
    let params = ArgParser::get_params();

    if params.len() < 3 {
        return Err("Debe proveer dos rutas para hacer el diff".to_string());
    }

    let left = FileReader::read_file_lines(&params[1])?;
    let right = FileReader::read_file_lines(&params[2])?;

    let diff = Diff::new(left, right);
    diff.print();

    Ok(())
}
