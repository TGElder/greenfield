use crate::utils::computer::{self, global_costs};

pub struct System {
    update: bool,
}

impl System {
    pub fn new() -> System {
        System { update: false }
    }

    pub fn update(&mut self) {
        self.update = true;
    }

    pub fn run(&mut self, parameters: computer::global_costs::Parameters) {
        if !self.update {
            return;
        }
        self.update = false;

        global_costs::compute_global_costs(parameters);
    }
}
