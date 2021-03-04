extern crate ncurses;

pub mod state;

use crate::state::{State, Mode};
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

pub fn draw(window: &WindowState, state: &State, input_state: &state::InputState) {
    clear();

    let (x, y) = state.cursor_pos;

    mvprintw(y, x, "X");

    draw_user_message(window, input_state);
    draw_input_mode(window, input_state, &state.mode);

    refresh();
}

fn draw_user_message(window: &WindowState, input_state: &state::InputState) {
    if let Some(msg) = &input_state.user_message {
	let mut message = String::from("> ");
	message.push_str(&msg);
	mvprintw(window.height - 2, 0, &message);
    }
}

fn draw_input_mode(window: &WindowState, input_state: &state::InputState, mode: &Mode) {
    let readable: String = input_state.input_buffer.iter().map(|&c| c as u8 as char).collect();

    mvprintw(0, 0, &format!("input: {:?}", input_state.input_buffer));

    match mode {
	Mode::Normal => mvprintw(window.height - 1, 0, &format!("normal {}", readable)),
	Mode::Input => mvprintw(window.height - 1, 0, &format!(":{}", readable)),
    };
}

pub fn quit() {
    endwin();
}
