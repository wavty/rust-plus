mod fib;
mod fig;
mod readfile;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_message() {
        let event = Event::Message(UserId(1), TopicId(1), "Hello, world!".into());
        if let Event::Message(_, _, msg) = event {
            println!("msg: {}", msg);
        }
    }

    #[test]
    fn test_main() {
        let alice = User {
            id: UserId(1),
            name: "Alice".into(),
            gender: Gender::Female,
        };
        let bob = User {
            id: UserId(2),
            name: "Bob".into(),
            gender: Gender::Male,
        };

        let topic = Topic {
            id: TopicId(1),
            name: "rust".into(),
            owner: alice.id,
        };
        let event1 = Event::Join(alice.id, topic.id);
        let event2 = Event::Join(bob.id, topic.id);
        let event3 = Event::Message(alice.id, topic.id, "Hello, world!".into());
        let event4 = Event::Leave(bob.id, topic.id);

        println!(
            "event1: {:?}, event2: {:?}, event3: {:?}, event4: {:?}",
            event1, event2, event3, event4
        );

        process_event(&event1);
        process_event(&event2);
        process_event(&event3);

        println!("{} {} {}", topic.id.0, topic.name, topic.owner.0);
        println!("{} {} {:?}", alice.id.0, alice.name, alice.gender);
        println!("{:?}", Gender::Unspecified);
    }
}

#[derive(Debug)]
pub enum Gender {
    Unspecified = 0,
    Female = 1,
    Male = 2,
}

#[derive(Debug, Copy, Clone)]
pub struct UserId(u64);

#[derive(Debug, Copy, Clone)]
pub struct TopicId(u64);

#[derive(Debug)]
pub struct User {
    id: UserId,
    name: String,
    gender: Gender,
}

#[derive(Debug)]
pub struct Topic {
    id: TopicId,
    name: String,
    owner: UserId,
}

#[derive(Debug)]
pub enum Event {
    Join(UserId, TopicId),
    Leave(UserId, TopicId),
    Message(UserId, TopicId, String),
}

pub fn process_event(event: &Event) {
    match event {
        Event::Join(user_id, topic_id) => {
            println!("user {} joined topic {}", user_id.0, topic_id.0);
        }
        Event::Leave(user_id, topic_id) => {
            println!("user {} left topic {}", user_id.0, topic_id.0);
        }
        Event::Message(user_id, topic_id, message) => {
            println!(
                "user {} left topic {} with message: {}",
                user_id.0, topic_id.0, message
            );
        }
    }
}

pub fn create_user(id: UserId, name: String, gender: Gender) -> User {
    User { id, name, gender }
}

pub fn create_topic(id: TopicId, name: String, owner: UserId) -> Topic {
    Topic { id, name, owner }
}
