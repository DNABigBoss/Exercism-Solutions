use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock{
    hours:i32,
    minutes:i32,
}

impl fmt::Display for Clock {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Clock {
        let allMinutes = hours*60+minutes;
        let m = ((allMinutes)%1440+1440)%1440;
        let hours = m/60;
        let minutes = m%60;
        Clock {hours ,minutes}
    }

    pub fn add_minutes(&self, minutes: i32) -> Clock {
        Clock::new(self.hours, self.minutes+minutes)
    }
}
