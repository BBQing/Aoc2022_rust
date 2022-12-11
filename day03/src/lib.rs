pub const LETTERS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

pub fn priority(chr: &char) -> i32 {

    if let Some(x) = LETTERS.find(*chr) {
        x as i32 + 1
    }
    else {0}
}