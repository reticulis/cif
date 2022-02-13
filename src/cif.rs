use super::values::*;

pub struct Cif {
    line: String,
}

impl Cif {
    pub fn new(line: &str) -> Cif {
        Cif { line: line.to_owned() }
    }

    pub fn parse(&self, metadata: bool) -> &KEYWORDS {
        match &self.line.starts_with(" ") {
            true => panic!("Error parsing the file!"),
            false => {}
        }
        match &self.line == "" || &self.line == "\n" {
            true => return &KEYWORDS::Empty,
            false => {}
        }
        for i in 0..=3 {
            let polish = POLISH_KEYWORD.get(i).unwrap();
            match &self.line.starts_with(polish) {
                true => match POLISH_KEYWORDS.get(polish) {
                    Some(k) => return k,
                    None => return &KEYWORDS::End,
                },
                false => continue,
            }
        }
        match &self.line.contains(";") {
            true => &KEYWORDS::End,
            false => &KEYWORDS::Empty
        }
    }

    pub fn spell_check(&self, key: KEYWORDS) {
        let text = match key {
            KEYWORDS::Cif => "CIF: polish",
            KEYWORDS::Version => "WERSJA jeden",
            _ => panic!(""),
        };
        match &self
            .line
            .split_whitespace()
            .collect::<Vec<&str>>()
            .join(" ")
            == text
        {
            true => (),
            false => panic!(""),
        }
    }
    pub fn parse_size(&self, x: &mut u32, y: &mut u32, bpp: &mut u32) {
        let vec = &self.line.split_whitespace().skip(1).collect::<Vec<&str>>();
        match vec.iter().position(|&x| x == "szerokość:") {
            Some(w) => match vec.iter().position(|&x| x == "wysokość:") {
                Some(h) => match vec.iter().position(|&x| x == "bitów_na_piksel:") {
                    Some(b) => {
                        *x += self.get_number(&vec[w + 1..h], ',');
                        *y += self.get_number(&vec[h + 1..b], ',');
                        *bpp += match self.get_number(&vec[b + 1..], ',') {
                            24 => 24,
                            32 => 32,
                            _ => panic!("Error parsing the file!")
                        };
                    }
                    // println!("{:?} {}", &vec[w+1..h], h);
                    // println!("{}", self.parse_width(&vec[h+1..]))
                    None => panic!("Error parsing the file!"),
                },
                None => panic!("Error parsing the file!"),
            },
            None => panic!("Error parsing the file!"),
        }
    }

    pub(crate) fn parse_metadata(&self) {

    }

    pub fn parse_rgb(&self) -> [u8; 3] {
        let vec = &self.line.split_whitespace().collect::<Vec<&str>>();
        let mut i = 0;
        let mut result = [0,0,0];
        for x in 0..vec.len() {
            match vec.get(x) {
                Some(&s) => {
                    match i == 3 {
                        true => panic!("Error parsing the file!"),
                        false => match s.ends_with(";") {
                            true => {
                                result[i] += self.polish_number(&s[..s.len()-1]);
                                i += 1
                            },
                            false => result[i] += self.polish_number(&s[..s.len()])
                        }
                    }
                },
                None => panic!()
            }
        }
        result.iter().for_each(|&x| if x <= 255 {} else {panic!()});
        result.map(|x| x as u8)
    }

    fn get_number(&self, arr: &[&str], ch: char) -> u32 {
        match arr.len() {
            0 => panic!("Error parsing the file!"),
            1 => match NTENS.get(arr[0]) {
                    Some(&i) => return i,
                    None => {}
                },
            _ => {}
        }
        let mut result = 0;
        for i in 0..arr.len() {
            let arr = match arr[i].ends_with(ch) {
                true => {
                    &arr[i][..arr[i].len()-1]
                },
                false => arr[i]
            };
            match self.polish_number(arr) {
                1000 => result *= 1000,
                i => result += i
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
                            Some(&i) =>  i,
                            None => match THOUSAND.get(arr) {
                                Some(&i) => i,
                                None => panic!("Error parsing the file!"),
                            },
                        },
                    }
                },
            }
        }
    }
}
