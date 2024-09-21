use common::Pid;

pub fn register_publisher() -> Option<Pid> {
    todo!()
}

pub fn create_topic(pid: Pid, topic: String) {
    todo!()
}

pub fn delete_topic(pid: Pid, topic: String) {
    todo!()
}

pub fn send(pid: Pid, topic: String, message: String) {
    todo!()
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
