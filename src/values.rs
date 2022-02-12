use std::collections::HashMap;
use phf::phf_map;

pub enum KEYWORDS {
    Cif,
    Version,
    Size(i32),
    Height(i32),
    Width(i32),
    Bpp(i32),
    Metadata,
    Rgb(u8, u8, u8)
}

pub static POLISH_KEYWORD: [&'static str; 7] = [
    "CIF",
    "WERSJA",
    "ROZMIAR",
    "szerokość:",
    "wysokość:",
    "bitów_na_piksel",
    "METADANE"
];

pub static HUNDREDS: phf::Map<&'static str, i32> = phf_map! {
    "sto" => 100,
    "dwieście" => 200,
    "trzysta" => 300,
    "czterysta" => 400,
    "pięćset" => 500,
    "sześćset" => 600,
    "siedemset" => 700,
    "osiemset" => 800,
    "dziewięcset" => 900
};

pub static NTENS: phf::Map<&'static str, i32> = phf_map! {
    "dziesięć" => 10,
    "jedenaście" => 11,
    "dwanaście" => 12,
    "trzynaście" => 13,
    "czternaście" => 14,
    "piętnaście" => 15,
    "szesnaście" => 16,
    "siedemnaście" => 17,
    "osiemnaście" => 18,
    "dziewiętnaście" => 19
};

pub static TENS: phf::Map<&'static str, i32> = phf_map! {
    "dwadzieścia" => 20,
    "trzydzieści" => 30,
    "czterdzieści" => 40,
    "pięćdziesiąt" => 50,
    "sześćdziesiąt" => 60,
    "siedemdziesiąt" => 70,
    "osiemdziesiąt" => 80,
    "dziewięćdziesiąt" => 90,
};

pub static ONES: phf::Map<&'static str, i32> = phf_map! {
    "jeden" => 1,
    "dwa" => 2,
    "trzy" => 3,
    "cztery" => 4,
    "pięć" => 5,
    "sześć" => 6,
    "siedem" => 7,
    "osiem" => 8,
    "dziewięć" => 9
};