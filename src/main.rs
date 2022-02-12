use std::env::args;
use std::fs::File;
use std::io::{BufRead, BufReader};
mod cif;
mod values;
use values::KEYWORDS;

#[derive(PartialEq)]
enum FormatFile {
    Png,
    Bmp,
}

struct Decoder {
    file: File,
    output: FormatFile
}

impl Decoder {
    fn new(input: &str, output: &str) -> Decoder {
        match input.ends_with(".cif") {
            true => Decoder {
                file: File::open(input).expect("Error opening the file!"),
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
        let buf = BufReader::new(&self.file);
        let mut size = 0;
        let mut height = 0;
        let mut width = 0;
        let mut bpp = 0;
        for s in buf.lines() {
            match s {
                Ok(s) => match cif::parse(&s) {
                    KEYWORDS::Cif => continue,
                    KEYWORDS::Version => continue,
                    KEYWORDS::Size(i) => size = i,
                    KEYWORDS::Height(i) => height = i,
                    KEYWORDS::Width(i) => width = i,
                    KEYWORDS::Bpp(i) => bpp = i,
                    KEYWORDS::Metadata => continue,
                    KEYWORDS::Rgb(r,g,b) => unimplemented!(),
                },
                Err(e) => panic!("{}", e)
            }
        }
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
    decoder.decode_to_png()
}

