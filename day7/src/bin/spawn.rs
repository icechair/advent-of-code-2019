use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn spawn(tx: mpsc::Sender<String>, rx: mpsc::Receiver<String>) {
    let ping = rx.recv().expect("child cannot read");
    println!("child received: {}" ,ping);
    thread::sleep(Duration::from_secs(1));
    tx.send(String::from("PONG")).expect("child cannot send");
}

fn main() {
    let (child_tx, parent_rx) = mpsc::channel();
    let (parent_tx, child_rx) = mpsc::channel();
    thread::spawn(move || spawn(child_tx, child_rx));
    parent_tx.send(String::from("PING")).expect("parent cannot send");
    for rec in parent_rx {
        println!("parent received: {}", rec);
    }
}
