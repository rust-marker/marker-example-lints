//@no-rustfix
#[allow(clippy::manual_is_ascii_check)]

fn main() {
    let x = 0;
    let c = 'x';

    // Lint
    let _ = (b'a'..b'z').contains(&x);
    let _ = (b'A'..b'Z').contains(&x);
    let _ = (b'0'..b'9').contains(&x);
    let _ = ('a'..'z').contains(&c);
    let _ = ('A'..'Z').contains(&c);
    let _ = ('0'..'9').contains(&c);

    // Don't lint (inclusive)
    let _ = (b'a'..=b'z').contains(&x);
    let _ = (b'A'..=b'Z').contains(&x);
    let _ = (b'0'..=b'9').contains(&x);
    let _ = ('a'..='z').contains(&c);
    let _ = ('A'..='Z').contains(&c);
    let _ = ('0'..='9').contains(&c);

    let b: i32 = 32;

    // Don't lint (something else)
    let _ = (b'a'..).contains(&x);
    let _ = (..=b'Z').contains(&x);
    let _ = (256..7272).contains(&b);
    let _ = (0..1).contains(&x);
    let _ = (0..9).contains(&x);
}
