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
