// Unsigned 8-bit integer 255 gets incremented. Lets panic!

fn panic_u8_runtime() {
    let x: u8 = 1;
    for i in 0..=255 {
        let _y = x + i; // This will panic when i is 255
    }
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
