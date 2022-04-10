pub struct GameData {
    pub hp: i32,
    pub name: String,
}

impl GameData {
    pub fn takes_damage(&mut self, hit_point: i32) -> i32 {
        self.hp -= hit_point;
        self.hp
    }
    pub fn heals_itself(&mut self, heal_point: i32) -> i32 {
        self.hp += heal_point;
        self.hp
    }
    pub fn show_hp(&self) -> i32 {
        self.hp
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn hit() {
        let mut game = GameData { hp: 100, name: "lol".to_string() };

        let hit = 90;

        assert_eq!(10, game.takes_damage(hit));
    }

    fn heal() {
        let mut game = GameData { hp: 100, name: "lol".to_string()  };

        let heal = 10;

        assert_eq!(10, game.heals_itself(heal));
    }
}
