use pub_api;
use sub_api;

fn main() {
    let pid = pub_api::register_publisher().unwrap();

    let topic1 = "My First Topic".to_string();
    let topic2 = "My Second Topic".to_string();
    let topic3 = "My Third Topic".to_string();
    
    let message1 = "My first message!".to_string();
    let message2 = "My second message!".to_string();
    let message3 = "My third message!".to_string();
    let message4 = "My fourth message!".to_string();

    pub_api::create_topic(pid, topic1.clone());

    pub_api::create_topic(pid, topic2.clone());
    pub_api::send(pid, topic2.clone(), message1.clone());
    pub_api::send(pid, topic2.clone(), message2.clone());
    pub_api::send(pid, topic2.clone(), message3.clone());

    let sid = sub_api::register_subscriber().unwrap();

    // Wait for publisher to publish topics and messages //
    std::thread::sleep(std::time::Duration::from_secs(3));

    // Subscribe to first topic //
    sub_api::subscribe(sid, topic1.clone());
    sub_api::subscribe(sid, topic2.clone());

    pub_api::send(pid, topic1.clone(), message1.clone());
    pub_api::send(pid, topic2.clone(), message1.clone());
    pub_api::send(pid, topic2.clone(), message2.clone());

    // Print messages from first topic //
    let my_msgs = sub_api::pull(sid, topic1.clone());
    println!("From topic 1:");
    for msg in my_msgs {
        println!("Message received: {}", msg);
    }

    // Try getting messages from second topic //
    let my_msgs = sub_api::pull(sid, topic2.clone());
    println!("From topic 2:");
    for msg in my_msgs {
        println!("Message received: {}", msg);
    }

    pub_api::send(pid, topic1.clone(), message3.clone());

    // Print messages from first topic //
    let my_msgs = sub_api::pull(sid, topic1.clone());
    println!("From topic 1:");
    for msg in my_msgs {
        println!("Message received: {}", msg);
    }
}