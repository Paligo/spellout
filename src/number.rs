use icu_locid::{locale, Locale};

enum NumberType {
    Cardinal,
}

struct Modifier {}

impl Modifier {
    fn new(number_type: NumberType, format: &str) -> Self {
        Modifier {}
    }
}
struct Spellout {}

fn spellout(locale: Locale) -> Spellout {
    Spellout {}
}

impl Spellout {
    fn number(&self, num: u64, modifier: Modifier) -> String {
        "three".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spellout_number() {
        let spellout_en = spellout(locale!("en"));
        let modifier = Modifier::new(NumberType::Cardinal, "stuff");
        assert_eq!(spellout_en.number(3, modifier), "three");
        // assert_eq!(spellout_number(3, modifier, locale!("en")), "three");
    }
}
