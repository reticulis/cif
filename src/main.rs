use std::env::args;
use std::fs::File;
use std::io::{BufRead, BufReader};

mod cif;
mod values;
use crate::cif::Cif;
use values::KEYWORDS;

#[derive(PartialEq)]
enum FormatFile {
    Png,
    Bmp,
}

struct Decoder {
    file: File,
    output: String,
}

struct Image {
    width: u32,
    height: u32,
    bpp: u32,
    buf: Vec<[u8; 3]>
}

impl Decoder {
    fn new(input: String, output: String) -> Decoder {
        match input.ends_with(".cif") {
            true => Decoder {
                file: File::open(input).expect("Error opening the file!"),
                output,
            },
            false => panic!("Input is not CIF file!"),
        }
    }

    fn decode(&self) {
        let cif = self.parse();
        let mut imgbuf = image::ImageBuffer::new(cif.width, cif.height);
        for (pixel, buf)in imgbuf.pixels_mut().zip(cif.buf){
            *pixel = image::Rgb(buf)
        }
        imgbuf.save(&self.output).unwrap()
    }

    fn parse(&self) -> Image {
        let buf = BufReader::new(&self.file);
        let (mut width, mut height, mut bpp) = (0,0,0);
        let mut cif_b = false;
        let mut version = false;
        let mut metadata = false;
        let mut end = false;
        let mut image: Vec<[u8; 3]> = vec![];
        for s in buf.lines() {
            match s {
                Ok(s) => {
                    let cif = Cif::new(&s);
                    match end {
                        true => {
                            let r = s.as_str();
                            image.push(cif.parse_rgb())
                        },
                        false => {
                            match cif.parse(metadata) {
                                KEYWORDS::Cif => {
                                    cif.spell_check(KEYWORDS::Cif);
                                    cif_b = true;
                                }
                                KEYWORDS::Version => {
                                    cif.spell_check(KEYWORDS::Version);
                                    version = true;
                                }
                                KEYWORDS::Size => {
                                    cif.parse_size(&mut width, &mut height, &mut bpp)
                                }
                                KEYWORDS::Metadata => {
                                    cif.parse_metadata();
                                    metadata = true
                                },
                                KEYWORDS::Empty => {
                                    metadata = false;
                                    continue
                                },
                                KEYWORDS::End => {
                                    end = true;
                                    image.push(cif.parse_rgb())
                                }
                            }
                        }
                    }
                },
                Err(e) => panic!("{}", e),
            }
        }
        match cif_b && version {
            true => {},
            false => panic!()
        }
        Image {
            width,
            height,
            bpp,
            buf: image
        }
    }
}

fn main() {
    let args = args().collect::<Vec<String>>();
    let input = match args.get(1) {
        Some(s) => s.clone(),
        None => panic!("Error reading input!"),
    };
    let output = match args.get(2) {
        Some(s) => s.clone(),
        None => panic!("Error reading output!"),
    };
    let decoder = Decoder::new(input,output);
    decoder.decode()
}
