pub const SOLID_VLINE_CHAR: char = '│';
pub const SOLID_HLINE_CHAR: char = '─';
pub const SOLID_TLCORNER_CHAR: char = '┌';
pub const SOLID_TRCORNER_CHAR: char = '┐';
pub const SOLID_BLCORNER_CHAR: char = '└';
pub const SOLID_BRCORNER_CHAR: char = '┘';

pub const DASHED_VLINE_CHAR: char = '┄';
pub const DASHED_HLINE_CHAR: char = '┆';
pub const DASHED_TLCORNER_CHAR: char = '┌';
pub const DASHED_TRCORNER_CHAR: char = '┐';
pub const DASHED_BLCORNER_CHAR: char = '└';
pub const DASHED_BRCORNER_CHAR: char = '┘';

static ROMAN_PAIRS: &'static [(&'static str, u16)] = &[
  ("M", 1000), ("CM", 900), ("D",  500), ("CD", 400),
  ("C", 100),  ("XC", 90),  ("L",  50),  ("XL", 40),
  ("X", 10),   ("IX", 9),   ("V",  5),   ("IV", 4),
  ("I", 1)
];

pub fn num_to_roman(n: u64) -> String {
    let mut out = String::new();
    let mut n = n;
    for &(name, value) in ROMAN_PAIRS.iter() {
        while n >= value as u64 {
            n -= value as u64;
            out.push_str(name);
        }
    }
    out
}

pub fn num_to_latin(n: u64) -> String {
    let mut out = String::new();
    let mut prefix = n / 26;
    while prefix > 26 {
        out.push_str("A");
        prefix = prefix / 26;
    }
    if prefix < 1 {
        out.push((65 + n) as u8 as char);
    } else {
        out.push((64 + prefix) as u8 as char);
        out.push((prefix * 26 % 26) as u8 as char);
    }
    out
}
