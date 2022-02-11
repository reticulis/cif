use std::env::args;
use std::fs::File;
use std::io::BufReader;
mod cif;

#[derive(PartialEq)]
enum FormatFile {
    Png,
    Bmp,
}

struct Decoder {
    file: BufReader<File>,
    output: FormatFile
}

impl Decoder {
    fn new(input: &str, output: &str) -> Decoder {
        match input.ends_with(".cif") {
            true => Decoder {
                file: BufReader::new(File::open(input).expect("Error opening the file!")),
                output: Decoder::decide(output)
            },
            false => panic!("Input is not CIF file!")
        }
    }

    fn decide(format: &str) -> FormatFile {
        match format.ends_with(".png") {
            true => FormatFile::Png,
            false => match format.ends_with(".bmp") {
                true => FormatFile::Bmp,
                false => panic!("Invalid output file format!")
            }
        }
    }

    fn decode(&self) {
        match &self.output == &FormatFile::Png {
            true => self.decode_to_png(),
            false => self.decode_to_bmp()
        }
    }
    fn decode_to_png(&self) {
        unimplemented!()
    }

    fn decode_to_bmp(&self) {
        unimplemented!()
    }
}

fn main() {
    let args = args().collect::<Vec<String>>();
    let input = match args.get(1) {
        Some(s) => s,
        None => panic!("Error reading input!")
    };
    let output = match args.get(2) {
        Some(s) => s,
        None => panic!("Error reading output!")
    };
    let decoder = Decoder::new(input, output);
}

