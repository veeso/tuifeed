//! # Strings
//!
//! Strings helpers

/**
 * MIT License
 *
 * tuifeed - Copyright (c) 2021 Christian Visintin
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */
mod lookup;

use regex::Regex;
use unicode_truncate::UnicodeTruncateStr;

lazy_static! {
    static ref HTML_TAG_REGEX: Regex = Regex::new(r"<[^>]+>").unwrap();
    /**
     * Matches HTML entities in string
     *
     * - group 2: decimal (maybe)
     * - group 3: literal (e.g. amp, gt, ...) (maybe)
     */
    static ref HTML_ENTITIES_REGEX: Regex = Regex::new(r"&(#([0-9]+))?([a-z]+)?;").unwrap();
    static ref REPEATED_NEWLINES_REGEX: Regex = Regex::new(r"(\r?\n|\r)\d*(\r?\n|\r)").unwrap();
}

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
            let replace_with = match u32::from_str_radix(mtch.as_str(), 10) {
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
