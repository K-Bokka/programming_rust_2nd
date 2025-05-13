use crate::s00_intro::{City, get_chicago, get_tokyo};

pub fn run() -> () {
    println!("Capture of variables");

    let tokyo = get_tokyo();
    let chicago = get_chicago();
    let mut cities = vec![chicago, tokyo];
    sort_by_statistic(&mut cities, Statistic::Population);
    println!("{:?}", cities);
    sort_by_statistic(&mut cities, Statistic::Gdp);
    println!("{:?}", cities);
    let cities = start_sorting_thread(cities, Statistic::Population)
        .join()
        .unwrap();
    println!("{:?}", cities);
}

#[derive(Copy, Clone)]
enum Statistic {
    Population,
    Gdp,
}

impl City {
    fn get_statistic(&self, stat: Statistic) -> i64 {
        match stat {
            Statistic::Population => self.population,
            Statistic::Gdp => self.gdp,
        }
    }
}

fn sort_by_statistic(cities: &mut Vec<City>, stat: Statistic) -> () {
    cities.sort_by_key(|city| -city.get_statistic(stat));
}

use std::thread;

fn start_sorting_thread(mut cities: Vec<City>, stat: Statistic) -> thread::JoinHandle<Vec<City>> {
    let key_fb = move |city: &City| -> i64 { -city.get_statistic(stat) };
    thread::spawn(move || {
        cities.sort_by_key(key_fb);
        cities
    })
}
