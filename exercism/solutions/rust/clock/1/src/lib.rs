use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let (hours, minutes) = Self::make_clock(hours, minutes);

        Self { hours, minutes }
    }

    pub fn add_minutes(&mut self, minutes_added: i32) -> Self {
        let (hours, minutes) = Self::make_clock(self.hours, self.minutes + minutes_added);

        Self { hours, minutes }
    }

    fn make_clock(mut hours: i32, mut minutes: i32) -> (i32, i32) {
        let inc_hours = minutes / 60;
        minutes %= 60;

        if minutes >= 0 {
            hours = (hours + inc_hours) % 24;
        } else {
            hours = (hours + inc_hours - 1) % 24;
            minutes += 60;
        }

        if hours < 0 {
            hours += 24;
        }

        (hours, minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}