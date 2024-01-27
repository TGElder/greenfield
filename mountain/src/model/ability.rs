#[derive(Clone, Copy)]
pub enum Ability {
    Beginner,
    Intermediate,
    Advanced,
    Expert,
}

impl Ability {
    pub fn max_grade(&self) -> f32 {
        match self {
            Ability::Beginner => 0.15,
            Ability::Intermediate => 0.25,
            Ability::Advanced => 0.40,
            Ability::Expert => 0.70,
        }
    }
}

pub const ABILITIES: [Ability; 4] = [
    Ability::Beginner,
    Ability::Intermediate,
    Ability::Advanced,
    Ability::Expert,
];
