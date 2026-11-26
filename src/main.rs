

use std::io;
struct Player {
    name: String,
    hp: i32,
}
impl Player {
    fn new(name: &str, hp: i32) -> Self{
        Self {
            name: String::from(name),
            hp,
        }
    }
    fn take_dmg(&mut self, amount: i32) {
        self.hp -= amount;
        if self.hp < 0 {
            self.hp = 0;
        }
    }
    fn is_alive(&self) -> bool {
        self.hp > 0
    }
}
fn main() {
    let mut p = Player::new("player", 100);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line.");
    let dmg: i32 = input
        .trim()
        .parse()
        .expect("Invalid Number.");
    p.take_dmg(dmg);
    println!("{} has {} hp (alive: {})", p.name, p.hp, p.is_alive());
}
