mod hybrid;

use std::fs::{File};
use std::io::{BufReader};
use serde::{Deserialize, Serialize};

const OUTPUT_FILE: &str = "r_nice.txt";
const CONFIG_FILE: &str = "patterns.json";

#[derive(Debug, Deserialize, Serialize)]
struct PatternConfig {
    words: Vec<String>,
    same_four: bool,
    same_five: bool,
    #[serde(default)]
    same_six: bool,
}

#[derive(Debug, Clone)]
struct WordPattern {
    text: String,
    case_sensitive: bool,
    length: usize,
    require_readable_prefix: bool, 
}

impl WordPattern {
    fn from_config(word: &str) -> Self {
        let mut text = word;
        let mut require_readable = false;
        let mut case_sensitive = true;

        if text.starts_with('@') {
            require_readable = true;
            text = &text[1..];
        }

        if text.starts_with('*') {
            case_sensitive = false;
            text = &text[1..];
        }

        WordPattern {
            text: text.to_string(),
            case_sensitive,
            length: text.len(),
            require_readable_prefix: require_readable,
        }
    }


    fn is_readable_char(c: u8) -> bool {
        matches!(c,
            b'a' | b'e' | b'i' | b'o' | b'u' |  
            b'A' | b'E' | b'I' | b'O' | b'U' |  
            b'1' | b'2' | b'3' | b'4' | b'5' | b'6' | b'7' | b'8' | b'9' | b'0'  
        )
    }
}

#[derive(Debug, Clone)]
struct PatternChecker {
    words: Vec<WordPattern>,
    same_four: bool,
    same_five: bool,
    same_six: bool,
}

impl PatternChecker {
    fn from_config(config: PatternConfig) -> Self {
        let words = config.words.iter()
            .map(|w| WordPattern::from_config(w))
            .collect();

        PatternChecker {
            words,
            same_four: config.same_four,
            same_five: config.same_five,
            same_six: config.same_six,
        }
    }

    fn load_from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let config: PatternConfig = serde_json::from_reader(reader)?;
        Ok(Self::from_config(config))
    }

    fn check(&self, address: &str) -> Option<String> {
        let bytes = address.as_bytes();
        let len = bytes.len();

        if len < 5 {
            return None;
        }

        // Проверяем слова
        for pattern in &self.words {
            let min_len = if pattern.require_readable_prefix {
                pattern.length + 2  
            } else {
                pattern.length + 1  
            };

            if len < min_len {
                continue;
            }

            let end_slice = &bytes[len - pattern.length..];


            let text_matches = if pattern.case_sensitive {
                end_slice == pattern.text.as_bytes()
            } else {
                let end_upper: Vec<u8> = end_slice.iter()
                    .map(|&b| b.to_ascii_uppercase())
                    .collect();
                let pattern_upper = pattern.text.to_uppercase();
                end_upper == pattern_upper.as_bytes()
            };

            if !text_matches {
                continue;
            }


            if pattern.require_readable_prefix {
                let prefix_char = bytes[len - pattern.length - 1];
                if !WordPattern::is_readable_char(prefix_char) {
                    continue; 
                }
            }


            if pattern.case_sensitive {
                return Some(pattern.text.clone());
            } else {
                return Some(format!("{} (i)", pattern.text));
            }
        }


        if self.same_six && len >= 7 {
            let last6 = &bytes[len-6..];
            if last6[0] == last6[1] && last6[1] == last6[2] &&
               last6[2] == last6[3] && last6[3] == last6[4] && last6[4] == last6[5] {
                let chars = std::str::from_utf8(last6).unwrap_or("??????");
                return Some(format!("6SAME({})", chars));
            }
        }


        if self.same_five && len >= 6 {
            let last5 = &bytes[len-5..];
            if last5[0] == last5[1] && last5[1] == last5[2] &&
               last5[2] == last5[3] && last5[3] == last5[4] {
                let chars = std::str::from_utf8(last5).unwrap_or("?????");
                return Some(format!("5SAME({})", chars));
            }
        }


        if self.same_four && len >= 5 {
            let last4 = &bytes[len-4..];
            if last4[0] == last4[1] && last4[1] == last4[2] && last4[2] == last4[3] {
                let chars = std::str::from_utf8(last4).unwrap_or("????");
                return Some(format!("4SAME({})", chars));
            }
        }

        None
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("═══════════════════════════════════════════════════════");
    println!("TRON VANITY GENERATOR (CPU VERSION)");
    println!("═══════════════════════════════════════════════════════");


    let checker = match PatternChecker::load_from_file(CONFIG_FILE) {
        Ok(c) => {
            println!("✓ Config loaded: {}", CONFIG_FILE);
            println!("  Words: {}", c.words.len());
            for w in &c.words {
                let case_str = if w.case_sensitive { "exact" } else { "ignorecase" };
                let readable_str = if w.require_readable_prefix { ", readable-prefix" } else { "" };
                println!("    - {} ({}, {} chars{})", w.text, case_str, w.length, readable_str);
            }
            println!("  Same chars: 4={} 5={} 6={}", c.same_four, c.same_five, c.same_six);
            c
        },
        Err(e) => {
            eprintln!("✗ Error loading config: {}", e);
            eprintln!("  Using default patterns...");
            PatternChecker::from_config(PatternConfig {
                words: vec!["*USDT".to_string(), "*BOSS".to_string(), "*GOLD".to_string()],
                same_four: true,
                same_five: true,
                same_six: false,
            })
        }
    };

    println!("\n⚡ CPU MODE: Parallel generation using all CPU cores");
    println!("Output: {}", OUTPUT_FILE);
    println!("Press Ctrl+C to stop\n");

    let generator = hybrid::CpuGenerator::new(checker)?;
    generator.run(OUTPUT_FILE)
}
