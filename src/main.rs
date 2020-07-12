use std::env;
use dice_scoreboard::{self as dice, Player};

fn main() {
    let mut names = env::args();
    names.next();
    let mut players: Vec<Player> = Vec::new(); 
    for name in names {
        players.push(Player::new(name));
    }
    dice::run(players);
}
