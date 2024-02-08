#[derive(Copy, Clone)]
enum State {
    Normal,
    Comment,
    Upper,
    Lower
}

fn machine_cycle(state: State, c: char) -> (Option<char>, State) {
    use self::State::*;
    match (state, c) {
        (Normal, '#') => (None, Comment),
        (Normal, '^') => (None, Upper),
        (Normal, '_') => (None, Lower),
        (Normal, other) => (Some(other), Normal),
        (Comment, '#') => (None, Normal),
        (Comment, _) => (None, Comment),
        (Upper, '^') => (None, Normal),
        (Upper, other) => (Some(other.to_ascii_uppercase()), Upper),
        (Lower, '_') => (None, Normal),
        (Lower, other) => (Some(other.to_ascii_lowercase()), Lower)
    }
}

pub fn main_md() {
    let mut state = State::Normal;
    let mut result: String = String::new();
    let inp = "This _Is_ some ^input^#commented out#.";

    for ch in inp.chars() {
        let (out, new_state) = machine_cycle(state, ch);
        if let Some(c) = out {
            result.push(c);
        }
        state = new_state;
    }
    println!("{}", result)
}