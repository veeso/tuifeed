//! # Serialization
//!
//! Configuration seralizer

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
use serde::de::DeserializeOwned;
use std::io::Read;
use thiserror::Error;

/// ## SerializerError
///
/// Contains the error for serializer/deserializer
#[derive(Debug)]
pub struct SerializerError {
    kind: SerializerErrorKind,
    msg: Option<String>,
}

/// ## SerializerErrorKind
///
/// Describes the kind of error for the serializer/deserializer
#[derive(Error, Debug)]
pub enum SerializerErrorKind {
    #[error("Operation failed")]
    Generic,
    #[error("IO error")]
    Io,
    #[error("Serialization error")]
    Serialization,
    #[error("Syntax error")]
    Syntax,
}

impl SerializerError {
    /// ### new
    ///
    /// Instantiate a new `SerializerError`
    pub fn new(kind: SerializerErrorKind) -> SerializerError {
        SerializerError { kind, msg: None }
    }

    /// ### new_ex
    ///
    /// Instantiates a new `SerializerError` with description message
    pub fn new_ex(kind: SerializerErrorKind, msg: String) -> SerializerError {
        SerializerError {
            kind,
            msg: Some(msg),
        }
    }
}

impl std::fmt::Display for SerializerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.msg {
            Some(msg) => write!(f, "{} ({})", self.kind, msg),
            None => write!(f, "{}", self.kind),
        }
    }
}

/// ### deserialize
///
/// Read data from readable and deserialize its content as TOML
pub fn deserialize<S>(mut readable: Box<dyn Read>) -> Result<S, SerializerError>
where
    S: DeserializeOwned + Sized + std::fmt::Debug,
{
    // Read file content
    let mut data: String = String::new();
    if let Err(err) = readable.read_to_string(&mut data) {
        return Err(SerializerError::new_ex(
            SerializerErrorKind::Io,
            err.to_string(),
        ));
    }
    // Deserialize
    match toml::de::from_str(data.as_str()) {
        Ok(deserialized) => Ok(deserialized),
        Err(err) => Err(SerializerError::new_ex(
            SerializerErrorKind::Syntax,
            err.to_string(),
        )),
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::config::Config;

    use pretty_assertions::assert_eq;
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn should_create_serialization_errors() {
        let error: SerializerError = SerializerError::new(SerializerErrorKind::Syntax);
        assert!(error.msg.is_none());
        assert_eq!(format!("{}", error), String::from("Syntax error"));
        let error: SerializerError =
            SerializerError::new_ex(SerializerErrorKind::Syntax, String::from("bad syntax"));
        assert!(error.msg.is_some());
        assert_eq!(
            format!("{}", error),
            String::from("Syntax error (bad syntax)")
        );
        // Fmt
        assert_eq!(
            format!("{}", SerializerError::new(SerializerErrorKind::Generic)),
            String::from("Operation failed")
        );
        assert_eq!(
            format!("{}", SerializerError::new(SerializerErrorKind::Io)),
            String::from("IO error")
        );
        assert_eq!(
            format!(
                "{}",
                SerializerError::new(SerializerErrorKind::Serialization)
            ),
            String::from("Serialization error")
        );
    }

    #[test]
    fn should_deserialize_config() {
        let config = create_good_toml_config();
        let reader = File::open(config.path()).expect("Could not open TOML file");
        let config: Config = deserialize(Box::new(reader)).ok().unwrap();
        assert_eq!(config.sources.len(), 2);
        assert_eq!(
            config.sources.get("nytimes").unwrap().as_str(),
            "https://rss.nytimes.com/services/xml/rss/nyt/World.xml"
        );
        assert_eq!(
            config.sources.get("lefigaro").unwrap().as_str(),
            "https://www.lefigaro.fr/rss/figaro_actualites.xml"
        );
    }

    #[test]
    fn should_fail_config_deserialization() {
        let config = create_bad_toml_config();
        let reader = File::open(config.path()).expect("Could not open TOML file");
        assert!(deserialize::<Config>(Box::new(reader)).is_err());
    }

    fn create_good_toml_config() -> tempfile::NamedTempFile {
        let mut tmpfile = tempfile::NamedTempFile::new().unwrap();
        let file_content: &str = r##"
        [sources]
        nytimes = "https://rss.nytimes.com/services/xml/rss/nyt/World.xml"
        lefigaro = "https://www.lefigaro.fr/rss/figaro_actualites.xml"
        "##;
        tmpfile.write_all(file_content.as_bytes()).unwrap();
        tmpfile
    }

    fn create_bad_toml_config() -> tempfile::NamedTempFile {
        let mut tmpfile = tempfile::NamedTempFile::new().unwrap();
        let file_content: &str = r##"
        [sources]
        nytimes = "https://rss.nytimes.com/services/xml/rss/nyt/World.xml"
        lefigaro
        "##;
        tmpfile.write_all(file_content.as_bytes()).unwrap();
        tmpfile
    }
}
