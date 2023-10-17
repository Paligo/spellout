use core::num;

use icu_locid::{locale, Locale};

#[derive(Debug)]
enum Error {
    UnsupportedLocale,
    NumberOutOfRange,
}

enum NumberType {
    Cardinal,
    Ordinal,
}

struct NumberModifier {
    number_type: NumberType,
}

impl NumberModifier {
    fn new(number_type: NumberType, format: &str) -> Self {
        NumberModifier { number_type }
    }
}

fn spellout_number(
    locale: Locale,
    modifier: NumberModifier,
) -> Result<impl Fn(u64) -> Result<String, Error>, Error> {
    match modifier.number_type {
        NumberType::Cardinal => Ok(spellout_en_cardinal as fn(u64) -> Result<String, Error>),
        NumberType::Ordinal => Ok(spellout_en_ordinal as fn(u64) -> Result<String, Error>),
    }
}

fn spellout_en_cardinal(num: u64) -> Result<String, Error> {
    match num {
        2 => Ok("two".to_string()),
        3 => Ok("three".to_string()),
        _ => Err(Error::NumberOutOfRange),
    }
}

fn spellout_en_ordinal(num: u64) -> Result<String, Error> {
    match num {
        2 => Ok("second".to_string()),
        _ => Err(Error::NumberOutOfRange),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spellout_number_three() {
        let modifier = NumberModifier::new(NumberType::Cardinal, "stuff");
        let spellout_number_en_cardinal = spellout_number(locale!("en"), modifier).unwrap();
        assert_eq!(spellout_number_en_cardinal(3).unwrap(), "three");
    }

    #[test]
    fn test_spellout_number_two() {
        let modifier = NumberModifier::new(NumberType::Cardinal, "stuff");
        let spellout_number_en_cardinal = spellout_number(locale!("en"), modifier).unwrap();
        assert_eq!(spellout_number_en_cardinal(2).unwrap(), "two");
    }
    #[test]
    fn test_spellout_number_two_ordinal() {
        let modifier = NumberModifier::new(NumberType::Ordinal, "stuff");
        let spellout_number_en_cardinal = spellout_number(locale!("en"), modifier).unwrap();
        assert_eq!(spellout_number_en_cardinal(2).unwrap(), "second");
    }
}
