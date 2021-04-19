use crate::utils::quit;
use mpd::{
    Client,
    State::{Pause, Play, Stop},
};

pub struct Song {
    pub title: String,
    pub elapsed: i64,
    pub total: i64,
    pub state: String,
    pub is_change: bool,
}

pub struct Model {
    client: mpd::Client,
    current_song: String,
}

impl Model {
    pub fn create_conn(host: &str, port: &str) -> Model {
        let host_string = match host {
            "localhost" => "127.0.0.1",
            _ => host,
        };
        let client = Client::connect(format!("{}:{}", host_string, port));
        if client.is_err() {
            quit();
        }
        Model {
            client: client.unwrap(),
            current_song: String::new(),
        }
    }
    pub fn get_current(&mut self) -> Song {
        let current = self.client.currentsong();
        if current.is_err() {
            quit();
        }
        let current = current.unwrap();
        if current.is_none() {
            return Song {
                title: "".to_string(),
                elapsed: 0,
                total: 0,
                state: "".to_string(),
                is_change: true,
            };
        }
        let current = current.unwrap();
        let title = match current.title {
            Some(x) => x,
            None => String::new(),
        };
        let status = self.client.status();
        if status.is_err() {
            quit();
        }
        let status = status.unwrap();
        let elapsed = match status.elapsed {
            Some(x) => x.num_seconds(),
            None => 0,
        };
        let total = match status.duration {
            Some(x) => x.num_seconds(),
            None => 0,
        };
        let state = match status.state {
            Play => "Playing",
            Pause => "Paused",
            Stop => "Stopped",
        };
        let is_change = if title == self.current_song {
            false
        } else {
            true
        };
        Song {
            title,
            elapsed,
            total,
            state: state.to_string(),
            is_change,
        }
    }
    pub fn get_volume(&mut self) -> i8 {
        let status = self.client.status();
        if status.is_err() {
            quit();
        }
        status.unwrap().volume
    }
    pub fn queue(&mut self) -> Vec<mpd::Song> {
        match self.client.queue() {
            Ok(x) => x,
            Err(_) => vec![],
        }
    }
}
