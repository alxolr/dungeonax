mod arena;
mod character;

use crate::arena::Arena;
use crate::character::Character;

fn main() {
    println!("Hello, world!");

    let mut arena = Arena::new(
        Character::new("Johny".to_string(), 1232213210, 15),
        Character::new("Helga".to_string(), 2132131310, 30),
    );

    arena.fight();

    println!(
        "Arena winner is {} after {} rounds",
        arena
            .winner
            .unwrap_or(Character::new("The Arena".to_string(), 100, 100))
            .name,
        arena.round
    );
}
