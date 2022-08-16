use crate::{player::Player, frame::{Frame, Drawable}, NUM_COLS};

const LABELS: [&str; 10] = ["0", "1", "2", "3", "4", "5",  "6", "7", "8", "9"];

pub struct Scoreboard {
    pub value: usize,
}

impl Scoreboard {
    pub fn new() -> Self {
        Scoreboard{value: 0}
    }

    pub fn update(&mut self, player: &Player) {
        self.value = player.score;
    }
}

impl Drawable for Scoreboard {
    fn draw(&self, frame: &mut Frame) {
        let len = self.value.to_string().len();
        for (idx, ch) in self.value.to_string().char_indices() {
            let x = NUM_COLS - 1 - len + idx;
            frame[x][0] = LABELS[(ch as usize) - 48];
        }

    }
}
