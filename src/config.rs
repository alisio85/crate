pub const fn require(condition: bool, _message: &str) {
    if !condition {
        panic!();
    }
}
