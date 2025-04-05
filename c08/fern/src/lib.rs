pub struct Fern {
    size: f64,
    growth_rate: f64,
}

impl Fern {
    fn grow(&mut self) {
        self.size *= 1.0 + self.growth_rate;
    }
}

pub fn run_simulation(fern: &mut Fern, days: usize) {
    for _ in 0..days {
        fern.grow();
    }
}
