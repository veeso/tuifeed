//! # Strings
//!
//! Strings helpers

mod lookup;

use lazy_regex::{Lazy, Regex};
use unicode_truncate::UnicodeTruncateStr;

static HTML_TAG_REGEX: Lazy<Regex> = lazy_regex!(r"<[^>]+>");
/**
 * Matches HTML entities in string
 *
 * - group 2: decimal (maybe)
 * - group 3: literal (e.g. amp, gt, ...) (maybe)
 */
static HTML_ENTITIES_REGEX: Lazy<Regex> = lazy_regex!(r"&(#([0-9]+))?([a-z]+)?;");
static REPEATED_NEWLINES_REGEX: Lazy<Regex> = lazy_regex!(r"(\r?\n|\r)\d*(\r?\n|\r)");

/// ### elide_string_at
///
/// Elide string at `len` and append `
pub fn elide_string_at(s: &str, len: usize) -> String {
    if s.len() < len {
        s.to_string()
    } else {
        format!("{}…", s.unicode_truncate(len - 1).0)
    }
}

/// ### replace_multiple_newlines
///
/// Remove repeated newlines in string and replace them with `with`
pub fn replace_multiple_newlines(s: &str, with: &str) -> String {
    REPEATED_NEWLINES_REGEX.replace_all(s, with).to_string()
}

/// strip_html
///
/// Strip html tags and entities from string
pub fn strip_html(s: &str) -> String {
    let mut escaped = HTML_TAG_REGEX.replace_all(s, "").to_string();
    let copy = escaped.clone();
    for group in HTML_ENTITIES_REGEX.captures_iter(copy.as_str()) {
        if let Some(mtch) = group.get(2) {
            // Convert mtch to u32
            let replace_with = match mtch.as_str().parse::<u32>() {
                Err(_) => '�',
                Ok(val) => char::from_u32(val).unwrap_or('�'),
            };
            // Get char from decimal
            escaped = escaped.replace(&group[0], replace_with.to_string().as_str());
        } else if let Some(mtch) = group.get(3) {
            let replace_with = lookup::HTML_ENTITIES_TABLE
                .iter()
                .find(|(repr, _)| *repr == mtch.as_str())
                .map(|(_, code)| code)
                .unwrap_or(&"�");
            escaped = escaped.replace(&group[0], replace_with);
        }
    }
    escaped
}

#[cfg(test)]
mod test {

    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn should_elide_string() {
        assert_eq!(
            elide_string_at("Lorem ipsum dolores", 10).as_str(),
            "Lorem ips…"
        );
    }

    #[test]
    fn should_not_elide_string() {
        assert_eq!(elide_string_at("Hello world!", 24).as_str(), "Hello world!");
    }

    #[test]
    fn should_elide_without_panicking() {
        assert_eq!(
            elide_string_at("`Milano, la sfida dei «no pass»: arrivano in piazza Duomo. Le forze dell’ordine bloccano il corteo`", 30).as_str(),
            "`Milano, la sfida dei «no pas…"
        )
    }

    #[test]
    fn should_strip_html() {
        assert_eq!(
            strip_html(
                r#"<p><img src="https://images2.corriereobjects.it/methode_image/2021/11/09/Cultura/Foto%20Cltura%20-%20Trattate/Il%20salvataggio%20delle%20vacche%20bis-kWoC-U3300981161016RfG-656x492@Corriere-Web-Sezioni.jpg" title="Polesine, novembre 1951,settant’anni fa l’alluvione che travolse il Veneto" alt="Polesine, novembre 1951 />Hello</p> World!"#
            ),
            "Hello World!"
        );
        assert_eq!(
            strip_html(r#"Hello, &lt;World&gt;&#33;"#),
            "Hello, <World>!"
        );
    }

    #[test]
    fn should_replace_multiple_newlines() {
        assert_eq!(
            replace_multiple_newlines(
                r"Hello, world!


How are you?
I'm fine,
thanks!",
                "\n"
            )
            .as_str(),
            r"Hello, world!

How are you?
I'm fine,
thanks!"
        );
    }
}
