pub trait Board {
    type Player;
    type PlayResult;
    type Move;
    type Finish;

    fn play(&mut self, player: Self::Player, choice: Self::Move) -> Self::PlayResult;

    fn check_for_finish(&self) -> Option<Self::Finish>;

    fn new_board() -> Self;
}
