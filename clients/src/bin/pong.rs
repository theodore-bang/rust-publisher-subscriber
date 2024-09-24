
use sub_api;
use pub_api;

fn main() {
    let my_topic = "pong".to_string();
    let other_topic = "ping".to_string();

    let message = "PONG".to_string();

    let pid = pub_api::register_publisher().unwrap();
    let sid = sub_api::register_subscriber().unwrap();

    pub_api::create_topic(pid, &my_topic);

    sleep(1);

    sub_api::subscribe(sid, &other_topic);

    sleep(1);
    

    loop {
        let _msgs = sub_api::pull(sid, &other_topic);

        // println!("SID {sid} received: {:?}", msgs);

        pub_api::send(pid, &my_topic, &message);
    }
}

fn sleep(secs: u64) {
    std::thread::sleep(std::time::Duration::from_secs(secs));
}