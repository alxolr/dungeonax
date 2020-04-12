use crate::character::Character;

#[derive(Debug, PartialEq)]
pub enum ArenaCorner {
    Left,
    Right,
}

#[derive(Debug)]
pub struct Arena<'a> {
    pub left: &'a mut Character,
    pub right: &'a mut Character,
    pub winner: Option<ArenaCorner>,
    pub round: i32,
}

impl<'a> Arena<'a> {
    pub fn new(left_ch: &'a mut Character, right_ch: &'a mut Character) -> Self {
        Arena {
            left: left_ch,
            right: right_ch,
            winner: None,
            round: 0,
        }
    }

    pub fn fight(&mut self) {
        self.left.info().unwrap().print_stdout();
        self.right.info().unwrap().print_stdout();

        while self.left.health >= 0 && self.right.health >= 0 {
            self.round += 1;
            println!("Round {}\n", &self.round);

            self.right.receive_damage(self.left.attack());
            self.right.info().unwrap().print_stdout();

            self.left.receive_damage(self.right.attack());
            self.left.info().unwrap().print_stdout();
            println!("\n");

            if self.left.health < 0 && self.right.health < 0 {
                self.winner = if self.left.health > self.right.health {
                    Some(ArenaCorner::Left)
                } else {
                    Some(ArenaCorner::Right)
                }
            } else if self.left.health <= 0 {
                self.winner = Some(ArenaCorner::Right);
            } else {
                self.winner = Some(ArenaCorner::Left);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn starts_at_round_zero() {
        let mut left_ch = Character::new("Any".to_string(), 100, (50, 60));
        let mut right_ch = Character::new("Job".to_string(), 300, (50, 70));
        let arena = Arena::new(&mut left_ch, &mut right_ch);

        assert_eq!(arena.round, 0);
    }

    #[test]
    fn fight_until_death() {
        let mut left_ch = Character::new("Any".to_string(), 100, (50, 60));
        let mut right_ch = Character::new("Job".to_string(), 300, (50, 100));
        let mut arena = Arena::new(&mut left_ch, &mut right_ch);
        &mut arena.fight();

        println!("{:?}", arena.winner)
    }
}
