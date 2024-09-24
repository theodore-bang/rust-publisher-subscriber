use pub_api::*;
use sub_api::*;

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

    // Test that subs can subscribe to topics

    // Test that pubs can send messages
    
    // Test that subs can pull messages

    // Test that pubs can send new messages

    // Test that subs can pull new messages

    // Test that sub will get messages other subs saw

    // Subs get empty messages from invalid topics, or if no new messages


}