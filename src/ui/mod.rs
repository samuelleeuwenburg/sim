extern crate ncurses;

use ncurses::*;

enum Key {
    Left,
    Right,
    Up,
    Down,
}

type InputMap = Vec<Key>;

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

pub fn input() {
    let mut keys = vec![];

    let mut key = getch();
    while key != -1 {
	keys.push(key);
	key = getch();
    }
}

pub fn draw() {
    clear();

    box_(stdscr(), 0, 0);

    refresh();
}
