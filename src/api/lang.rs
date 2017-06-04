//! API language enum.

use std::fmt;

/// Languages supported by the API.
///
/// Usage:
///
/// - Add the `Accept-Language` HTTP header to the request with the value `<language>`.
/// - Add the query parameter `?lang=<language>` to your request URL.
#[derive(
    Debug, Clone,
    PartialEq, Eq, PartialOrd, Ord
)]
pub enum Lang {
    /// English (default language)
    EN,
    /// Spanish
    ES,
    /// Dutch
    DE,
    /// French
    FR,
    /// Korean
    KO,
    /// Chinese
    ZH
}

impl fmt::Display for Lang {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match *self {
            Lang::EN => "en",
            Lang::ES => "es",
            Lang::DE => "de",
            Lang::FR => "fr",
            Lang::KO => "ko",
            Lang::ZH => "zh"
        })
    }
}