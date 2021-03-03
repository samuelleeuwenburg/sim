extern crate ncurses;

use std::sync::mpsc;
use crate::state;
use crate::state::State;
use ncurses::*;

pub struct WindowState {
    height: i32,
    width: i32,
}

pub fn setup() -> WindowState {
    initscr();
    keypad(stdscr(), true);
    noecho();
    raw();
    set_escdelay(0);

    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    nodelay(stdscr(), true);

    let mut state = WindowState {
	height: 0,
	width: 0,
    };

    getmaxyx(stdscr(), &mut state.height, &mut state.width);

    state
}

pub fn get_input(state: &State) -> Vec<i32> {
    let mut keys = vec![];

    let mut key = getch();
    while key != -1 {
	keys.push(key);
	key = getch();
    }

    keys
}

pub fn draw(window: &WindowState, state: &State, input: &Vec<i32>) {
    clear();

    let (x, y) = state.cursor_pos;

    mvprintw(y, x, "X");

    draw_input_mode(window, input, &state.mode);

    refresh();
}

fn draw_input_mode(window: &WindowState, input: &Vec<i32>, mode: &state::Mode) {
    let readable: String = input.iter()
	.map(|&c| c as u8 as char)
	.into_iter()
	.collect();

    match mode {
	state::Mode::Normal => mvprintw(window.height - 1, 0, &format!("normal {}", readable)),
	state::Mode::Input => mvprintw(window.height - 1, 0, &format!(":{}", readable)),
    };
}

pub fn quit() {
    endwin();
}
