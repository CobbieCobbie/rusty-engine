// Unsigned 8-bit integer 255 gets incremented. Lets panic!

fn panic_u8_runtime() {
    let x: u8 = 255;
    x + 1;
}

fn main() {
    println!("This is a overflow test");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "attempt to add with overflow")]
    fn test_panic_u8_runtime() {
        panic_u8_runtime();
    }
}
