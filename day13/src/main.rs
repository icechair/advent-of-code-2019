extern crate intcode;
extern crate termion;
use intcode::spawn;
use std::cmp::Ordering;
use std::env;
use std::fs::read_to_string;
use std::io::{stdout, Write};
use std::sync::mpsc::TryRecvError;
use std::thread;
use std::time::Duration;
use termion::{
    event::Key,
    input::TermRead,
    raw::{IntoRawMode, RawTerminal},
};
const tiles: [char; 5] = [' ', '#', '+', '=', '@'];
#[derive(Debug, Copy, Clone)]
struct Point(i64, i64);

fn run(pdata: String) -> i64 {
    //rustbox.print
    let (tx, rx, _) = spawn(pdata, None);

    let mut blocks: i64 = 0;
    let mut in_buf: Vec<String> = Vec::with_capacity(3);
    let mut stdin = termion::async_stdin().keys();
    let mut stdout = stdout()
        .into_raw_mode()
        .expect("main: cannot enter raw mode");
    write!(
        stdout,
        "{}{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1)
    )
    .unwrap();

    let mut paddle = Point(0, 0);
    let mut prev_ball = Point(0, 0);
    let mut ball = Point(0, 0);

    let mut segment: i64 = 0;

    loop {
        match stdin.next() {
            Some(Ok(key)) => match key {
                Key::Char('a') => tx.send(format!("{}", -1)).unwrap(),
                Key::Char('d') => match tx.send(format!("{}", 1)) {
                    Ok(_) => {}
                    Err(e) => panic!("{}", e),
                },
                Key::Char('s') => match tx.send(format!("{}", 0)) {
                    Ok(_) => {}
                    Err(e) => panic!("{}", e),
                },
                Key::Char('q') => break,
                _ => {}
            },
            Some(Err(e)) => panic!("{}", e),
            None => {}
        };
        match rx.try_recv() {
            Ok(v) => in_buf.push(v),
            Err(e) => match e {
                TryRecvError::Empty => {
                    stdout.lock().flush().unwrap();
                    /*                     thread::sleep(Duration::from_millis(500));
                    let dif = ball.0 - paddle.0;
                    if dif < -1 {
                        match tx.send(format!("{}", 1)) {
                            Ok(_) => {}
                            Err(e) => panic!("{}", e),
                        }
                    } else if dif > 0 {
                        match tx.send(format!("{}", -1)) {
                            Ok(_) => {}
                            Err(e) => panic!("{}", e),
                        }
                    } else {
                        match tx.send(format!("{}", 0)) {
                            Ok(_) => {}
                            Err(e) => panic!("{}", e),
                        }
                    } */
                    prev_ball = ball;
                }
                TryRecvError::Disconnected => break,
            },
        }

        if in_buf.len() == 3 {
            let data: i64 = in_buf
                .pop()
                .expect("in_buf: contain id")
                .parse()
                .expect("inputbuffer: cannot parse x");
            let y: i64 = in_buf
                .pop()
                .expect("in_buf: contain y")
                .parse()
                .expect("inputbuffer: cannot parse x");
            let x: i64 = in_buf
                .pop()
                .expect("in_buf: contain x")
                .parse()
                .expect("inputbuffer: cannot parse x");
            if data == 2 {
                blocks += 1;
            }

            if data == 3 {
                paddle = Point(x, y);
                write!(
                    stdout,
                    "{}{}",
                    termion::cursor::Goto(1 as u16, 1 as u16),
                    format!(
                        "Score: {}, lB {:?}, B {:?}, P {:?}, {}",
                        segment,
                        prev_ball,
                        ball,
                        paddle,
                        ball.0 - paddle.0
                    )
                )
                .unwrap();
            }

            if data == 4 {
                ball = Point(x, y);
                write!(
                    stdout,
                    "{}{}",
                    termion::cursor::Goto(1 as u16, 1 as u16),
                    format!(
                        "Score: {}, lB {:?}, B {:?}, P {:?}, {}",
                        segment,
                        prev_ball,
                        ball,
                        paddle,
                        ball.0 - paddle.0
                    )
                )
                .unwrap();
            }

            in_buf.clear();

            if x == -1 {
                segment = data;
                write!(
                    stdout,
                    "{}{}",
                    termion::cursor::Goto(1 as u16, 1 as u16),
                    format!(
                        "Score: {}, lB {:?}, B {:?}, P {:?}, {}",
                        segment,
                        prev_ball,
                        ball,
                        paddle,
                        ball.0 - paddle.0
                    )
                )
                .unwrap();
            } else {
                write!(
                    stdout,
                    "{}{}",
                    termion::cursor::Goto((x + 1) as u16, (y + 2) as u16),
                    tiles[data as usize]
                )
                .unwrap();
            }
        }
        //thread::sleep(Duration::from_millis(50));
    }
    write!(stdout, "\r{}{}", termion::clear::All, termion::cursor::Show).unwrap();
    blocks
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let pdata = read_to_string(filename).expect("cannot read file to string");
    println!("{}", run(pdata));
}
