use pub_api::*;
use sub_api::*;

fn main() {
    let topic = "A Topic Name of 20B!";
    let pid = pub_api::register_publisher().unwrap();
    create_topic(pid, topic);
    let sid = sub_api::register_subscriber().unwrap();
    subscribe(sid, topic);

    loop {
        let _ = pull(sid, topic);
    }
}