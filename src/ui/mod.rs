extern crate ncurses;

use std::sync::mpsc;
use ncurses::*;

pub enum Message {
    AddTrack(String),
}

pub struct InputView {
    pub message: String,
    pub input: Option<String>,
}

pub fn setup() {
    initscr();
    keypad(stdscr(), true);
    noecho();

    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    nodelay(stdscr(), true);

    let mut window_height = 0;
    let mut window_width = 0;
    getmaxyx(stdscr(), &mut window_height, &mut window_width);
}

pub fn input(tx: mpsc::Sender<Message>) {
    let mut keys = vec![];

    let mut key = getch();
    while key != -1 {
	keys.push(key);
	key = getch();
    }

    match keys.as_slice() {
	// A
	&[65] => tx.send(Message::AddTrack("./test_files/f#_warm.wav".to_owned())).unwrap(),
	_ => (),
    }
}

pub fn draw(state: &InputView) {
    clear();

    draw_input_view(state);

    refresh();
}

pub fn draw_input_view(data: &InputView) {
    box_(stdscr(), 0, 0);
    printw(&data.message);
}
