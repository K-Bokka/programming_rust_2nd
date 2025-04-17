use std::cmp::Ordering::{self, *};
use std::mem::size_of;

fn main() {
    // 10.1
    assert_eq!(compare(1, 2), Less);

    assert_eq!(size_of::<Ordering>(), 1);
    assert_eq!(size_of::<HttpStatus>(), 2);

    assert_eq!(HttpStatus::Ok as i32, 200);

    assert_eq!(http_status(200), Some(HttpStatus::Ok));
    assert_eq!(http_status(500), None);

    assert_eq!(TimeUnit::Seconds.singular(), "second");

    // 10.1.1
    let four_score_and_seven_years_ago = RoughTime::InThePast(TimeUnit::Years, 4 * 20 + 7);
    let three_hours_from_now = RoughTime::InTheFuture(TimeUnit::Hours, 3);
    let just_now = RoughTime::JustNow;
    println!("{:?}", four_score_and_seven_years_ago);
    println!("{:?}", three_hours_from_now);
    println!("{:?}", just_now);

    let unit_sphere = Shape::Sphere {
        center: Point3D::ORIGIN,
        radius: 1.0,
    };
    println!("{:?}", unit_sphere);
}

fn compare(a: i32, b: i32) -> Ordering {
    if a > b {
        Greater
    } else if a < b {
        Less
    } else {
        Equal
    }
}
#[derive(Debug, PartialEq)]
enum HttpStatus {
    Ok = 200,
    BadRequest = 400,
    NotFound = 404,
}

fn http_status(code: u32) -> Option<HttpStatus> {
    match code {
        200 => Some(HttpStatus::Ok),
        400 => Some(HttpStatus::BadRequest),
        404 => Some(HttpStatus::NotFound),
        _ => None,
    }
}

#[derive(Debug, PartialEq, Copy, Clone, Eq)]
#[allow(dead_code)]
enum TimeUnit {
    Seconds,
    Minutes,
    Hours,
    Days,
    Months,
    Years,
}

impl TimeUnit {
    fn plural(self) -> &'static str {
        match self {
            TimeUnit::Seconds => "seconds",
            TimeUnit::Minutes => "minutes",
            TimeUnit::Hours => "hours",
            TimeUnit::Days => "days",
            TimeUnit::Months => "months",
            TimeUnit::Years => "years",
        }
    }
    fn singular(self) -> &'static str {
        self.plural().trim_end_matches('s')
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum RoughTime {
    InThePast(TimeUnit, u32),
    JustNow,
    InTheFuture(TimeUnit, u32),
}
#[allow(dead_code)]
#[derive(Debug)]
struct Point3D(f32, f32, f32);

impl Point3D {
    const ORIGIN: Point3D = Point3D(0.0, 0.0, 0.0);
}
#[allow(dead_code)]
#[derive(Debug)]
enum Shape {
    Sphere {
        center: Point3D,
        radius: f32,
    },
    Cuboid {
        corner_1: Point3D,
        corner_2: Point3D,
    },
}

#[allow(dead_code)]
enum RelationShipStatus {
    Single,
    InARelationship,
    ItsComplicated(Option<String>),
    ItsExtremelyComplicated { car: String, cdr: String },
}
