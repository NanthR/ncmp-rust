use crossterm::event::{self, KeyEvent};
use std::{
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

pub enum Event<I> {
    Input(I),
    Tick,
}

pub struct EventFunctions {
    rx: mpsc::Receiver<Event<KeyEvent>>,
}

impl EventFunctions {
    pub fn new(tick_rate: u64) -> EventFunctions {
        let (tx, rx) = mpsc::channel();
        let tick_rate = Duration::from_millis(tick_rate);
        thread::spawn(move || {
            let mut last_tick = Instant::now();
            loop {
                let timeout = tick_rate
                    .checked_sub(last_tick.elapsed())
                    .unwrap_or_else(|| Duration::from_secs(0));
                if event::poll(timeout).expect("polling") {
                    if let event::Event::Key(key) = event::read().expect("Reading") {
                        tx.send(Event::Input(key)).expect("Sending");
                    }
                }
                if last_tick.elapsed() >= tick_rate {
                    if let Ok(_) = tx.send(Event::Tick) {
                        last_tick = Instant::now();
                    }
                }
            }
        });
        EventFunctions { rx }
    }
    pub fn get(&self) -> Result<Event<KeyEvent>, mpsc::RecvError> {
        self.rx.recv()
    }
    // pub fn quit(&self) {
    //     self.tx.send(Event::Input(Key::Char('q')));
    // }
}
