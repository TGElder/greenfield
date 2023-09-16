pub enum Reservation {
    Permanent { id: usize, from: u128 },
    Temporary { id: usize, from: u128, to: u128 },
}

impl Reservation {
    pub fn id(&self) -> &usize {
        match self {
            Self::Permanent { id, .. } => id,
            Self::Temporary { id, .. } => id,
        }
    }
}
