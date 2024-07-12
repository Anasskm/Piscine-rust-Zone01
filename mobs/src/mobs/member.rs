#[derive(Debug, Clone, PartialEq)]
pub struct Member {
    pub name: String,
    pub role: Role,
    pub age: u8,
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Role {
    Underboss,
    Caporegime,
    Soldier,
    Associate,
}

impl Member {
    pub fn new(name: &str, role: Role, age: u8) -> Member {
        Member {
            name: name.to_string(),
            role,
            age,
        }
    }
    pub fn get_promotion(&mut self) {
        match self.role {
            Role::Associate => self.role = Role::Soldier,
            Role::Soldier => self.role = Role::Caporegime,
            Role::Caporegime => self.role = Role::Underboss,
            _ => {}
        }
    }
}
impl Role {
    pub fn power(&self) -> u8 {
        match self {
            Role::Associate => 1,
            Role::Soldier => 2,
            Role::Caporegime => 3,
            Role::Underboss => 4,
        }
    }
}
