
use {
    Call,
    EventCenter,
    Field,
    Observer,
};

pub struct IntervalEvent<'a> {
    pub interval: Field<'a, f64>,
}

impl<'a> Call<'a> for IntervalEvent<'a> {
    fn call(&'a self, ec: &mut EventCenter, command: ||: 'static) -> uint {
        ec.add_observer(box IntervalEventObserver::new(*self.interval.get(), command))
    }
}

struct IntervalEventObserver {
    command: ||: 'static,
    can_trigger: bool,
    cur_interval: f64,
    interval: f64,
}

impl IntervalEventObserver {
    pub fn new(interval: f64, command: ||: 'static) -> IntervalEventObserver {
        IntervalEventObserver {
            command: command,
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

    fn trigger(&mut self) {
        (self.command)();
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

