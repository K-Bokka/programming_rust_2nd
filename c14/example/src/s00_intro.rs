#[derive(Debug)]
#[allow(dead_code)]
pub struct City {
    pub name: String,
    pub population: i64,
    pub country: String,
    pub gdp: i64,
}

pub fn run() -> () {
    println!("Introduction");

    let tokyo = get_tokyo();
    let chicago = get_chicago();

    let mut cities = vec![chicago, tokyo];
    sort_cities(&mut cities);
    println!("{:?}", cities);
    sort_cities2(&mut cities);
    println!("{:?}", cities);
}

pub fn get_tokyo() -> City {
    City {
        name: "tokyo".to_string(),
        population: 14_186_237,
        country: "Japan".to_string(),
        gdp: 113_000_000_000_000,
    }
}

pub fn get_chicago() -> City {
    City {
        name: "chicago".to_string(),
        population: 2_695_598,
        country: "Japan".to_string(),
        gdp: 132_000_000_000_000,
    }
}

fn sort_cities(cities: &mut Vec<City>) {
    // cities.sort();
    cities.sort_by_key(city_population_descending)
}

fn city_population_descending(city: &City) -> i64 {
    -city.population
}

fn sort_cities2(cities: &mut Vec<City>) {
    // cities.sort();
    cities.sort_by_key(|city| city.population)
}
