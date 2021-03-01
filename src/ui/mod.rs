extern crate ncurses;

use std::sync::mpsc;
use crate::state::{State, Message, handle_input};
use ncurses::*;

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

pub fn input(state: &State, tx: &mpsc::Sender<Message>) {
    let mut keys = vec![];

    let mut key = getch();
    while key != -1 {
	keys.push(key);
	key = getch();
    }

    if let Some(msg) = handle_input(keys.as_slice()) {
	tx.send(msg).unwrap();
    }
}

pub fn draw(state: &State) {
    clear();

    refresh();
}
