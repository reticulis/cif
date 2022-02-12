use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use phf::phf_map;
use super::values::*;

pub static POLISH_KEYWORDS: phf::Map<&'static str, KEYWORDS> = phf_map! {
    "CIF" => KEYWORDS::Cif,
    "WERSJA" => KEYWORDS::Version,
    "ROZMIAR" => KEYWORDS::Size(0),
    "szerokość:" => KEYWORDS::Width(0),
    "wysokość:" => KEYWORDS::Height(0),
    "bitów_na_piksel:" => KEYWORDS::Bpp(0),
    "METADANE" => KEYWORDS::Metadata
};

struct Cif {
    line: &'static str
}

impl Cif {
    fn new(line: &'static str) -> Cif {
        Cif {
            line
        }
    }
    fn parse_size() -> KEYWORDS {
        unimplemented!()
    }
    fn parse_height() -> KEYWORDS {
        unimplemented!()
    }

    fn parse_width() -> KEYWORDS {
        unimplemented!()
    }

    fn parse_bpp() -> KEYWORDS {
        unimplemented!()
    }

}

pub fn parse(line: &str) -> KEYWORDS {
    unimplemented!()
}

