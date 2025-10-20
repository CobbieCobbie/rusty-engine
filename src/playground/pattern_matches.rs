use std::io::stdin;
use std::sync::mpsc;

pub fn check_even_odd() {

    let (tx, rx) = mpsc::channel();
    std::thread::spawn(move || {

        let number = stdin()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap();
        tx.send(number).unwrap();
    });

    while let Ok(received) = rx.recv() {

        match received % 2 {
            0 => println!("Even number: {}", received),
            1 => println!("Odd number: {}", received),
            _ => unreachable!(), // This case will never happen
        }

    }
}