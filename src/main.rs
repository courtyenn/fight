/// The protagonist of our epic MMORPG.
struct Player {
    hp: i32,
    potion_count: i32,
}

impl Player {
    const MAXIMUM_HP: i32 = 50;
    const STARTING_POTION_COUNT: i32 = 3;
    const ATTACK_POWER: i32 = 10;
    const POTION_POWER: i32 = 30;
    /// Make a new player in the initial state.
    fn new() -> Player {
        return Player {
            hp: Player::MAXIMUM_HP,
            potion_count: Player::STARTING_POTION_COUNT,
        };
    }
    fn print_status(&self) {
        println!(
            "Player HP: {}, potions left: {}",
            self.hp, self.potion_count
        );
    }
    fn take_damage(&mut self, damage_amount: i32) {
        self.hp -= damage_amount;
    }
    fn attack(&mut self, opponent: &mut Opponent) {
        opponent.take_damage(Player::ATTACK_POWER);
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
            self.hp = (self.hp + Player::POTION_POWER).min(Player::MAXIMUM_HP);
            self.potion_count -= 1;
            return true;
        } else {
            return false;
        }
    }
}

/// The antagonist of our epic MMORPG.
struct Opponent {
    hp: i32,
}

impl Opponent {
    const MAXIMUM_HP: i32 = 100;
    fn new() -> Opponent {
        return Opponent {
            hp: Opponent::MAXIMUM_HP,
        };
    }
    fn take_damage(&mut self, damage_amount: i32) {
        self.hp -= damage_amount;
    }
    fn attack(&mut self, player: &mut Player) {
        player.take_damage(Player::ATTACK_POWER);
    }
    fn print_status(&self) {
        println!("Opponent HP: {}", self.hp);
    }
}

fn read_one_line() -> String {
    // TODO: don't be magic
    std::io::stdin().lines().next().unwrap().unwrap()
}

fn main() {
    let mut player = Player::new();
    let mut opponent = Opponent::new();

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
        let response = read_one_line();
        if response == "A" {
            player.attack(&mut opponent);
        } else if response == "P" {
            let success = player.use_potion();
            if success {
                println!("You used a potion.");
            } else {
                println!("You don't have any potions left!");
                continue;
            }
        } else {
            println!("Invalid input: {}", response);
            println!("Valid inputs are A and P");
            continue;
        }

        opponent.attack(&mut player);
    }
}