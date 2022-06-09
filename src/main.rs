use std::{thread, time::Duration, sync::mpsc};

mod core;
fn main() {
    let (sender, receiver)= mpsc::channel();

    let handle = thread::spawn(move || {
        sender.send(1).unwrap();
        thread::sleep(Duration::from_millis(500));
    });
    
    println!("receive {}", receiver.recv().unwrap());
    handle.join().unwrap();

}
