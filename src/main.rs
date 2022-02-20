use image::ImageBuffer;
use std::fs::File;
use std::io::{BufRead, BufReader};

mod modules;
mod values;
use crate::args::Args;
use crate::cif::Cif;
use modules::*;

use crate::values::KEYWORDS;

struct Decoder {
    file: File,
    output: String,
}

struct Image {
    width: u32,
    height: u32,
    bpp: u32,
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

        match cif.bpp {
            3 => {
                let mut imgbuf = ImageBuffer::new(cif.width, cif.height);
                for (pixel, buf) in imgbuf.pixels_mut().zip(cif.buf_rgb) {
                    *pixel = image::Rgb(buf)
                }
                imgbuf.save(&self.output).unwrap();
            }
            4 => {
                let mut imgbuf = ImageBuffer::new(cif.width, cif.height);
                for (pixel, buf) in imgbuf.pixels_mut().zip(cif.buf_rgba) {
                    *pixel = image::Rgba(buf)
                }
                imgbuf.save(&self.output).unwrap();
            }
            _ => {}
        }
        println!("SUCCESS!")
    }

    fn parse(&self) -> Image {
        let buf = BufReader::new(&self.file);

        let (
            mut cif_b,
            mut version_b,
            mut size_b,
            mut metadata_b,
            mut metadata,
            mut metadata_num,
            mut width,
            mut height,
            mut bpp,
            mut skip,
        ) = (false, false, false, false, false, 0, 0, 0, 0, 0);

        let mut buf_rgb = Vec::new();
        let mut buf_rgba = Vec::new();

        for (i, line) in buf.lines().enumerate() {
            skip += 1;
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
                            Err(e) => panic!("{} Line: {}", e, i),
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
                            Err(e) => panic!("{} Line: {}", e, i),
                        }
                    }
                    KEYWORDS::Size => {
                        if size_b {
                            panic!("Another line starting with \"SIZE\"! \nNumber line: {}", i)
                        }
                        size_b = true;
                        match cif.parse_size(&mut width, &mut height, &mut bpp) {
                            Ok(()) => (),
                            Err(e) => panic!("{} Line: {}", e, i),
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
                            match bpp {
                                3 => buf_rgb.push(cif.parse_rgb()),
                                4 => buf_rgba.push(cif.parse_rgba()),
                                _ => {
                                    panic!("")
                                }
                            }
                            break;
                        } else {
                            panic!("")
                        }
                    }
                },
                Err(e) => panic!("{}", e),
            }
        }

        let buf = BufReader::new(&self.file);

        for line in buf.lines().skip(skip) {
            let cif = Cif::new(line.expect("Error reading the file!"));
            match bpp {
                3 => buf_rgb.push(cif.parse_rgb()),
                4 => buf_rgba.push(cif.parse_rgba()),
                _ => {
                    panic!("")
                }
            }
        }
        Image {
            width,
            height,
            bpp,
            buf_rgba,
            buf_rgb,
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
