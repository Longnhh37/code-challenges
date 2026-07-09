const SECONDS_PER_EARTH_YEAR: f64 = 31_557_600.0; //365.25 * 24 * 60 * 60

#[derive(Debug)]
pub struct Duration {
    seconds: u64,
}

impl From<u64> for Duration {
    fn from(seconds: u64) -> Self {
        Self { seconds }
    }
}

impl Duration {
    fn as_years(&self) -> f64 {
        self.seconds as f64 / SECONDS_PER_EARTH_YEAR
    }

    pub fn years_on<P: Planet>(&self) -> f64 {
        self.as_years() / P::PERIOD
    }
}

pub trait Planet {
    const PERIOD: f64;
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

impl Planet for Mercury {
    const PERIOD: f64 = 0.2408467;
}

impl Planet for Venus {
    const PERIOD: f64 = 0.61519726;
}
impl Planet for Earth {
    const PERIOD: f64 = 1.0;
}
impl Planet for Mars {
    const PERIOD: f64 = 1.8808158;
}
impl Planet for Jupiter {
    const PERIOD: f64 = 11.862651;
}

impl Planet for Saturn {
    const PERIOD: f64 = 29.4474988;
}

impl Planet for Uranus {
    const PERIOD: f64 = 84.016846;
}

impl Planet for Neptune {
    const PERIOD: f64 = 164.79132;
}

fn main() {
    let d = Duration::from(1_000_000_000);

    println!("Earth: {:.2}", d.years_on::<Earth>());
    println!("Mercury: {:.2}", d.years_on::<Mercury>());
    println!("Mars: {:.2}", d.years_on::<Mars>());
}
