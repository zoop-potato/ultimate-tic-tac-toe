#[derive(Debug)]
pub struct UTTTBoard {
    state: Option<FinishState>,
    board: [TTTBoard; 9],
}

#[derive(Debug)]
pub struct TTTBoard {
    state: Option<FinishState>,
    board: [Option<Player>; 9],
}

#[derive(Debug)]
pub enum FinishState {
    Win(Player),
    Draw,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Player {
    X,
    O,
}

#[derive(Debug)]
pub enum BoardPosition {
    TopLeft,
    TopMiddle,
    TopRight,
    MiddleLeft,
    Center,
    MiddleRight,
    BottomLeft,
    BottomMiddle,
    BottomRight,
}

impl BoardPosition {
    fn to_index(self) -> usize {
        match self {
            BoardPosition::TopLeft => 0,
            BoardPosition::TopMiddle => 1,
            BoardPosition::TopRight => 2,
            BoardPosition::MiddleLeft => 3,
            BoardPosition::Center => 4,
            BoardPosition::MiddleRight => 5,
            BoardPosition::BottomLeft => 6,
            BoardPosition::BottomMiddle => 7,
            BoardPosition::BottomRight => 8,
        }
    }
}

static WINNING_LINES: [[usize; 3]; 8] = [
    [0, 1, 2],
    [3, 4, 5],
    [6, 7, 8],
    [0, 3, 6],
    [1, 4, 7],
    [2, 5, 8],
    [0, 4, 8],
    [2, 4, 6],
];

enum PlayResult {
    PositionTaken,
    Finish(FinishState),
    Played,
    BoardComplete,
    IndexError,
}

impl TTTBoard {
    pub fn play(&mut self, player: Player, position: BoardPosition) -> PlayResult {
        if self.state.is_some() {
            // Board is already Complete
            return PlayResult::BoardComplete;
        }

        let position_state_option = self.board.get_mut(position.to_index());
        let mut position_state: Option<Player>;
        // unwrap the option
        match position_state_option {
            Some(x) => position_state = *x,
            None => return PlayResult::IndexError,
        };

        match position_state {
            // This postion has already been played in
            Some(_) => return PlayResult::PositionTaken,
            // Succsesfully play at position
            None => position_state = Some(player),
        }

        let finish = self.check_for_finish();
        match finish {
            // The board is full or has been won
            Some(x) => return PlayResult::Finish(x),
            None => {}
        }

        // play succeeded and play is still possible
        return PlayResult::Played;
    }

    pub fn check_for_finish(&self) -> Option<FinishState> {
        // map every index in WINNING_LINES to its value
        let mapped_lines = WINNING_LINES
            .iter()
            .map(|line|
                // Turn an array of indexs into the value of their board position
                line.iter()
                    .map(|index| self.board[*index])
                    .collect::<Vec<Option<Player>>>())
            .collect::<Vec<Vec<Option<Player>>>>();

        // Check if X won
        let x_won = mapped_lines
            .iter()
            .map(|line| line.iter().all(|position| *position == Some(Player::X)))
            .any(|line_win| line_win == true);
        if x_won { return Some(FinishState::Win(Player::X));}

        // Check if O won
        let o_won = mapped_lines
            .iter()
            .map(|line| line.iter().all(|position| *position == Some(Player::O)))
            .any(|line_win| line_win == true);
        if o_won { return Some(FinishState::Win(Player::O));}

        // Check for draw
        let full = !self
            .board
            .iter()
            .any(|position| *position == None);
        match full {
            // the board is full, therefore draw
            true => return Some(FinishState::Draw),
            // no player has won and spaces are open
            false => return None,
        }
    }

    pub fn new() -> Self {
        Self {
            state: None,
            board: [None; 9],
        }
    }
}
