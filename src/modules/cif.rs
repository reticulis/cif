use crate::values::*;

pub struct Cif {
    line: String,
}

impl Cif {
    pub fn new(line: String) -> Cif {
        Cif { line }
    }
    pub fn parse(&self, metadata: bool) -> Result<KEYWORDS, &str> {
        if self.line.starts_with(" ") {
            return Err(ERR_PARSE);
        }
        if self.line == "" || self.line == "\n" {
            return Ok(KEYWORDS::Empty);
        }

        for i in 0..=3 {
            if let Some(&polish) = POLISH_KEYWORD.get(i) {
                if self.line.starts_with(polish) {
                    return match POLISH_KEYWORDS.get(polish) {
                        Some(&k) => Ok(k),
                        None => Ok(KEYWORDS::End),
                    };
                } else {
                    continue;
                }
            }
        }

        if self.line.contains(";") {
            if metadata {
                Err("Error parsing the file!")
            } else {
                Ok(KEYWORDS::End)
            }
        } else {
            if metadata {
                if self.line.split_ascii_whitespace().collect::<Vec<&str>>().len() < 2 {
                    Err("Error parsing the file!")
                } else {
                    Ok(KEYWORDS::Empty)
                }
            } else {
                Err("Error parsing the file!")
            }
        }
    }

    pub fn spell_check(&self, key: KEYWORDS) -> Result<(), &str> {
        let text = match key {
            KEYWORDS::Cif => "CIF: polish",
            KEYWORDS::Version => "WERSJA jeden",
            _ => "",
        };
        if self
            .line
            .split_ascii_whitespace()
            .collect::<Vec<&str>>()
            .join(" ")
            != text
        {
            return Err(ERR_PARSE);
        }
        Ok(())
    }

    pub fn parse_metadata(&self) -> Result<bool, &str> {
        let words = self.line.split_ascii_whitespace().collect::<Vec<&str>>();
        match words.len() {
            1 => Err(ERR_PARSE),
            2 => {
                if words[1].contains(":") {
                    Err(ERR_PARSE)
                } else {
                    Ok(true)
                }
            }
            _ => {
                if words[1].contains(":") {
                    if words[1].split(":").collect::<Vec<&str>>().len() < 2 {
                        return Err(ERR_PARSE);
                    }
                    return Ok(false);
                } else {
                    return Ok(false);
                }
            }
        }
    }
    pub fn parse_size(&self, x: &mut u32, y: &mut u32, bpp: &mut u32) -> Result<(), &str> {
        let vec = self.line.split_ascii_whitespace().skip(1).collect::<Vec<&str>>();
        match vec.iter().position(|&x| x == POLISH_KEYWORD[4]) {
            Some(w) => match vec.iter().position(|&x| x == POLISH_KEYWORD[5]) {
                Some(h) => match vec.iter().position(|&x| x == POLISH_KEYWORD[6]) {
                    Some(b) => {
                        *x += match self.get_number(&vec[w + 1..h]) {
                            Ok(n) => n,
                            Err(e) => return Err(e),
                        };
                        *y += match self.get_number(&vec[h + 1..b]) {
                            Ok(n) => n,
                            Err(e) => return Err(e),
                        };
                        *bpp = match self.get_number(&vec[b + 1..]) {
                            Ok(n) => match n {
                                24 => 3,
                                32 => 4,
                                _ => return Err(ERR_PARSE),
                            },
                            Err(e) => return Err(e),
                        };
                    }
                    None => return Err(ERR_PARSE),
                },
                None => return Err(ERR_PARSE),
            },
            None => return Err(ERR_PARSE),
        }
        Ok(())
    }

    fn get_number(&self, arr: &[&str]) -> Result<u32, &str> {
        if arr.len() < 1 {
            return Err(ERR_PARSE);
        }

        let mut result = 0;
        let mut last = 0;
        let mut thousand = false;

        for i in 0..arr.len() {
            let arr = match arr[i].ends_with(",") {
                true => match arr.get(i) {
                    Some(&s) => &s[..s.len() - 1],
                    None => panic!("asd"),
                },
                false => arr.get(i).unwrap(),
            };
            match self.polish_number(arr) {
                Ok(i) => match i {
                    1000 => {
                        if result == 0 {
                            match arr == "tysiące" || arr == "tysięcy" {
                                true => return Err(ERR_PARSE),
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
                                true => return Err(ERR_PARSE),
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
                            return Err(ERR_PARSE);
                        }
                        last = i;
                        result += i
                    }
                },
                Err(e) => return Err(e),
            }
        }
        return Ok(result);
    }

    fn rgb(&self, bpp: usize, vec: &Vec<&str>, result: &mut [u8]) {
        let mut i = 0;
        for x in 0..vec.len() {
            match vec.get(x) {
                Some(&s) => match i == bpp {
                    true => {
                        panic!("{}", ERR_PARSE)
                    }
                    false => match s.ends_with(";") {
                        true => {
                            if i == bpp - 1 {
                                panic!("{}", ERR_PARSE)
                            }
                            result[i] += match self.polish_number(&s[..s.len() - 1]) {
                                Ok(u) => {
                                    if u <= 255 {
                                        u as u8
                                    } else {
                                        panic!("{}", ERR_PARSE)
                                    }
                                }
                                Err(_) => panic!("{}", ERR_PARSE),
                            };
                            i += 1
                        }
                        false => {
                            result[i] += match self.polish_number(s) {
                                Ok(u) => {
                                    if u <= 255 {
                                        u as u8
                                    } else {
                                        panic!("{}", ERR_PARSE)
                                    }
                                }
                                Err(_) => panic!("{}", ERR_PARSE),
                            }
                        }
                    },
                },
                None => {
                    panic!("{}", ERR_PARSE)
                }
            }
        }
    }

    pub fn parse_rgba(&self) -> [u8; 4] {
        let vec = &self.line.split_ascii_whitespace().collect::<Vec<&str>>();
        let mut result: [u8; 4] = [0, 0, 0, 0];

        self.rgb(4, &vec, &mut result);

        result
    }

    pub fn parse_rgb(&self) -> [u8; 3] {
        let vec = &self.line.split_ascii_whitespace().collect::<Vec<&str>>();
        let mut result: [u8; 3] = [0, 0, 0];

        self.rgb(3, vec, &mut result);

        result
    }

    fn polish_number(&self, arr: &str) -> Result<u32, &str> {
        match POL.get(arr) {
            Some(&i) => Ok(i),
            None => Err(ERR_PARSE)
        }
    }
}
