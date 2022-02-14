use image::ImageBuffer;
use std::env::args;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::exit;

mod cif;
mod values;
use crate::cif::Cif;
use crate::values::BPP;
use values::KEYWORDS;

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

        match cif.bpp {
            BPP::B24 => {
                let mut imgbuf = ImageBuffer::new(cif.width, cif.height);
                for (pixel, buf) in imgbuf.pixels_mut().zip(cif.buf_rgb) {
                    *pixel = image::Rgb(buf)
                }
                imgbuf.save(&self.output).unwrap();
            }
            BPP::B32 => {
                let mut imgbuf = ImageBuffer::new(cif.width, cif.height);
                for (pixel, buf) in imgbuf.pixels_mut().zip(cif.buf_rgba) {
                    *pixel = image::Rgba(buf)
                }
                imgbuf.save(&self.output).unwrap();
            }
        }
        println!("SUCCESS!")
    }

    fn parse(&self) -> Image {
        let buf = BufReader::new(&self.file);

        let (mut width, mut height, mut bpp) = (0, 0, BPP::B24);

        let mut cif_b = false;
        let mut version = false;
        let mut metadata = false;
        let mut size = false;
        let mut end = false;

        let mut buf_rgb = Vec::new();
        let mut buf_rgba = Vec::new();

        for s in buf.lines() {
            match s {
                Ok(s) => {
                    let cif = Cif::new(&s);
                    if !end {
                        match cif.parse(metadata) {
                            KEYWORDS::Cif => {
                                if cif_b {
                                    {
                                        println!("Error parsing the file! KEYWORDS::Cif");
                                        exit(1)
                                    }
                                }
                                cif.spell_check(KEYWORDS::Cif);
                                cif_b = true;
                            }
                            KEYWORDS::Version => {
                                if version {
                                    {
                                        println!("Error parsing the file!: KEYWORDS::Version");
                                        exit(1)
                                    }
                                }
                                cif.spell_check(KEYWORDS::Version);
                                version = true;
                            }
                            KEYWORDS::Size => {
                                if size {
                                    {
                                        println!("Error parsing the file! KEYWORDS::Size");
                                        exit(1)
                                    }
                                }
                                cif.parse_size(&mut width, &mut height, &mut bpp);
                                size = true;
                            }
                            KEYWORDS::Metadata => {
                                metadata = cif.parse_metadata();
                            }
                            KEYWORDS::Empty => {
                                metadata = true;
                                continue;
                            }
                            KEYWORDS::End => {
                                if end {
                                    continue;
                                }
                                end = true;
                                match bpp {
                                    BPP::B24 => buf_rgb.push(cif.parse_rgb(&bpp)),
                                    BPP::B32 => buf_rgba.push(cif.parse_rgba(&bpp)),
                                }
                            }
                        }
                    } else {
                        match bpp {
                            BPP::B24 => buf_rgb.push(cif.parse_rgb(&bpp)),
                            BPP::B32 => buf_rgba.push(cif.parse_rgba(&bpp)),
                        }
                    }
                }
                Err(e) => {
                    println!("Error parsing the file! {}", e);
                    exit(1)
                }
            }
        }
        if !(cif_b && version && end) {
            {
                println!("Error parsing the file! cif_b && version && end");
                exit(1)
            }
        }
        Image {
            width,
            height,
            bpp,
            buf_rgb,
            buf_rgba,
        }
    }
}

fn main() {
    #[global_allocator]
    static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

    let args = args().collect::<Vec<String>>();
    let input = match args.get(1) {
        Some(s) => s.to_owned(),
        None => panic!("Error reading input!"),
    };
    let output = match args.get(2) {
        Some(s) => s.to_owned(),
        None => panic!("Error reading output!"),
    };

    let decoder = Decoder::new(input, output);

    decoder.decode()
}
