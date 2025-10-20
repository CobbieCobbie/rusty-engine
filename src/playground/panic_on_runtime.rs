#![allow(dead_code)]
pub fn panic_u8_runtime() {
    let x: u8 = 1;
    for i in 0..=255 {
        let _y = x + i; // This will panic when i is 255
    }
}
