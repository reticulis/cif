use std::fs::File;
use std::io::{BufRead, BufReader};

mod modules;
mod values;
use crate::args::Args;
use crate::cif::Cif;
use modules::*;
use crate::BPP::B24;

use crate::values::{BPP, ERR_PARSE, KEYWORDS};

struct Decoder {
    file: File,
    output: String,
}

struct Image {
    width: u32,
    height: u32,
    bpp: BPP,
    buf_rgb: Vec<[u8; 3]>,
    buf_rgba: Vec<[u8; 4]>,
}

impl Decoder {
    fn new(input: String, output: String) -> Decoder {
        if input.ends_with(".cif") {
            if output.ends_with(".bmp") || output.ends_with(".png") {
                Decoder {
                    file: File::open(input).expect("Error opening the file!"),
                    output,
                }
            } else {
                panic!("Output is not PNG or BMP file!")
            }
        } else {
            panic!("Input is not a CIF file!")
        }
    }

    fn decode(&self) {
        let cif = self.parse();

        // match cif.bpp {
        //     BPP::B24 => {
        //         let mut imgbuf = ImageBuffer::new(cif.width, cif.height);
        //         for (pixel, buf) in imgbuf.pixels_mut().zip(cif.buf_rgb) {
        //             *pixel = image::Rgb(buf)
        //         }
        //         imgbuf.save(&self.output).unwrap();
        //     }
        //     BPP::B32 => {
        //         let mut imgbuf = ImageBuffer::new(cif.width, cif.height);
        //         for (pixel, buf) in imgbuf.pixels_mut().zip(cif.buf_rgba) {
        //             *pixel = image::Rgba(buf)
        //         }
        //         imgbuf.save(&self.output).unwrap();
        //     }
        // }
        println!("SUCCESS!")
    }

    fn parse(&self) {
        let buf = BufReader::new(&self.file);

        let (mut cif_b, mut version_b, mut size_b, mut metadata_b, mut metadata, mut metadata_num, mut x, mut y, mut bpp) =
            (false, false, false, false, false, 0, 0, 0, B24);

        for (i, line) in buf.lines().enumerate() {
            let cif = Cif::new(line.expect("Error reading the file!"));
            match cif.parse(metadata) {
                Ok(k) => match k {
                    KEYWORDS::Cif => {
                        if cif_b {
                            panic!("Another line starting with \"CIF\"! \nNumber line: {}", i)
                        }
                        cif_b = true;
                        match cif.spell_check(k) {
                            Ok(()) => (),
                            Err(e) => panic!("{} Line: {}", e,i)
                        }
                    }
                    KEYWORDS::Version => {
                        if version_b {
                            panic!(
                                "Another line starting with \"VERSION\"! \nNumber line: {}",
                                i
                            )
                        }
                        version_b = true;
                        match cif.spell_check(k) {
                            Ok(()) => (),
                            Err(e) => panic!("{} Line: {}", e,i)
                        }
                    }
                    KEYWORDS::Size => {
                        if size_b {
                            panic!("Another line starting with \"SIZE\"! \nNumber line: {}", i)
                        }
                        size_b = true;
                        match cif.parse_size(&mut x, &mut y, &mut bpp) {
                            Ok(()) => (),
                            Err(e) => panic!("{} Line: {}", e,i)
                        }
                    }
                    KEYWORDS::Metadata => {
                        metadata_b = true;
                        if metadata_num == 2 {
                            panic!(
                                "Another line starting with \"METADATA\"! \nNumber line: {}",
                                i
                            )
                        }
                        match cif.parse_metadata() {
                            Ok(t) => metadata = t,
                            Err(e) => panic!("{} Line: {}", e, i),
                        }
                        metadata_num += 1;
                    }
                    KEYWORDS::Empty => {}
                    KEYWORDS::End => {
                        if cif_b || version_b || metadata_b || size_b {
                            // parse asd
                            break
                        } else {
                            panic!("PANIKA!")
                        }
                    }
                },
                Err(e) => panic!("{}", e),
            }
        }
    }
}

fn main() {
    #[global_allocator]
    static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

    let args = Args::new();

    let decoder = Decoder::new(args.input, args.output);

    decoder.decode()
}
