trait Combatant {
    fn attack(&mut self, opponent: &mut dyn Combatant);
    fn take_damage(&mut self, damage_amount: i32);
}

/// The protagonist of our epic MMORPG.
struct Alchemist {
    hp: i32,
    potion_count: i32,
}

impl Alchemist {
    const MAXIMUM_HP: i32 = 50;
    const STARTING_POTION_COUNT: i32 = 3;
    const ATTACK_POWER: i32 = 10;
    const POTION_POWER: i32 = 30;
    /// Make a new player in the initial state.
    fn new() -> Alchemist {
        return Alchemist {
            hp: Alchemist::MAXIMUM_HP,
            potion_count: Alchemist::STARTING_POTION_COUNT,
        };
    }
    fn print_status(&self) {
        println!(
            "Player HP: {}, potions left: {}",
            self.hp, self.potion_count
        );
    }
    fn use_potion(&mut self) -> bool {
        if self.potion_count > 0 {
            /* Solution 1
            if (self.hp + 30 > Player::MAXIMUM_HP) {
                self.hp = Player::MAXIMUM_HP;
            } else {
                self.hp += 30;
            }
            */
            /* Solution 2
            self.hp += 30;
            if self.hp > Player::MAXIMUM_HP {
                self.hp = Player::MAXIMUM_HP;
            }
            */
            self.hp = (self.hp + Alchemist::POTION_POWER).min(Alchemist::MAXIMUM_HP);
            self.potion_count -= 1;
            return true;
        } else {
            return false;
        }
    }
}

impl Combatant for Alchemist {
    fn take_damage(&mut self, damage_amount: i32) {
        self.hp -= damage_amount;
    }
    fn attack(&mut self, opponent: &mut dyn Combatant) {
        opponent.take_damage(Alchemist::ATTACK_POWER);
    }
}

/// The antagonist of our epic MMORPG.
struct Warrior {
    hp: i32,
}

impl Warrior {
    const MAXIMUM_HP: i32 = 100;
    const ATTACK_POWER: i32 = 10;
    fn new() -> Warrior {
        return Warrior {
            hp: Warrior::MAXIMUM_HP,
        };
    }
    fn print_status(&self) {
        println!("Opponent HP: {}", self.hp);
    }
}

impl Combatant for Warrior {
    fn take_damage(&mut self, damage_amount: i32) {
        self.hp -= damage_amount;
    }
    fn attack(&mut self, opponent: &mut dyn Combatant) {
        opponent.take_damage(Warrior::ATTACK_POWER);
    }
}

fn read_one_line() -> String {
    // TODO: don't be magic
    std::io::stdin().lines().next().unwrap().unwrap()
}

enum PlayerMove {
    Attack,
    Drink,
}

fn read_player_move() -> Option<PlayerMove> {
    let response = read_one_line();
    match response.to_lowercase().as_str() {
        "a" | "attack" | "attacke" | "たたかう" | "戦う" => return Some(PlayerMove::Attack),
        "p" | "potion" | "drink" | "trinke" => return Some(PlayerMove::Drink),
        _ => {
            // _ equivalent of regex /.*/, default case
            return None;
        }
    }
    // if one thing { return PlayerMove::Attack }
    // else if other thing { return PlayerMove::Drink }
}

fn main() {
    let mut player = Alchemist::new();
    let mut opponent = Alchemist::new();

    loop {
        if player.hp <= 0 {
            println!("You died!");
            break;
        } else if opponent.hp <= 0 {
            println!("You won!");
            break;
        }

        player.print_status();
        opponent.print_status();
        println!("What will you do?");

        match read_player_move() {
            Some(PlayerMove::Attack) => {
                player.attack(&mut opponent);
            }
            Some(PlayerMove::Drink) => {
                let success = player.use_potion();
                if success {
                    println!("You used a potion.");
                } else {
                    println!("You don't have any potions left!");
                    continue;
                }
            }
            None => {
                println!("Invalid input.");
                println!("Valid inputs are A and P");
                continue;
            }
        }

        opponent.attack(&mut player);
    }
}
