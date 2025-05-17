use std::collections::HashMap;

pub fn run() -> () {
    println!("Callback");

    let mut router = BasicRouter::new();
    router.add_route("/", |_| get_form_res());
    // router.add_route("/new", |_| get_form_res()); // error[E0308]: mismatched types

    let mut boxed_router = BoxedBasicRouter::new();
    boxed_router.add_route("/", |_| get_form_res());
    boxed_router.add_route("/404", |_| not_found());

    fn add_ten(x: u32) -> u32 {
        x + 10
    }
    let fn_ptr: fn(u32) -> u32 = add_ten;
    let eleven = fn_ptr(1);
    println!("eleven: {}", eleven);

    let closure_ptr: fn(i32) -> i32 = |x| x + 1;
    let two = closure_ptr(1);
    println!("two: {}", two);

    let mut fn_pointer_router = FnPointerRouter::new();
    fn_pointer_router.add_route("/", |_| get_form_res());
    fn_pointer_router.add_route("/404", |_| not_found());
}

fn get_form_res() -> Response {
    Response {
        code: 200,
        headers: HashMap::new(),
        body: vec![],
    }
}

#[allow(dead_code)]
struct Request {
    method: String,
    url: String,
    headers: HashMap<String, String>,
    body: Vec<u8>,
}

#[allow(dead_code)]
struct Response {
    code: u32,
    headers: HashMap<String, String>,
    body: Vec<u8>,
}

struct BasicRouter<C>
where
    C: Fn(Request) -> Response,
{
    routes: HashMap<String, C>,
}

impl<C> BasicRouter<C>
where
    C: Fn(Request) -> Response,
{
    fn new() -> Self {
        Self {
            routes: HashMap::new(),
        }
    }

    fn add_route(&mut self, url: &str, callback: C) {
        self.routes.insert(url.to_string(), callback);
    }
}

type BoxedCallback = Box<dyn Fn(&Request) -> Response>;
struct BoxedBasicRouter {
    routes: HashMap<String, BoxedCallback>,
}

impl BoxedBasicRouter {
    fn new() -> Self {
        Self {
            routes: HashMap::new(),
        }
    }

    fn add_route<C>(&mut self, url: &str, callback: C)
    where
        C: Fn(&Request) -> Response + 'static,
    {
        self.routes.insert(url.to_string(), Box::new(callback));
    }

    #[allow(dead_code)]
    fn handle_request(&self, request: &Request) -> Response {
        match self.routes.get(&request.url) {
            None => not_found(),
            Some(callback) => callback(request),
        }
    }
}

fn not_found() -> Response {
    Response {
        code: 404,
        headers: HashMap::new(),
        body: vec![],
    }
}

struct FnPointerRouter {
    routes: HashMap<String, fn(&Request) -> Response>,
}

impl FnPointerRouter {
    fn new() -> Self {
        Self {
            routes: HashMap::new(),
        }
    }

    fn add_route(&mut self, url: &str, callback: fn(&Request) -> Response) {
        self.routes.insert(url.to_string(), callback);
    }
}
