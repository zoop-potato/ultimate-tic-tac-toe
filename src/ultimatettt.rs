use super::board::*;
use super::tictactoe::*;

#[derive(Debug)]
pub struct UTTTBoard {
    state: Option<FinishState>,
    board: [TTTBoard; 9],
    play_in: Option<BoardPosition>,
}

impl UTTTBoard {
    pub fn play(
        &mut self,
        player: Player,
        large_position: BoardPosition,
        small_position: BoardPosition,
    ) -> PlayResult {
        // check state to see if the game is already over
        if self.state.is_some() {
            return PlayResult::BoardIsFilled;
        }

        // check if large_position is allowed by play_in
        if self.play_in.is_some() {
            if self.play_in.unwrap() != large_position {
                // large_position is not allowed
                return PlayResult::WrongBoard;
            }
        }

        // Get board at large_position
        let board_option = self.board.get_mut(large_position.to_index());

        // unwarp board_option
        let mut board: &mut TTTBoard;
        match board_option {
            Some(x) => board = x,
            None => return PlayResult::IndexError,
        }

        // play on small board and store result
        let mut play_result = board.play(player, small_position);

        // check if the board is finished
        if let PlayResult::BoardFinish(_) = play_result {
            // small board is finished so check if game is over
            let game_finish = self.check_for_game_finish();
            if let Some(finish_state) = game_finish {
                // game is over so set state and return
                self.state = Some(finish_state);
                return PlayResult::GameFinish(finish_state);
            }
        }

        // if play_result is_success then set play_in
        if play_result.is_success() {
            // only restrict play if the board can be played in
            if self.board[small_position.to_index()].state.is_none() {
                self.play_in = Some(small_position);
            } else {
                self.play_in = None;
            }
        }

        // no special case on the large board occured
        return play_result;
    }

    pub fn check_for_game_finish(&self) -> Option<FinishState> {
        // map every index in WINNING_LINES to its value
        let mapped_lines = WINNING_LINES
            .iter()
            .map(|line|
                // Turn an array of indexs into the value of their board state
                line.iter()
                    .map(|index| &self.board[*index].state)
                    .collect::<Vec<&Option<FinishState>>>())
            .collect::<Vec<Vec<&Option<FinishState>>>>();

        // Check if X won
        let x_won = mapped_lines
            .iter()
            .map(|line| {
                line.iter()
                    .all(|position| **position == Some(FinishState::Win(Player::X)))
            })
            .any(|line_win| line_win == true);
        if x_won {
            return Some(FinishState::Win(Player::X));
        }

        // Check if O won
        let o_won = mapped_lines
            .iter()
            .map(|line| {
                line.iter()
                    .all(|position| **position == Some(FinishState::Win(Player::O)))
            })
            .any(|line_win| line_win == true);
        if o_won {
            return Some(FinishState::Win(Player::O));
        }

        // Check for draw
        let full = !self.board.iter().any(|position| position.state == None);
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
            board: [TTTBoard::new(); 9],
            play_in: None,
        }
    }
}