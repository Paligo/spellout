use core::num;
use std::f32::consts::E;

use icu_locid::{langid, locale, Locale};

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
    let langid_en = langid!("en");
    let langid_sv = langid!("sv");

    if locale.id == langid_en {
        match modifier.number_type {
            NumberType::Cardinal => Ok(number_en_cardinal as fn(u64) -> Result<String, Error>),
            NumberType::Ordinal => Ok(number_en_ordinal as fn(u64) -> Result<String, Error>),
        }
    } else if locale.id == langid_sv {
        match modifier.number_type {
            NumberType::Cardinal => Ok(number_sv_cardinal as fn(u64) -> Result<String, Error>),
            _ => Err(Error::UnsupportedNumberType),
        }
    } else {
        Err(Error::UnsupportedLocale)
    }
}

fn number_en_cardinal(num: u64) -> Result<String, Error> {
    match num {
        2 => Ok("two".to_string()),
        3 => Ok("three".to_string()),
        _ => Err(Error::NumberOutOfRange),
    }
}

fn number_en_ordinal(num: u64) -> Result<String, Error> {
    match num {
        2 => Ok("second".to_string()),
        _ => Err(Error::NumberOutOfRange),
    }
}

fn number_sv_cardinal(num: u64) -> Result<String, Error> {
    match num {
        2 => Ok("två".to_string()),
        3 => Ok("tre".to_string()),
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

    #[test]
    fn test_spellout_number_three_cardinal_swedish() {
        let modifier = NumberModifier::new(NumberType::Cardinal, "stuff");
        let spellout_number_en_cardinal = spellout_number(locale!("sv"), modifier).unwrap();
        assert_eq!(spellout_number_en_cardinal(3).unwrap(), "tre");
    }
}
