mod arena;
mod character;

use crate::arena::{Arena, ArenaCorner};
use crate::character::Character;

fn main() {
    println!("Hello, world!");
    let mut johny = Character::new("Johny Cage".to_string(), 100, (10, 15));
    let mut shan = Character::new("Shan Tsung".to_string(), 40, (10, 45));

    let mut arena = Arena::new(&mut johny, &mut shan);
    arena.fight();

    match arena.winner {
        Some(x) => match x {
            ArenaCorner::Right => println!("The duel won {}", &arena.right.name),
            ArenaCorner::Left => println!("The duel won {}", &arena.left.name),
        },
        None => println!("No one wone"),
    }
}
