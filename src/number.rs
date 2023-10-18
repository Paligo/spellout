use icu_locid::{langid, Locale};

const ENGLISH_UNITS: [&str; 20] = [
    "Zero",
    "One",
    "Two",
    "Three",
    "Four",
    "Five",
    "Six",
    "Seven",
    "Eight",
    "Nine",
    "Ten",
    "Eleven",
    "Twelve",
    "Thirteen",
    "Fourteen",
    "Fifteen",
    "Sixteen",
    "Seventeen",
    "Eighteen",
    "Nineteen",
];

const ENGLISH_TENS: [&str; 10] = [
    "", "Ten", "Twenty", "Thirty", "Forty", "Fifty", "Sixty", "Seventy", "Eighty", "Ninety",
];

#[derive(Debug)]
enum Error {
    UnsupportedLocale,
    UnsupportedNumberType,
    NumberOutOfRange,
}

enum NumberType {
    Cardinal,
    Ordinal,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Case {
    Lower,
    Upper,
    Title,
}

struct NumberModifier {
    number_type: NumberType,
    case: Case,
    format: String,
}

impl NumberModifier {
    fn new(number_type: NumberType, case: Case, format: String) -> Self {
        NumberModifier {
            number_type,
            case,
            format,
        }
    }
}

type SpelloutNumber = Box<dyn Fn(u64) -> Result<String, Error>>;

fn spellout_number(locale: Locale, modifier: NumberModifier) -> Result<SpelloutNumber, Error> {
    let langid_en = langid!("en");
    let langid_sv = langid!("sv");

    let spellout_number = if locale.id == langid_en {
        match modifier.number_type {
            NumberType::Cardinal => {
                Ok(number_en_cardinal as fn(u64, &str) -> Result<String, Error>)
            }
            NumberType::Ordinal => Ok(number_en_ordinal as fn(u64, &str) -> Result<String, Error>),
        }
    } else if locale.id == langid_sv {
        match modifier.number_type {
            NumberType::Cardinal => {
                Ok(number_sv_cardinal as fn(u64, &str) -> Result<String, Error>)
            }
            _ => Err(Error::UnsupportedNumberType),
        }
    } else {
        Err(Error::UnsupportedLocale)
    }?;

    Ok(Box::new(move |num| {
        spellout_number(num, &modifier.format).map(|s| match modifier.case {
            Case::Lower => s.to_lowercase(),
            Case::Upper => s.to_uppercase(),
            Case::Title => s,
        })
    }))
}

fn number_en_cardinal(num: u64, format: &str) -> Result<String, Error> {
    match num {
        0..=19 => Ok(ENGLISH_UNITS[num as usize].to_string()),
        20..=99 => {
            let tens = num / 10;
            let units = num % 10;
            let tens_str = ENGLISH_TENS[tens as usize].to_string();
            if units == 0 {
                Ok(tens_str)
            } else {
                let ones = number_en_cardinal(units, format)?;
                Ok(format!("{}-{}", tens_str, ones))
            }
        }
        100..=999 => {
            let hundreds = num / 100;
            let remainder = num % 100;
            let count = number_en_cardinal(hundreds, format)?;
            let hundreds_str = format!("{} Hundred", count);
            if remainder == 0 {
                Ok(hundreds_str)
            } else {
                let remainder_str = number_en_cardinal(remainder, format)?;
                Ok(format!("{} and {}", hundreds_str, remainder_str))
            }
        }
        _ => Err(Error::NumberOutOfRange),
    }
}

fn number_en_ordinal(num: u64, format: &str) -> Result<String, Error> {
    match num {
        2 => Ok("Second".to_string()),
        _ => Err(Error::NumberOutOfRange),
    }
}

fn number_sv_cardinal(num: u64, format: &str) -> Result<String, Error> {
    match num {
        2 => Ok("Två".to_string()),
        3 => Ok("Tre".to_string()),
        _ => Err(Error::NumberOutOfRange),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use icu_locid::locale;

    #[test]
    fn test_spellout_number_three() {
        let modifier = NumberModifier::new(NumberType::Cardinal, Case::Lower, "".to_string());
        let spellout_number_en_cardinal = spellout_number(locale!("en"), modifier).unwrap();
        assert_eq!(spellout_number_en_cardinal(3).unwrap(), "three");
    }

    #[test]
    fn test_spellout_number_two() {
        let modifier = NumberModifier::new(NumberType::Cardinal, Case::Lower, "".to_string());
        let spellout_number_en_cardinal = spellout_number(locale!("en"), modifier).unwrap();
        assert_eq!(spellout_number_en_cardinal(2).unwrap(), "two");
    }
    #[test]
    fn test_spellout_number_two_ordinal() {
        let modifier = NumberModifier::new(NumberType::Ordinal, Case::Lower, "".to_string());
        let spellout_number_en_ordinal = spellout_number(locale!("en"), modifier).unwrap();
        assert_eq!(spellout_number_en_ordinal(2).unwrap(), "second");
    }

    #[test]
    fn test_spellout_number_three_cardinal_swedish() {
        let modifier = NumberModifier::new(NumberType::Cardinal, Case::Lower, "".to_string());
        let spellout_number_en_cardinal = spellout_number(locale!("sv"), modifier).unwrap();
        assert_eq!(spellout_number_en_cardinal(3).unwrap(), "tre");
    }

    #[test]
    fn test_spellout_number_title_case() {
        let modifier = NumberModifier::new(NumberType::Cardinal, Case::Title, "".to_string());
        let spellout_number_en_cardinal = spellout_number(locale!("en"), modifier).unwrap();
        assert_eq!(spellout_number_en_cardinal(3).unwrap(), "Three");
    }

    #[test]
    fn test_spellout_number_18() {
        let modifier = NumberModifier::new(NumberType::Cardinal, Case::Title, "".to_string());
        let spellout_number_en_cardinal = spellout_number(locale!("en"), modifier).unwrap();
        assert_eq!(spellout_number_en_cardinal(18).unwrap(), "Eighteen");
    }

    #[test]
    fn test_spellout_number_20() {
        let modifier = NumberModifier::new(NumberType::Cardinal, Case::Title, "".to_string());
        let spellout_number_en_cardinal = spellout_number(locale!("en"), modifier).unwrap();
        assert_eq!(spellout_number_en_cardinal(20).unwrap(), "Twenty");
    }

    #[test]
    fn test_spellout_number_21() {
        let modifier = NumberModifier::new(NumberType::Cardinal, Case::Title, "".to_string());
        let spellout_number_en_cardinal = spellout_number(locale!("en"), modifier).unwrap();
        assert_eq!(spellout_number_en_cardinal(21).unwrap(), "Twenty-One");
    }

    #[test]
    fn test_spellout_number_100() {
        let modifier = NumberModifier::new(NumberType::Cardinal, Case::Title, "".to_string());
        let spellout_number_en_cardinal = spellout_number(locale!("en"), modifier).unwrap();
        assert_eq!(spellout_number_en_cardinal(100).unwrap(), "One Hundred");
    }

    #[test]
    fn test_spellout_number_143() {
        let modifier = NumberModifier::new(NumberType::Cardinal, Case::Title, "".to_string());
        let spellout_number_en_cardinal = spellout_number(locale!("en"), modifier).unwrap();
        assert_eq!(
            spellout_number_en_cardinal(143).unwrap(),
            "One Hundred and Forty-Three"
        );
    }
}
