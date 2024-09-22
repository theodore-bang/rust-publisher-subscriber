use common;
use pub_api;

fn main() {
    let my_pid = pub_api::register_publisher().unwrap();
    pub_api::create_topic(my_pid, "FooBar".to_string());
}
