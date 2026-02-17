//! # Parseagent
//! Attempts to turn the User-Agent string into something more readable
//! # Example
//! ```
//! use parseagent::Guess;
//! let parsed = Guess::new("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/144.0.0.0 Safari/537.36");
//! assert_eq!(&parsed.to_string(), "Linux Chrome/144.0");
//! ```
//! # Notes on usage
//! This crate is made for simpler logging. If you want to detect browser more robustly,
//! use the Client-Hints API instead
//! # Stability
//! Textual representation of formatted strings is subject to change,
//! and should not be relied upon. Including `parsed.ver`.
//! They were designed to be printed or shown to user

use std::fmt;

#[cfg(test)]
mod tests;

/// Browser engine
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Engine {
    /// Any chromium fork or Chrome on iOS
    Chrome,
    /// Firefox or Firefox on iOS
    Firefox,
    /// Webkit
    Webkit,
    /// Safari
    Safari,
    /// Unknown browser
    Unknown(String),
}

impl Default for Engine {
    fn default() -> Engine {
        Engine::Unknown("".to_string())
    }
}

impl fmt::Display for Engine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Engine::*;
        match self {
            Chrome => f.write_str("Chrome"),
            Firefox => f.write_str("Firefox"),
            Webkit => f.write_str("WebKit"),
            Safari => f.write_str("Safari"),
            Unknown(s) => f.write_str(s),
        }
    }
}

/// Browser OS
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Os {
    /// MS Windows, any version
    Windows,
    /// Macintosh
    Mac,
    /// Linux desktop
    Linux,
    /// Android
    Android,
    /// iOS
    Ios,
    /// Unknown OS
    #[default]
    Unknown,
}

impl Os {
    /// Checks if it is a mobile OS (Android or iOS)
    pub fn is_mobile(&self) -> bool {
        matches!(self, Os::Android | Os::Ios)
    }
}

impl fmt::Display for Os {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Os::*;
        match self {
            Windows => f.write_str("Windows"),
            Mac => f.write_str("Mac"),
            Linux => f.write_str("Linux"),
            Android => f.write_str("Android"),
            Ios => f.write_str("iOS"),
            Unknown => f.write_str("Unknown"),
        }
    }
}

/// Parsed user agent
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct Guess {
    /// Browser engine
    pub engine: Engine,
    /// Browser OS
    pub os: Os,
    /// Engine version, empty string by default
    pub ver: String,
}

fn ver(tok: &str) -> String {
    let ver = tok.split('/').next_back().unwrap();
    let firstdot = ver.find('.').unwrap_or(ver.len());
    if firstdot == ver.len() { return ver.to_string(); }
    let shortver = &ver[firstdot+1..];
    match shortver.find('.') {
        Some(seconddot) => ver[..seconddot+firstdot+1].to_string(),
        None => ver.to_string(),
    }
}

impl Guess {
    /// Tries to parse the given user_agent
    pub fn new(user_agent: &str) -> Guess {
        use {Engine::*, Os::*};

        let mut guess = Guess::default();

        for token in user_agent.split([' ', '(', ')', ';'])
            .filter(|i| !i.is_empty())
        {
            // OS tokens
            if token == "Windows" {
                guess.os = Windows;
            } else if token == "Macintosh" {
                guess.os = Mac;
            } else if token == "Android" || token == "Dalvik" {
                guess.os = Android;
            } else if token == "Linux" && guess.os != Android {
                guess.os = Linux;
            } else if token == "iPhone" || token == "iPad" || token == "iPod" {
                guess.os = Ios;
            }
            // Browser tokens
            if token.starts_with("Chrome/") || token.starts_with("CriOS/") {
                guess.engine = Chrome; guess.ver = ver(token);
            } else if token.starts_with("Firefox/") || token.starts_with("FxiOS/") {
                guess.engine = Firefox; guess.ver = ver(token);
            } else if token.starts_with("AppleWebKit/") {
                if let Mac | Ios = guess.os {
                    guess.engine = Safari;
                } else {
                    guess.engine = Webkit;
                    guess.ver = ver(token);
                }
            } else if token.starts_with("Version/") && guess.engine == Safari {
                guess.ver = ver(token);
            }
        }

        if matches!(guess.engine, Engine::Unknown(_)) {
            // Couldn't guess = set to first segment (Usually, "Mozilla/5.0")
            guess.engine = Engine::Unknown(user_agent.split(' ').next().unwrap().to_string());
        }

        guess
    }
}

impl fmt::Display for Guess {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.os, self.engine)?;
        if !self.ver.is_empty() && !matches!(self.engine, Engine::Unknown(_)) {
            write!(f, "/{}", self.ver)?;
        }
        Ok(())
    }
}
