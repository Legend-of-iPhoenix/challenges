/// Returns a number at least roughly  proportional to the relative frequency of the letter with the
/// given ASCII value
pub fn frequency_of(a: u8) -> f32 {
    // Frequencies from the [citation-needed] table on the Wikipedia page for character frequencies
    // Good enough :)
    match a {
        b'e' => 13.0,
        b't' => 9.6,
        b'a' => 8.2,
        b'o' => 7.8,
        b'i' => 6.9,
        b'n' => 6.7,
        b'h' => 6.2,
        b's' => 6.2,
        b'r' => 5.9,
        b'd' => 4.7,
        b'l' => 4.0,
        b'c' => 2.7,
        b'm' => 2.7,
        b'u' => 2.7,
        b'w' => 2.4,
        b'f' => 2.2,
        b'g' => 2.0,
        b'y' => 2.0,
        b'p' => 1.9,
        b'b' => 1.5,
        b'v' => 0.97,
        b'k' => 0.81,
        b'j' => 0.16,
        b'x' => 0.15,
        b'q' => 0.11,
        b'z' => 0.078,
        // Todo: Maybe give a less strong penalty to other common english characters, namely capital
        // letters and spaces.
        _ => -100.0,
    }
}

pub fn score_string(string: &[u8]) -> f32 {
    string.iter().map(|c| frequency_of(*c)).sum()
}
