//! Convert ASCII letters to English phonetic words or Japanese katakana names.

use std::fmt;

const EN_ALPHABET: [&str; 26] = [
    "Alfa", "Bravo", "Charlie", "Delta", "Echo", "Foxtrot", "Golf", "Hotel", "India", "Juliett",
    "Kilo", "Lima", "Mike", "November", "Oscar", "Papa", "Quebec", "Romeo", "Sierra", "Tango",
    "Uniform", "Victor", "Whiskey", "Xray", "Yankee", "Zulu",
];
const JA_ALPHABET: [&str; 26] = [
    "エイ",
    "ビー",
    "シー",
    "ディー",
    "イー",
    "エフ",
    "ジー",
    "エイチ",
    "アイ",
    "ジェイ",
    "ケイ",
    "エル",
    "エム",
    "エヌ",
    "オー",
    "ピー",
    "キュー",
    "アール",
    "エス",
    "ティー",
    "ユー",
    "ヴィー",
    "ダブリュー",
    "エクス",
    "ワイ",
    "ゼット",
];
const EN_NUMBERS: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];
const JA_NUMBERS: [&str; 10] = [
    "ゼロ",
    "イチ",
    "ニイ",
    "サン",
    "ヨン",
    "ゴウ",
    "ロク",
    "シチ",
    "ハチ",
    "キュウ",
];

/// Supported output language.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Language {
    English,
    Japanese,
}

impl TryFrom<&str> for Language {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "en" => Ok(Self::English),
            "ja" => Ok(Self::Japanese),
            other => Err(Error::UnsupportedLanguage(other.to_owned())),
        }
    }
}

/// Error returned for an unsupported language identifier.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Error {
    UnsupportedLanguage(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedLanguage(lang) => write!(f, "language {lang:?} is not supported"),
        }
    }
}

impl std::error::Error for Error {}

/// Conversion settings. Start with [`Options::english`] or [`Options::japanese`]
/// and override the desired fields.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Options {
    pub delimiter: String,
    pub sign: String,
    pub num: bool,
}

impl Options {
    pub fn english() -> Self {
        Self {
            delimiter: "-".into(),
            sign: "(CAPS)".into(),
            num: false,
        }
    }

    pub fn japanese() -> Self {
        Self {
            delimiter: "・".into(),
            sign: "（大文字）".into(),
            num: false,
        }
    }
}

/// A reusable converter.
#[derive(Clone, Debug)]
pub struct A2pcej {
    language: Language,
    options: Options,
}

impl A2pcej {
    pub fn new(language: Language, options: Options) -> Self {
        Self { language, options }
    }

    pub fn from_language(lang: &str) -> Result<Self, Error> {
        let language = Language::try_from(lang)?;
        let options = match language {
            Language::English => Options::english(),
            Language::Japanese => Options::japanese(),
        };
        Ok(Self::new(language, options))
    }

    /// Converts each Unicode scalar value, translating only ASCII letters and digits.
    pub fn convert(&self, letters: &str) -> String {
        letters
            .chars()
            .map(|letter| self.convert_char(letter))
            .collect::<Vec<_>>()
            .join(&self.options.delimiter)
    }

    fn convert_char(&self, letter: char) -> String {
        let (alphabet, numbers): (&[&str; 26], &[&str; 10]) = match self.language {
            Language::English => (&EN_ALPHABET, &EN_NUMBERS),
            Language::Japanese => (&JA_ALPHABET, &JA_NUMBERS),
        };
        match letter {
            'A'..='Z' => format!(
                "{}{}",
                alphabet[letter as usize - 'A' as usize],
                self.options.sign
            ),
            'a'..='z' => alphabet[letter as usize - 'a' as usize].to_owned(),
            '0'..='9' if self.options.num => numbers[letter as usize - '0' as usize].to_owned(),
            _ => letter.to_string(),
        }
    }
}

/// Converts with the English defaults.
pub fn conv_al(letters: &str) -> String {
    A2pcej::new(Language::English, Options::english()).convert(letters)
}

/// Converts with the Japanese defaults.
pub fn conv_ak(letters: &str) -> String {
    A2pcej::new(Language::Japanese, Options::japanese()).convert(letters)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convenience_functions_match_python() {
        assert_eq!(
            conv_al("Examples002"),
            "Echo(CAPS)-Xray-Alfa-Mike-Papa-Lima-Echo-Sierra-0-0-2"
        );
        assert_eq!(
            conv_ak("HogE"),
            "エイチ（大文字）・オー・ジー・イー（大文字）"
        );
    }

    #[test]
    fn options_and_numbers() {
        let converter = A2pcej::new(
            Language::English,
            Options {
                delimiter: ", ".into(),
                sign: "(CAPITAL)".into(),
                num: true,
            },
        );
        assert_eq!(converter.convert("A04#"), "Alfa(CAPITAL), zero, four, #");
    }

    #[test]
    fn unicode_is_preserved_by_character() {
        let converter = A2pcej::new(Language::Japanese, Options::japanese());
        assert_eq!(converter.convert("aあ"), "エイ・あ");
    }

    #[test]
    fn unsupported_language_is_an_error() {
        assert!(matches!(
            A2pcej::from_language("fr"),
            Err(Error::UnsupportedLanguage(_))
        ));
    }
}
