extern crate strum;
#[macro_use]
extern crate strum_macros;

use std::collections::HashMap;
use strum::IntoEnumIterator;

fn main() {
    println!("Hello, world!");
}

fn set_schedule(locations: &mut [Location], optoms: &[Optom]) {
    for period in Period::iter() {
        *locations[0].schedule.get_mut(&period).unwrap() =
            LocationStatus::Allocated(optoms[0].name.clone());
    }
}

#[derive(Clone, Hash, Eq, PartialEq, EnumIter)]
pub enum Period {
    MondayAm,
    MondayPm,
    TuesdayAm,
    TuesdayPm,
    WednesdayAm,
    WednesdayPm,
    ThursdayAm,
    ThursdayPm,
    FridayAm,
    FridayPm,
    SaturdayAm,
    SaturdayPm,
}

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub enum LocationStatus {
    Open,
    Closed,
    Allocated(String),
}

#[derive(Clone, Hash, Eq, PartialEq)]
pub enum OptomStatus {
    Free,
    Off,
    Working,
}

pub struct Location {
    pub schedule: HashMap<Period, LocationStatus>,
    pub name: String,
}

pub struct Optom {
    pub schedule: HashMap<Period, OptomStatus>,
    pub name: String,
    pub location_fitness: HashMap<String, i32>,
    pub location_must: HashMap<Period, String>,
}

impl Location {
    fn new(name: String) -> Location {
        let schedule: HashMap<Period, LocationStatus> = [
            (Period::MondayAm, LocationStatus::Open),
            (Period::MondayPm, LocationStatus::Open),
            (Period::TuesdayAm, LocationStatus::Open),
            (Period::TuesdayPm, LocationStatus::Open),
            (Period::WednesdayAm, LocationStatus::Open),
            (Period::WednesdayPm, LocationStatus::Open),
            (Period::ThursdayAm, LocationStatus::Open),
            (Period::ThursdayPm, LocationStatus::Open),
            (Period::FridayAm, LocationStatus::Open),
            (Period::FridayPm, LocationStatus::Open),
            (Period::SaturdayAm, LocationStatus::Open),
            (Period::SaturdayPm, LocationStatus::Open),
        ]
        .iter()
        .cloned()
        .collect();

        Location { name, schedule }
    }
}

impl Optom {
    fn new(name: String) -> Optom {
        let schedule: HashMap<Period, OptomStatus> = [
            (Period::MondayAm, OptomStatus::Free),
            (Period::MondayPm, OptomStatus::Free),
            (Period::TuesdayAm, OptomStatus::Free),
            (Period::TuesdayPm, OptomStatus::Free),
            (Period::WednesdayAm, OptomStatus::Free),
            (Period::WednesdayPm, OptomStatus::Free),
            (Period::ThursdayAm, OptomStatus::Free),
            (Period::ThursdayPm, OptomStatus::Free),
            (Period::FridayAm, OptomStatus::Free),
            (Period::FridayPm, OptomStatus::Free),
            (Period::SaturdayAm, OptomStatus::Free),
            (Period::SaturdayPm, OptomStatus::Free),
        ]
        .iter()
        .cloned()
        .collect();

        Optom {
            name,
            schedule,
            location_fitness: HashMap::new(),
            location_must: HashMap::new(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn when_one_location_and_one_optom_schedule_is_filled() {
        let mut optom1 = Optom::new("Optom1".to_string());
        optom1.location_fitness.insert("Location1".to_string(), 1);
        let optoms = vec![optom1];

        let location1 = Location::new("Location1".to_string());
        let mut locations = vec![location1];

        set_schedule(&mut locations, &optoms);

        assert!(locations[0]
            .schedule
            .values()
            .all(|v| *v == LocationStatus::Allocated("Optom1".to_string())));
    }
}
