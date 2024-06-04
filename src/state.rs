use std::collections::VecDeque;


#[derive(Clone, Debug)]
pub struct State {
    board: Vec<Vec<bool>>,
}

impl State {
    pub fn new() -> Self {
        let line = vec![false; 20];
        State {
            board: vec![line; 20],
        }
    }
    pub fn is_occupied(&mut self, x: usize, y: usize) -> bool {
        if x >= 20 || y >= 20 {
            return false;
        }
        self.board[x][y]
    }

    pub fn occupy(&mut self, x: usize, y: usize) {
        if x >= 20 || y >= 20 {
            return;
        }
        self.board[x][y] = true
    }
}

#[derive(Clone, Debug)]
pub struct Moves {
    pub states: VecDeque<State>,
}

impl Moves {
    pub fn new() -> Self {
        Moves {
            states: VecDeque::new(),
        }
    }
    pub fn add_state(&mut self) {
        self.states.push_back(State::new())
    }
    pub fn drop_state(&mut self) {
        self.states.pop_front();
    }
}
