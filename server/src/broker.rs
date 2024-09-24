// MESSAGE BROKER or MESSAGE BUS //
/*
// The `Broker` stucture holds all the topics and messages of the server,
// plus information about registered subscribers and publishers.
// There is one instance of a Broker on the server, and individual threads
// that are handling client requests access the Broker through an Arc Pointer
// and RwLock Mutex.
//
// The `Topic` stucture holds information about a specific topic.
// Topics are unique by their name. Topics hold a list of `Message` objects.
//
// A `Message` stucture holds a list of String `content` and a list of
// subscribers (`not_seen`) who have not yet seen the message.
// Messages get deleted after a client requests a pull, by checking if
// the `not_seen` list is empty.
*/

use std::collections::HashMap;
use common::{Sid, Pid};
use rand::Rng;

// Broker Data Structure //
pub struct Broker {
    sub_list: Vec<Sid>,
    pub_list: Vec<Pid>,
    topics: HashMap<String, Topic>,
}

// Broker Methods //
impl Broker {
    pub fn new() -> Self {
        Self {
            sub_list: Vec::new(),
            pub_list: Vec::new(),
            topics: HashMap::new(),
        }
    }

    // Check that SID is actually a subscriber //
    pub fn check_sid(&self, sid: Sid) -> bool {
        self.sub_list.contains(&sid)
    }
    // Check that PID is actually a subscriber //
    pub fn check_pid(&self, pid: Pid) -> bool {
        self.pub_list.contains(&pid)
    }

    pub fn register_sub(&mut self) -> Sid {
        let new_sid: u64 = rand::thread_rng().gen();
        self.sub_list.push(new_sid);
        println!("Server: registering SID {}", new_sid);
        new_sid
    }
    pub fn register_pub(&mut self) -> Pid {
        let new_pid: u64 = rand::thread_rng().gen();
        self.pub_list.push(new_pid);
        println!("Server: registering SID {}", new_pid);
        new_pid
    }

    pub fn create_topic(&mut self, pid: Pid, topic_name: String) {
        // If pid is not of registered Publisher, do nothing //
        if !self.check_pid(pid) {
            println!("Server: {} is not a publisher! {:?}", pid, self.pub_list);
            return ();
        }
        let new_topic = Topic {
            topic_name: topic_name.clone(),
            publisher: pid,
            subs_list: Vec::new(),
            messages: Vec::new(),
        };

        self.topics.insert(topic_name.clone(), new_topic);
        println!("Server: created topic \"{}\"", topic_name);
    }

    pub fn delete_topic(&mut self, pid: Pid, topic_name: String) {
        // If topic is not found, do nothing //
        let Some(found) = self.topics.get(&topic_name) else {
            println!("Server: topic not found");
            return () 
        };

        // If pid is not that of Topic's creator, do nothing //
        if found.publisher == pid {
            self.topics.remove(&topic_name);
            println!("Server: deleted topic \"{}\"", topic_name);
        }
    }

    pub fn add_message(&mut self, pid: Pid, topic_name: String, content: String) {
        // If pid is not of registered Publisher, do nothing //
        if !self.check_pid(pid) {
            return ();
        }
        let Some(topic) = self.topics.get_mut(&topic_name) else {return ()};
        topic.new_message(content);
        println!("Server: added message to topic: {}", topic_name);
    }

    pub fn subscribe(&mut self, sid: Sid, topic_name: String) {
        // If sid is not of registered Subscriber, do nothing //
        if !self.check_sid(sid) {
            return ();
        }
        let Some(found) = self.topics.get_mut(&topic_name) else {return ()};
        found.subscribe(sid);
    }

    pub fn pull(&mut self, sid: Sid, topic_name: String) -> Vec<String> {
        // If sid is not of registered Subscriber, return empty list //
        if !self.check_sid(sid) {
            return vec![];
        }
        // println!("Pulling \"{}\" messages for {}", topic_name, sid);
        if let Some(topic) = self.topics.get_mut(&topic_name) {
            println!("Server: pulling {} messages for {sid}", &topic.topic_name);
            return topic.get_messages(sid);
        } else {
            println!("Server: failed to find topic");
            vec![]
        }
    }

}

// Topic Structure //
/*
 - publisher: Pid of topic creator
 - subs_list: list of current subscribers
*/
pub struct Topic {
    pub topic_name: String,
    pub publisher: Pid,
    pub subs_list: Vec<Sid>,
    pub messages: Vec<Message>,

}
impl Topic {
    pub fn subscribe(&mut self, sid: Sid) {
        self.subs_list.push(sid);
    }
    
    pub fn new_message(&mut self, content: String) {
        let new_message = Message {
            content: content.clone(),
            not_seen: self.subs_list.clone(),
        };

        self.messages.push(new_message);
    }

    pub fn get_messages(&mut self, sid: Sid) -> Vec<String> {
        let mut output = Vec::new();

        
        for message in self.messages.iter_mut() {
            if message.not_seen.contains(&sid) {
                // println!("Message found");
                output.push(message.content.clone());
            }
            message.not_seen.retain(|&a_sid| a_sid != sid );
        }

        self.messages.retain(|msg| !msg.not_seen.is_empty() );
        
        output
    }
}

// Message Structure //
#[derive(Debug, Clone)]
pub struct Message {
    pub content: String,
    pub not_seen: Vec<Sid>,
}