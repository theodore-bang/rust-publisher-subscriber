use std::collections::HashMap;

use common::{Sid, Pid};

pub struct Broker {
    sid_generator: Sid,
    pid_generator: Pid,
    sub_list: Vec<Sid>,
    pub_list: Vec<Sid>,
    topics: HashMap<String, Topic>,
}

impl Broker {
    pub fn new() -> Self {
        Self {
            sid_generator: 0,
            pid_generator: 0,
            sub_list: Vec::new(),
            pub_list: Vec::new(),
            topics: HashMap::new(),
        }
    }

    pub fn check_sid(&self, sid: Sid) -> bool {
        self.sub_list.contains(&sid)
    }
    pub fn check_pid(&self, pid: Pid) -> bool {
        self.sub_list.contains(&pid)
    }

    pub fn register_sub(&mut self) -> Sid {
        self.sid_generator += 1;
        self.sub_list.push(self.sid_generator);
        println!("Registering SID: {}", self.sid_generator);
        self.sid_generator.clone()
    }
    pub fn register_pub(&mut self) -> Pid {
        self.pid_generator += 1;
        self.pub_list.push(self.pid_generator);
        println!("Registering PID: {}", self.pid_generator);
        self.pid_generator.clone()
    }

    pub fn create_topic(&mut self, pid: Pid, topic_name: String) {
        let new_topic = Topic {
            topic_name: topic_name.clone(),
            publisher: pid,
            subs_list: Vec::new(),
            messages: Vec::new(),
        };

        self.topics.insert(topic_name.clone(), new_topic);
        println!("Created topic: {}", topic_name);
    }

    pub fn delete_topic(&mut self, pid: Pid, topic_name: String) {
        let Some(found) = self.topics.get(&topic_name) else { return () };

        if found.publisher == pid {
            self.topics.remove(&topic_name);
        }
        println!("Deleted topic: {}", topic_name);
    }

    pub fn add_message(&mut self, topic_name: String, content: String) {
        let Some(found) = self.topics.get_mut(&topic_name) else {return ()};
        found.new_message(content);
        println!("Added message to topic: {}", topic_name);
    }

    pub fn subscribe(&mut self, sid: Sid, topic_name: String) {
        let Some(found) = self.topics.get_mut(&topic_name) else {return ()};
        found.subscribe(sid);
    }

    pub fn pull(&mut self, sid: Sid, topic_name: String) -> Vec<String> {
        // println!("Pulling \"{}\" messages for {}", topic_name, sid);
        if let Some(topic) = self.topics.get_mut(&topic_name) {
            println!("Found topic");
            return topic.get_messages(sid);
        } else {
            println!("Failed to find topic");
            vec![]
        }
    }

}

pub struct Topic {
    pub topic_name: String,
    pub publisher: Pid,
    pub subs_list: Vec<Sid>,
    pub messages: Vec<Message>,

}

#[derive(Debug, Clone)]
pub struct Message {
    pub content: String,
    pub not_seen: Vec<Sid>,
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
        
        output
    }
}