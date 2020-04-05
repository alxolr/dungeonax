extern crate rand;

use rand::{thread_rng, Rng};
use std::cmp::PartialEq;

#[derive(Debug, Clone)]
pub struct Character {
    pub name: String,
    pub health: i32,
    attack: (i32, i32),
}

impl Character {
    pub fn new(name: String, health: i32, attack: (i32, i32)) -> Character {
        Character {
            name: name,
            health: health,
            attack: attack,
        }
    }

    pub fn receive_damage(&mut self, damage: i32) {
        self.health -= damage;
    }

    pub fn attack(&self) -> i32 {
        let mut rng = thread_rng();
        let attack_dmg = rng.gen_range(self.attack.0, self.attack.1) as i32;

        println!("{} attacked for {} damage", &self.name, &attack_dmg);

        attack_dmg
    }
}

impl PartialEq for Character {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.attack == other.attack
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn have_new_instance() {
        let name: String = "John".to_string();
        const HEALTH: i32 = 100;
        const ATTACK: (i32, i32) = (100, 150);

        assert_eq!(
            Character {
                name: name.clone(),
                health: HEALTH,
                attack: ATTACK,
            },
            Character::new(name.clone(), HEALTH, ATTACK)
        )
    }

    #[test]
    fn should_have_attack() {
        let name: String = "John".to_string();
        const HEALTH: i32 = 100;
        const ATTACK: (i32, i32) = (100, 150);

        let ch = Character::new(name, HEALTH, ATTACK);
        let attack = ch.attack();

        println!("{} Attacked for", &attack);

        assert_eq!(attack >= ATTACK.0, true);
        assert_eq!(attack <= ATTACK.1, true);
    }

    #[test]
    fn receive_damage() {
        let mut character = Character::new("John".to_string(), 100, (50, 60));
        const DAMAGE: i32 = 32;

        character.receive_damage(DAMAGE);

        assert_eq!(character.health, 100 - DAMAGE);
    }

    #[test]
    fn test_attack() {
        let character = Character::new("John".to_string(), 100, (50, 60));

        assert_eq!(character.attack, (50, 60))
    }
}
