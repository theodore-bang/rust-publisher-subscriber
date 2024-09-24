
use pub_api::*;
use sub_api::*;

fn sleep(secs: u64) {
    std::thread::sleep(std::time::Duration::from_secs(secs));
}
fn main() {
    let first_topic = "FIRST TOPIC";
    let second_topic = "SECOND TOPIC";
    let changed_owner = "TOPIC CHANGED OWNER";
    let deleted_topic = "DELETED TOPIC";
    
    // Test we can registers subs and pubs //
    let pub1 = register_publisher().unwrap();
    let pub2 = register_publisher().unwrap();
    let sub1 = register_subscriber().unwrap();
    let sub2 = register_subscriber().unwrap();

    // Test pubs can create topics and delete topics
    create_topic(pub1, &first_topic);
    create_topic(pub1, &deleted_topic);

    sleep(1);

    delete_topic(pub1, &deleted_topic);
    create_topic(pub2, &second_topic);

    // Topic will change owner if Publisher tries to create topic
    // with existing name
    create_topic(pub2, &changed_owner);
    create_topic(pub1, &changed_owner);

    // Test that subs can subscribe to topics
    subscribe(sub1, &first_topic);
    subscribe(sub1, &second_topic);
    subscribe(sub2, &first_topic);
    subscribe(sub2, &second_topic);

    // Test that pubs can send messages
    send(pub1, &first_topic, &"first message");
    send(pub1, &first_topic, &"second message");
    send(pub1, &first_topic, &"third message");
    send(pub2, &second_topic, &"first message");
    send(pub2, &second_topic, &"second message");

    sleep(1);
    
    // Test that subs can pull messages
    let sub1_msgs1 = pull(sub1, &first_topic);
    let sub1_msgs2 = pull(sub1, &second_topic);

    for msg in sub1_msgs1 {
        println!("{sub1} received: {msg} from {first_topic}");
    }
    for msg in sub1_msgs2 {
        println!("{sub1} received: {msg} from {second_topic}");
    }

    // Test that pubs can send new messages
    send(pub1, &first_topic, &"more message");
    send(pub1, &first_topic, &"even more messages");

    sleep(1);

    // Test that subs can pull new messages
    let sub1_msgs1 = pull(sub1, &first_topic);
    let sub1_msgs2 = pull(sub1, &second_topic);

    for msg in sub1_msgs1 {
        println!("{sub1} received: {msg} from {first_topic}");
    }
    for msg in sub1_msgs2 {
        println!("{sub1} received: {msg} from {second_topic}");
    }

    // Test that sub will get messages other subs saw
    let sub2_msgs1 = pull(sub2, &first_topic);
    for msg in sub2_msgs1 {
        println!("{sub2} received: {msg} from {first_topic}");
    }

    // Subs get empty messages if no new messages
    let sub1_msgs1 = pull(sub1, &first_topic);

    for msg in sub1_msgs1 {
        println!("{sub1} received: {msg} from {first_topic}");
    }

}