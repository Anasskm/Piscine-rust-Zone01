pub mod boss;
pub mod member;

pub use boss::Boss;
pub use member::Member;

#[derive(Debug, Clone, PartialEq)]
pub struct Mob {
    pub name: String,
    pub boss: Boss,
    pub members: Vec<Member>,
    pub cities: Vec<(String, u8)>,
    pub wealth: u32,
}

impl Mob {
    pub fn recruit(&mut self, name: &str, age: u8) {
        self.members
            .push(Member::new(name, member::Role::Associate, age))
    }
    pub fn attack(&mut self, mob: &mut Mob) {
        let pw_self: u8 = self.members.iter().map(|m| m.role.power()).sum();
        let pw_mob: u8 = mob.members.iter().map(|m| m.role.power()).sum();
        if pw_self > pw_mob {
            mob.members.pop();
            if mob.members.len() == 0 {
                self.wealth += mob.wealth;
                mob.wealth = 0;
                while mob.cities.len() > 0 {
                    self.cities.push(mob.cities[0].clone());
                    mob.cities.remove(0);
                }
            }
            return;
        }
        self.members.pop();
        if self.members.len() == 0 {
            mob.wealth += self.wealth;
            self.wealth = 0;
            while self.cities.len() > 0 {
                mob.cities.push(self.cities[0].clone());
                self.cities.remove(0);
            }
        }
    }
    pub fn steal(&mut self, mob: &mut Mob, value: u32) {
        if mob.wealth > value {
            mob.wealth -= value;
            self.wealth += value;
            return;
        }
        self.wealth += mob.wealth;
        mob.wealth = 0;
    }
    pub fn conquer_city(&mut self, mobs: Vec<Mob>, city: String, value: u8) {
        let have = mobs
            .iter()
            .flat_map(|m| m.cities.clone())
            .any(|(e, _)| e == city);
        if !have {
            self.cities.push((city, value))
        }
    }
}
