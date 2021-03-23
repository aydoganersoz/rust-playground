const MINUTES_PER_HOUR: i32 = 60;

#[derive(Debug, PartialEq)]
struct Clock {
    hours: i32,
    minutes: i32,
}

impl ToString for Clock {
    fn to_string(&self) -> String {
        format!("{:02}:{:02}", self.hours, self.minutes)
    }
}

impl From<Clock> for String {
    fn from(clock: Clock) -> Self {
        let hours = clock.hours + clock.minutes / MINUTES_PER_HOUR;
        let minutes = clock.minutes % MINUTES_PER_HOUR;
        format!("{:02}:{:02}", hours, minutes)
    }
}

// ToString trait
fn test1() {
    let my_clock = Clock {
        hours: 23,
        minutes: 59,
    };

    assert_eq!(my_clock.to_string(), "23:59");
}

// From trait
fn test2() {
    let my_clock = Clock {
        hours: 23,
        minutes: 59,
    };

    assert_eq!(String::from(my_clock), "23:59");
}

pub fn test() {
    test1();
    test2();
}
