
use {
    Field,
    Observer,
    Triggered,
};

pub struct IntervalEvent<'a> {
    pub interval: Field<'a, f64>,
}

impl<'a> Triggered<'a> for IntervalEvent<'a> {
    fn get_observer(&'a self) -> Box<Observer> {
        (box IntervalEventObserver::new(*self.interval.get())) as Box<Observer>
    }
}

struct IntervalEventObserver {
    can_trigger: bool,
    cur_interval: f64,
    interval: f64,
}

impl IntervalEventObserver {
    pub fn new(interval: f64) -> IntervalEventObserver {
        IntervalEventObserver {
            can_trigger: false,
            cur_interval: 0.0,
            interval: interval,
        }
    }
}

impl Observer for IntervalEventObserver {
    fn can_trigger(&self) -> bool {
        self.can_trigger
    }

    fn after_trigger(&mut self) {
        self.can_trigger = false
    }

    fn update(&mut self, dt: f64) {
        self.cur_interval += dt;
        if self.cur_interval > self.interval {
            self.can_trigger = true;
            self.cur_interval -= self.interval;
        }
    }
}

