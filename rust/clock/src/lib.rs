/*
* Instructions
* * Implement a clock that handles times without dates.
* * You should be able to add and subtract minutes to it.
* *
* * Two clocks that represent the same time should be equal to each other.
* *
* * Rust Traits for .to_string()
* * You will also need to implement .to_string() for the Clock struct. We will be using this to display the Clock's state. You can either do it via implementing it directly or using the Display trait.
* *
* * If so, try implementing the Display trait for Clock instead.
* *
* * Traits allow for a common way to implement functionality for various types.
* *
* * For additional learning, consider how you might implement String::from for the Clock type. You don't have to actually implement this—it's redundant with Display, which is generally the better choice when the destination type is String—but it's useful to have a few type-conversion traits in your toolkit.
* *
* * Source
* * Pairing session with Erin Drummond
* *
*/

use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub struct Clock {
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let total_minutes = (hours as i64) * 60 + (minutes as i64);
        Clock {
            minutes: total_minutes.rem_euclid(1440) as i32,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(0, self.minutes + minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let hours = self.minutes / 60;
        let minutes = self.minutes % 60;
        write!(f, "{:02}:{:02}", hours, minutes)
    }
}

impl From<Clock> for String {
    fn from(clock: Clock) -> Self {
        // Como você já implementou o Display, pode simplesmente reaproveitá-lo aqui:
        clock.to_string()
    }
}
