const MINUTES_PER_HOUR: i32 = 60;

#[derive(Debug, PartialEq)]
pub struct Clock {
  pub hours: i32,
  pub minutes: i32,
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
