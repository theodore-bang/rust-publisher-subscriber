use sub_api;

fn main() {
    let topic1 = "My First Topic";
    let topic2 = "My Second Topic";

    // Register //
    let sid = sub_api::register_subscriber().unwrap();

    // Wait for publisher to publish topics and messages //
    std::thread::sleep(std::time::Duration::from_secs(3));

    // Subscribe to first topic //
    sub_api::subscribe(sid, &topic1);
    sub_api::subscribe(sid, &topic2);

    // Print messages from first topic //
    let my_msgs = sub_api::pull(sid, &topic1);
    for msg in my_msgs {
        println!("Subscriber: message received: {}", msg);
    }

    // Try getting messages from second topic //
    let my_msgs = sub_api::pull(sid, &topic2);
    for msg in my_msgs {
        println!("Subscriber: message received: {}", msg);
    }

}