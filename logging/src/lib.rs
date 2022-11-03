use log::{info, warn, error, debug, trace};
#[derive(Debug)]
pub struct Frog {
    energy: u8,
    sleeping: bool,
}

impl Frog {
    pub fn new() -> Self {
        debug!("A new Frog has been created");
        Default::default()
    }
    pub fn hop(&mut self) {
        self.energy -= 1;
        info!("Frog hopped, and how much energy is left");
        if self.energy == 0 {
            warn!("the frog will go to sleep since he ran out of energy");
            self.sleep();
        }
    }
    pub fn sleep(&mut self) {
        if self.sleeping {
            error!("the Frog is already asleep");
        } else {
            self.sleeping = true;
        }
    }
}

impl Default for Frog {
    fn default() -> Self {
        trace!("default value was generated");
        Frog {
            energy: 5,
            sleeping: false,
        }
    }
}