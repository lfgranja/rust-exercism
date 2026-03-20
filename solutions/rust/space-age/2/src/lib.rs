const YEAR_IN_EARTH: u64 = 31_557_600;

#[derive(Debug)]
pub struct Duration {
    seconds: u64,
}

/// Represents a duration of time, measured in seconds.
impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration { seconds: s }
    }
}

impl Duration {
    fn in_earth_years(&self) -> f64 {
        self.seconds as f64 / YEAR_IN_EARTH as f64
    }
}

/// Represents a planet and provides methods to convert durations to years on that planet.
pub trait Planet {
    const ORBITAL_PERIOD: f64;
    fn years_during(d: &Duration) -> f64 {
        d.in_earth_years() / Self::ORBITAL_PERIOD
    }
}

macro_rules! planet {
    ($name:ident, $period:expr) => {
        pub struct $name;
        impl Planet for $name {
            const ORBITAL_PERIOD: f64 = $period;
        }
    };
}

planet!(Mercury, 0.2408467);
planet!(Venus, 0.61519726);
planet!(Earth, 1.0);
planet!(Mars, 1.8808158);
planet!(Jupiter, 11.862615);
planet!(Saturn, 29.447498);
planet!(Uranus, 84.016846);
planet!(Neptune, 164.79132);
