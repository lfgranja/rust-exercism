/*
space-age/src/lib.rs
Introduction
The year is 2525 and you've just embarked on a journey to visit all planets in the Solar System (Mercury, Venus, Earth, Mars, Jupiter, Saturn, Uranus and Neptune). The first stop is Mercury, where customs require you to fill out a form (bureaucracy is apparently not Earth-specific). As you hand over the form to the customs officer, they scrutinize it and frown. "Do you really expect me to believe you're just 50 years old? You must be closer to 200 years old!"

Amused, you wait for the customs officer to start laughing, but they appear to be dead serious. You realize that you've entered your age in Earth years, but the officer expected it in Mercury years! As Mercury's orbital period around the sun is significantly shorter than Earth, you're actually a lot older in Mercury years. After some quick calculations, you're able to provide your age in Mercury Years. The customs officer smiles, satisfied, and waves you through. You make a mental note to pre-calculate your planet-specific age before future customs checks, to avoid such mix-ups.

Note
If you're wondering why Pluto didn't make the cut, go watch this YouTube video.

Instructions
Given an age in seconds, calculate how old someone would be on a planet in our Solar System.

One Earth year equals 365.25 Earth days, or 31,557,600 seconds. If you were told someone was 1,000,000,000 seconds old, their age would be 31.69 Earth-years.

For the other planets, you have to account for their orbital period in Earth Years:

Planet	Orbital period in Earth Years
Mercury	0.2408467
Venus	0.61519726
Earth	1.0
Mars	1.8808158
Jupiter	11.862615
Saturn	29.447498
Uranus	84.016846
Neptune	164.79132
Note
The actual length of one complete orbit of the Earth around the sun is closer to 365.256 days (1 sidereal year). The Gregorian calendar has, on average, 365.2425 days. While not entirely accurate, 365.25 is the value used in this exercise. See Year on Wikipedia for more ways to measure a year.

Topics
Some Rust topics you may want to read about while solving this problem:

Traits, both the From trait and implementing your own traits

Default method implementations for traits

Macros, the use of a macro could reduce boilerplate and increase readability for this exercise. For instance, a macro can implement a trait for multiple types at once, though it is fine to implement years_during in the Planet trait itself. A macro could define both the structs and their implementations. Info to get started with macros can be found at:

The Macros chapter in The Rust Programming Language
an older version of the Macros chapter with helpful detail
Rust By Example

*/

// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

const YEAR_IN_EARTH: u64 = 31_557_600;

///
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
