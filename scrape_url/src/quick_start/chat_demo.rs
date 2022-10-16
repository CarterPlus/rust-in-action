#[derive(Debug)]
enum Gender {
    Unspecified = 0,
    Female = 1,
    Male = 2,
}

#[derive(Debug, Copy, Clone)]
struct UserId(u64);

#[derive(Debug, Copy, Clone)]
struct TopicId(u64);

struct User {
    id: UserId,
    name: String,
    gender: Gender,
}

#[derive(Debug)]
struct Topic {
    id: TopicId,
    name: String,
    owner: UserId,
}

// 定义聊天室中可能发生的事情
enum Event {
    Join((UserId, TopicId)),
    Leave((UserId, TopicId)),
    Message((UserId, TopicId, String)),
}

pub fn chatStart() {
    let alice = User {id: UserId(1), name: "Alice".into(), gender: Gender::Female};
    let bob = User {id: UserId(1), name: "Bob".into(), gender: Gender::Male};

    let topic = Topic {id: TopicId(1), name: "rust".into(), owner: UserId(1)};
    let event1 = Event::Join((alice.id, topic.id));
    let evnet2 = Event::Join((bob.id, topic.id));
    let event3 = Event::Message((alice.id, topic.id, "Hello World!".into()));

    println!("event1: {:?}, event2: {:?}, event3: {:?}", event1, evnet2, event3);
}

fn process_event(event: &Event) {
    match event {
        Event::Join((uid, _tid)) => println!("user {:?} joined", uid),
        Event::Leave((uid, tid)) => println!("user {:?} left {:?}", uid, tid),
        Event::Message((_, _, msg)) => println!("broadcase: {}", msg),
    }
}

fn process_message(event: &Event) {
    if let Event::Message((_, _, msg)) = event {
        println!("broadcase: {}", msg);
    }
}