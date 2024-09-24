use pub_api;

fn main() {
    let pid = pub_api::register_publisher().unwrap();

    let topic = "A Topic Name of 20B!".to_string();

    loop {
        pub_api::create_topic(pid, &topic);
    }
}