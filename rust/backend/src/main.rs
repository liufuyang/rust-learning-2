#![feature(plugin)]
#![plugin(rocket_codegen)]

use rocket::State;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

struct SomeObj {
    count: Mutex<i64>,
}

impl SomeObj {
    fn add(&self) {
        // self.count += 1
        let mut _count_ref = self.count.lock().unwrap();
        let mut _count = *_count_ref;
        // thread::sleep(Duration::from_millis(10));
        _count += 1;
        *_count_ref = _count;
    }
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/count")]
fn count(obj: State<SomeObj>) -> String {
    // let current_count = obj.count;
    let current_count = obj.count.lock().unwrap();
    format!("Number of visits: {}", current_count)
}

#[get("/add")]
fn add(obj: State<SomeObj>) -> &'static str {
    obj.add();
    "Shared obj added 1"
}

fn main() {
    rocket::ignite()
        .manage(SomeObj {
            count: Mutex::new(0),
        }).mount("/", routes![index, count, add])
        .launch();
}
