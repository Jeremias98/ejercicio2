mod tools;

use tools::arg_parser::ArgParser;
use tools::diff::Diff;
use tools::file_reader::FileReader;

pub fn run() {
    let params = ArgParser::get_params();

    if params.len() < 3 {
        panic!("Debe proveer dos rutas para hacer el diff");
    }

    let left = match FileReader::read_file_lines(&params[1]) {
        Ok(lines) => lines,
        Err(msj) => panic!("{}", msj),
    };

    let right = match FileReader::read_file_lines(&params[2]) {
        Ok(lines) => lines,
        Err(msj) => panic!("{}", msj),
    };

    let diff = Diff::new(left, right);
    diff.print();
}
