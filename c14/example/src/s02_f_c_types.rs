use crate::s00_intro::{
    City, city_gdp_descending, city_population_descending, get_chicago, get_funafuti, get_tokyo,
};
use rand::random;

pub fn run() -> () {
    println!("Function type and closure type");

    let my_key_fn: fn(&City) -> i64 = if random::<bool>() {
        city_population_descending
    } else {
        city_gdp_descending
    };

    let tokyo = get_tokyo();
    let chicago = get_chicago();
    let funafuti = get_funafuti();
    let mut cities = vec![chicago, funafuti, tokyo];

    cities.sort_by_key(my_key_fn);
    println!("{:?}", cities);

    let count = count_selected_cities(&cities, over_trillion_gdp);
    println!("{} cities have over 1 trillion GDP", count);

    // 外部変数を使用していないクロージャの場合はエラーにならない。クロージャから関数への型変換が行われるらしい
    let count = count_selected_cities(&cities, |city| city.population > 100_000);
    println!("{} cities have over 100,000 population use fn", count);
    let limit = 100_000;
    // let count = count_selected_cities(&cities, |city| city.population > limit); // error[E0308]: mismatched types
    let count = count_selected_cities2(&cities, |city| city.population > limit);
    println!("{} cities have over 100,000 population use Fn", count);
}

fn count_selected_cities(cities: &Vec<City>, test_fn: fn(&City) -> bool) -> usize {
    let mut count = 0;
    for city in cities {
        if test_fn(city) {
            count += 1;
        }
    }
    count
}

fn count_selected_cities2<F>(cities: &Vec<City>, test_fn: F) -> usize
where
    F: Fn(&City) -> bool,
{
    let mut count = 0;
    for city in cities {
        if test_fn(city) {
            count += 1;
        }
    }
    count
}

fn over_trillion_gdp(city: &City) -> bool {
    city.gdp > 1_000_000_000_000
}
