pub mod one {
    pub fn add_one (n: usize) -> usize {
        n + 1
    }
}

#[cfg(feature = "add_two")]
pub mod two;