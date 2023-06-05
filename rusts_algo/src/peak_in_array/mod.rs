
pub fn exec(n:&[i32]) -> u32 {
    let mut max_streak:u32 = 1;
    let mut state = State::TBD;
    let mut next_state = State::TBD;
    let mut streak = 1;

    let len = n.len();
    if len == 1 {
        return max_streak;
    }

    for i in 1..len {
        // set next state
        next_state = if n[i - 1] < n[i] {
            State::UPWARD
        } else if n[i - 1] == n[i] {
            State::PLATEAU
        } else {
            State::DOWNWARD
        };
        match (state, next_state) {
            (State::TBD, State::PLATEAU | State::DOWNWARD) => (),
            (State::TBD, State::UPWARD) => {
                state = State::UPWARD;
                streak = 1;
            },
            (State::PLATEAU, State::DOWNWARD | State::PLATEAU) => (),
            (State::PLATEAU, State::UPWARD) => {
                state = State::UPWARD;
                streak = 1;
            },
            (State::UPWARD, State::UPWARD | State::DOWNWARD) => {
                streak += 1;
                state = next_state;
            },
            (State::UPWARD, State::PLATEAU) => {
                max_streak = max_streak.max(streak);
                state = next_state;
            },
            (State::DOWNWARD, State::DOWNWARD) => streak += 1,
            (State::DOWNWARD, State::UPWARD | State::PLATEAU) => {
                max_streak = max_streak.max(streak);
                state = next_state;
            },
            _ => (),
        }
    }

    println!("input - {:?}, max_streak - {max_streak}", n);
    max_streak
}

#[derive(Clone, Copy)]
enum State {
    TBD,
    UPWARD,
    DOWNWARD,
    PLATEAU,
}

pub fn run() {
    let v = [1, 2, 3, 3, 4, 0, 10, 6, 5, -1, -3, 2, 3];
    exec(&v);
}
