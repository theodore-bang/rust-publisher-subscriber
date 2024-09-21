use common::{Sid, Messages, Stub, Procedures};

pub fn register_subscriber() -> Option<Sid> {
    todo!()
}

pub fn subscribe(sid: Sid, topic: String) {
    todo!()
}

pub fn pull(sid: Sid, topic: String) -> Messages {
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
