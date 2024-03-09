use aho_corasick::AhoCorasick;
use std::sync::LazyLock;

const PATTERNS: &[&str; 11] = &["˸", "：", "⁄", "［", "］", "＠", "＂", "․", "‚", "≻", "＃"];
const REPLACE_WITH: &[&str; 11] = &[":", ":", "/", "[", "]", "@", "\"", ".", ",", ">", "#"];

static AC: LazyLock<AhoCorasick> = LazyLock::new(|| AhoCorasick::new(PATTERNS).unwrap());

pub(crate) trait Unsanitizer {
    fn unsanitize(&self) -> String;
}

impl Unsanitizer for str {
    fn unsanitize(&self) -> String {
        AC.replace_all(self, REPLACE_WITH)
    }
}
