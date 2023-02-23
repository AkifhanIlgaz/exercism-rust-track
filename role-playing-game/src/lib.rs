pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        if self.health == 0 {
            if self.level >= 10 {
                return Some(Player {
                    health: 100,
                    mana: Some(100),
                    level: self.level,
                });
            } else {
                return Some(Player {
                    health: 100,
                    mana: None,
                    level: self.level,
                });
            }
        }
        None
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        match self.mana {
            Some(mana) => {
                if mana < mana_cost {
                    return 0;
                } else {
                    self.mana = Some(mana.saturating_sub(mana_cost));
                    return mana_cost * 2;
                }
            }
            None => {
                self.health = self.health.saturating_sub(mana_cost);
                return 0;
            }
        }
    }
}
