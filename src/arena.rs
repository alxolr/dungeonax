use crate::character::Character;

#[derive(Debug)]
pub struct Arena {
    left: Character,
    right: Character,
    pub winner: Option<Character>,
    pub round: i32,
}

impl Arena {
    pub fn new(left_ch: Character, right_ch: Character) -> Arena {
        Arena {
            left: left_ch,
            right: right_ch,
            winner: None,
            round: 0,
        }
    }

    pub fn fight(&mut self) {
        while self.left.health >= 0 && self.right.health >= 0 {
            self.right.receive_damage(self.left.attack);
            self.left.receive_damage(self.right.attack);
            self.round += 1;

            if self.left.health < 0 && self.right.health < 0 {
                self.winner = if self.left.health > self.right.health {
                    Some(self.left.clone())
                } else {
                    Some(self.right.clone())
                }
            } else if self.left.health <= 0 {
                self.winner = Some(self.right.clone());
            } else {
                self.winner = Some(self.left.clone());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn has_new() {
        let left_ch = Character::new("Any".to_string(), 100, 50);
        let right_ch = Character::new("Job".to_string(), 300, 50);
        let arena = Arena::new(left_ch, right_ch);

        assert_eq!(arena.round, 0);
    }

    #[test]
    fn fight_until_death() {
        let left_ch = Character::new("Any".to_string(), 100, 50);
        let right_ch = Character::new("Job".to_string(), 300, 50);
        let mut arena = Arena::new(left_ch, right_ch.clone());
        arena.fight();

        println!("{:?} Arena", arena);

        assert_eq!(arena.winner.unwrap(), right_ch);
    }
}
