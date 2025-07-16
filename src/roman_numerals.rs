fn overline(s: &str) -> String {
    s.chars().map(|c| format!("{}\u{305}", c)).collect()
}

pub fn to_roman(mut num: u128) -> String {
    if num == 0 {
        return String::from("");
    }
    let mut result = String::new();
    if num >= 4000 {
        let m_count = num / 1000;
        let roman = to_roman_basic(m_count as u32);
        result.push_str(&overline(&roman));
        num = num % 1000;
    }
    result.push_str(&to_roman_basic(num as u32));
    result
}

fn to_roman_basic(mut num: u32) -> String {
    let numerals = [ 
        (1000, "M"), (900, "CM"), (500, "D"), (400, "CD"),
        (100, "C"), (90, "XC"), (50, "L"), (40, "XL"),
        (10, "X"), (9, "IX"), (5, "V"), (4, "IV"), (1, "I")
    ];
    let mut result = String::new();
    for &(value, symbol) in numerals.iter() {
        while num >= value {
            result.push_str(symbol);
            num -= value;
        }
    }
    result
} 
