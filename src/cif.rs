use super::values::*;
use std::process::exit;

pub struct Cif {
    line: String,
}

impl Cif {
    pub fn new(line: &str) -> Cif {
        Cif {
            line: line.to_owned(),
        }
    }

    pub fn parse(&self, metadata: bool) -> &KEYWORDS {
        if self.line.starts_with(" ") {
            println!("{}{}", ERR_PARSE, self.line);
            exit(1)
        }
        if &self.line == "" || &self.line == "\n" {
            return &KEYWORDS::Empty;
        }

        for i in 0..=3 {
            let polish = POLISH_KEYWORD[i];
            match &self.line.starts_with(polish) {
                true => {
                    return match POLISH_KEYWORDS.get(polish) {
                        Some(k) => k,
                        None => &KEYWORDS::End,
                    }
                }
                false => continue,
            }
        }
        // println!("{}", self.line);
        match self.line.contains(";") {
            true => {
                if !metadata {
                    println!("{}{}", ERR_PARSE, self.line);
                    exit(1)
                } else {
                    return &KEYWORDS::End;
                }
            }
            false => {
                if !metadata {
                    if self.line.split_whitespace().collect::<Vec<&str>>().len() < 2 {
                        println!("{}{}", ERR_PARSE, self.line);
                        exit(1)
                    } else {
                        &KEYWORDS::Empty
                    }
                } else {
                    println!("{}{}", ERR_PARSE, self.line);
                    exit(1)
                }
            }
        }
    }

    pub fn spell_check(&self, key: KEYWORDS) {
        let text = match key {
            KEYWORDS::Cif => "CIF: polish",
            KEYWORDS::Version => "WERSJA jeden",
            _ => "",
        };
        if &self
            .line
            .split_whitespace()
            .collect::<Vec<&str>>()
            .join(" ")
            != text
        {
            println!("{}{}", ERR_PARSE, self.line);
            exit(1)
        }
    }

    pub fn parse_size(&self, x: &mut u32, y: &mut u32, bpp: &mut BPP) {
        let vec = &self.line.split_whitespace().skip(1).collect::<Vec<&str>>();
        match vec.iter().position(|&x| x == POLISH_KEYWORD[4]) {
            Some(w) => match vec.iter().position(|&x| x == POLISH_KEYWORD[5]) {
                Some(h) => match vec.iter().position(|&x| x == POLISH_KEYWORD[6]) {
                    Some(b) => {
                        *x += self.get_number(&vec[w + 1..h]);
                        *y += self.get_number(&vec[h + 1..b]);
                        *bpp = match self.get_number(&vec[b + 1..]) {
                            24 => BPP::B24,
                            32 => BPP::B32,
                            _ => {
                                println!("{}{}", ERR_PARSE, self.line);
                                exit(1)
                            }
                        };
                    }
                    None => {
                        println!("{}{}", ERR_PARSE, self.line);
                        exit(1)
                    }
                },
                None => {
                    println!("{}{}", ERR_PARSE, self.line);
                    exit(1)
                }
            },
            None => {
                println!("{}{}", ERR_PARSE, self.line);
                exit(1)
            }
        }
    }

    pub fn parse_metadata(&self) -> bool {
        let words = self.line.split_whitespace().collect::<Vec<&str>>();
        match words.len() {
            1 => {
                println!("{}{}", ERR_PARSE, self.line);
                exit(1)
            }
            2 => {
                if words[1].contains(":") {
                    println!("{}{}", ERR_PARSE, self.line);
                    exit(1)
                } else {
                    return false;
                }
            }
            _ => {
                if words[1].contains(":") {
                    if words[1].split(":").collect::<Vec<&str>>().len() < 2 {
                        println!("{}{}", ERR_PARSE, self.line);
                        exit(1)
                    }
                    true
                } else {
                    return true;
                }
            }
        }
    }

    fn rgb(&self, bpp: &BPP, vec: &Vec<&str>, result: &mut [u32]) {
        let mut i = 0;
        let p = match bpp {
            BPP::B24 => 3,
            BPP::B32 => 4,
        };
        for x in 0..vec.len() {
            match vec.get(x) {
                Some(&s) => match i == p {
                    true => {
                        println!("{}{}", ERR_PARSE, self.line);
                        exit(1)
                    }
                    false => match s.ends_with(";") {
                        true => {
                            if i == p - 1 {
                                println!("{}{}", ERR_PARSE, self.line);
                                exit(1)
                            }
                            result[i] += self.polish_number(&s[..s.len() - 1]);
                            i += 1
                        }
                        false => result[i] += self.polish_number(&s[..s.len()]),
                    },
                },
                None => {
                    println!("{}{}", ERR_PARSE, self.line);
                    exit(1)
                }
            }
        }
    }

    pub fn parse_rgba(&self, bpp: &BPP) -> [u8; 4] {
        let vec = &self.line.split_whitespace().collect::<Vec<&str>>();
        let mut result: [u32; 4] = [0, 0, 0, 0];

        self.rgb(bpp, &vec, &mut result);

        result.map(|x| {
            if x <= 255 {
                x as u8
            } else {
                println!("{}{}", ERR_PARSE, self.line);
                exit(1)
            }
        })
    }

    pub fn parse_rgb(&self, bpp: &BPP) -> [u8; 3] {
        let vec = &self.line.split_whitespace().collect::<Vec<&str>>();
        let mut result: [u32; 3] = [0, 0, 0];

        self.rgb(bpp, vec, &mut result);

        result.map(|x| {
            if x <= 255 {
                x as u8
            } else {
                println!("{}{}", ERR_PARSE, self.line);
                exit(1)
            }
        })
    }

    fn get_number(&self, arr: &[&str]) -> u32 {
        if arr.len() < 1 {
            println!("{}{}", ERR_PARSE, self.line);
            exit(1)
        }

        let mut result = 0;
        let mut last = 0;
        let mut thousand = false;

        for i in 0..arr.len() {
            let arr = match arr[i].ends_with(",") {
                true => &arr[i][..arr[i].len() - 1],
                false => arr[i],
            };
            match self.polish_number(arr) {
                1000 => {
                    if result == 0 {
                        match arr == "tysiące" || arr == "tysięcy" {
                            true => {
                                println!("{}{}", ERR_PARSE, self.line);
                                exit(1)
                            }
                            false => {
                                result += 1000;
                                thousand = true;
                            }
                        }
                    } else {
                        match thousand
                            || (last < 5 && arr != "tysiące")
                            || (last >= 5 && arr != "tysięcy")
                            || (arr == "tysiąc")
                        {
                            true => {
                                println!("{}{}", ERR_PARSE, self.line);
                                exit(1)
                            }
                            false => {
                                result *= 1000;
                                last = 0;
                                thousand = true;
                            }
                        }
                    }
                }
                i => {
                    if last == 0 {
                        last = i;
                    }
                    if i > last {
                        println!("{}{}", ERR_PARSE, self.line);
                        exit(1)
                    }
                    last = i;
                    result += i
                }
            }
        }
        result
    }

    fn polish_number(&self, arr: &str) -> u32 {
        match THOUSAND.get(arr) {
            Some(&i) => i,
            None => match HUNDREDS.get(arr) {
                Some(&i) => i,
                None => match NTENS.get(arr) {
                    Some(&i) => i,
                    None => match TENS.get(arr) {
                        Some(&i) => i,
                        None => match ONES.get(arr) {
                            Some(&i) => i,
                            None => {
                                println!("{}{}", ERR_PARSE, self.line);
                                exit(1)
                            }
                        },
                    },
                },
            },
        }
    }
}
