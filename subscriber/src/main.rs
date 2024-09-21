use common;
use sub_api;

fn main() {
    println!("I am a subscriber! {}", sub_api::add(400, 20));
}
