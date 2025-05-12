#[derive(Debug)]
#[allow(dead_code)]
struct City {
    name: String,
    population: i64,
    country: String,
}

pub fn run() -> () {
    println!("Introduction");
    
    let tokyo = City {
        name: "tokyo".to_string(),
        population: 14_186_237,
        country: "Japan".to_string(),
    };
    let chicago = City {
        name: "chicago".to_string(),
        population: 2_695_598,
        country: "Japan".to_string(),
    };

    let mut cities = vec![chicago, tokyo];
    sort_cities(&mut cities);
    println!("{:?}", cities);
    sort_cities2(&mut cities);
    println!("{:?}", cities);
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
