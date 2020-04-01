use std::cmp::PartialEq;

#[derive(Debug, Clone)]
pub struct Character {
    pub name: String,
    pub health: i32,
    pub attack: i32,
}

impl Character {
    pub fn new(name: String, health: i32, attack: i32) -> Character {
        Character {
            name: name,
            health: health,
            attack: attack,
        }
    }

    pub fn receive_damage(&mut self, damage: i32) {
        self.health -= damage;
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
        const ATTACK: i32 = 100;

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
    fn receive_damage() {
        let mut character = Character::new("John".to_string(), 100, 50);
        const DAMAGE: i32 = 32;

        character.receive_damage(DAMAGE);

        assert_eq!(character.health, 100 - DAMAGE);
    }

    #[test]
    fn attack() {
        let character = Character::new("John".to_string(), 100, 50);

        assert_eq!(character.attack, 50);
    }
}
