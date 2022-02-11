enum KEYWORDS {
    Version(&'static str),
    Size(&'static str),
    Width(&'static str),
    Height(&'static str),
    Bpp(&'static str),
    Metadata(&'static str)
}

const POLISH_KEYWORDS: [KEYWORDS; 6] = [
    KEYWORDS::Version("WERSJA"),
    KEYWORDS::Size("ROZMIAR"),
    KEYWORDS::Width("szerokość"),
    KEYWORDS::Height("wysokość"),
    KEYWORDS::Bpp("bitów_na_pixel"),
    KEYWORDS::Metadata("METADANE")
];

pub fn parse(line: String) {

}