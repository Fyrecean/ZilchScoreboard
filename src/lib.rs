use std::vec;
use std::io;

#[derive(Debug)]
pub struct Player {
    name: String,
    rolls: Vec<Option<u32>>,
    score: u32,
}
impl Player {
    pub fn new(name: String) -> Player {
        Player {
            name,
            rolls: vec![None],
            score: 0 
        }
    }

    fn add_points(&mut self, amount: u32) -> u32 {
        self.rolls.push(Some(amount));
        self.score += amount;
        self.score
    }
}

pub fn run(mut players: Vec<Player>) {
    let player_count = players.len();
    let mut turn = 0;
    let take_turn = |player: &mut Player| -> u32 {
        let roll = parse_input();
        player.add_points(roll)
    };
    let mut winner = loop {
        print_scores(&players);
        let points = take_turn(&mut players[turn % player_count]);
        if points >= 10_000 {
            break turn % player_count
        }
        turn += 1;
    };
    turn += 1;
    for _i in 1..player_count {
        print_scores(&players);
        let score = take_turn(&mut players[turn % player_count]);
        if score > players[winner].score {
            winner = turn % player_count;
        }    
        turn += 1;
    }
    print_scores(&players);
    println!(
        "{} has won with a score of {}.", 
        players[winner].name, 
        players[winner].score
    );
} 

fn parse_input() -> u32 {
    loop {
        let mut buf = String::new();
        match io::stdin().read_line(&mut buf) {
            Ok(_) => {
                match buf.trim().parse::<u32>() {
                    Ok(val) => {
                        break val;
                    },
                    Err(error) => println!("Try again. \n{}", error)
                }
            },
            Err(error) => println!("Try again. \n{}", error)
        }
    }
}

fn print_scores(players: &Vec<Player>) {
    for player in players.iter() {
        println!("{}: {}", player.name, player.score);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn print_test() {
        assert!(true);
    }
}