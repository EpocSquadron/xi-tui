use std::thread;
use std::sync::mpsc;
use std::io::stdin;
use termion;
use termion::input::TermRead;

pub struct Input {
    pub tx: mpsc::Sender<termion::event::Event>,
    pub rx: mpsc::Receiver<termion::event::Event>,
}

impl Input {
    pub fn new() -> Input {
        let (tx, rx) = mpsc::channel();
        Input {
            tx: tx,
            rx: rx,
        }
    }

    pub fn run(&mut self) {
        let tx = self.tx.clone();
        thread::spawn(move || {
            for event_res in stdin().events() {
                match event_res {
                    Ok(event) => {
                        tx.send(event).unwrap();
                    },
                    Err(err) => {
                        error!("{:?}", err);
                    }
                }
            }
        });
    }
}

impl Default for Input {
	fn default() -> Self {
		Self::new()
	}
}
