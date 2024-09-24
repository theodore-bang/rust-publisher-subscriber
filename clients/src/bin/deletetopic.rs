use pub_api::*;

fn main() {
    let pid = register_publisher().unwrap();

    let topic = "A Topic Name of 20B!";

    loop {
        delete_topic(pid, &topic);
    }
}