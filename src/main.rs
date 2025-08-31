use std::io::{self, Read};
use std::sync::mpsc;
use std::thread;

fn main() {
    // Create a channel to signal when user presses a key
    let (tx, rx) = mpsc::channel();

    // Spawn a thread to listen for input
    thread::spawn(move || {
        let mut buf = [0u8; 1];
        let stdin = io::stdin();
        let mut handle = stdin.lock();

        // Wait until user presses any key + ENTER
        handle.read(&mut buf).unwrap();
        tx.send(()).unwrap();
    });
    let mut count: u32 = 0;
    // Main loop keeps running until user presses a key
    'testloop: loop {
        count += 1;
        // Break if input thread sent a message
        if rx.try_recv().is_ok() || count == u32::MAX{
            println!("User pressed a key, breaking loop! Count: {}", count);
            break 'testloop;
        }
    }
}
