pub mod generate;
pub mod grid;

pub use grid::*;

pub trait ExpectPrefixExt {
    fn expect_prefix(self, prefix: &str) -> Self;
}

impl ExpectPrefixExt for &str {
    fn expect_prefix(self, prefix: &str) -> Self {
        self.strip_prefix(prefix).expect(prefix)
    }
}
