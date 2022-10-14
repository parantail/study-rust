#![allow(unused_variables)]

#[derive(Debug)]
struct CubeSat {
    id: u64,
    mailbox: Mailbox,
}

impl CubeSat {
    fn recv(&self, mailbox: &mut Mailbox) -> Option<Message> {
        mailbox.deliver(&self)
    }
}

#[derive(Debug)]
enum StatusMessage {
    Ok,
}

#[derive(Debug)]
struct Mailbox {
    messages: Vec<Message>,
}
impl Mailbox {
    fn post(&self, to: &mut CubeSat, msg: Message) {
        self.messages.push(msg);
    }

    fn deliver(&self, recipient: &CubeSat) -> Option<Message> {
        for i in 0..self.messages.len() {
            if self.messages[i].to == recipient.id {
                let msg = self.messages.remove(i);
                return Some(msg);
            }
        }

        None
    }
}

#[derive(Debug)]
struct Message {
    to: u64,
    content: String,
}

struct GroundStation;

impl GroundStation {
    fn send(&self, mailbox: &mut Mailbox, to: &mut CubeSat, msg: Message) {
        mailbox.post(to, msg);
    }

    fn connect(&self, sat_id: u64) -> CubeSat {
        CubeSat {
            id: sat_id,
            mailbox: Mailbox { messages: vec![] },
        }
    }
}

fn check_status(sat_id: CubeSat) -> CubeSat {
    println!("{:?}: {:?}", sat_id, StatusMessage::Ok);

    sat_id
}

fn fetch_sat_ids() -> Vec<u64> {
    vec![1, 2, 3]
}

fn main() {
    let base = GroundStation {};

    let sat_a = CubeSat {
        id: 0,
        mailbox: Mailbox { messages: vec![] },
    };
    let sat_b = CubeSat {
        id: 1,
        mailbox: Mailbox { messages: vec![] },
    };
    let sat_c = CubeSat {
        id: 2,
        mailbox: Mailbox { messages: vec![] },
    };

    let mut sat_a = check_status(sat_a);
    let sat_b = check_status(sat_b);
    let sat_c = check_status(sat_c);
    println!("a: {:?}, b: {:?}, c: {:?}", sat_a, sat_b, sat_c);

    base.send(&mut sat_a, Message::from("Hello"));

    let mut sat_a = check_status(sat_a);
    let sat_b = check_status(sat_b);
    let sat_c = check_status(sat_c);
    println!("a: {:?}, b: {:?}, c: {:?}", sat_a, sat_b, sat_c);

    let msg = sat_a.recv();
    println!("a: {:?}", sat_a);
    println!("msg: {:?}", msg);

    let sat_ids = fetch_sat_ids();

    for sat_id in sat_ids {
        let mut sat = base.connect(sat_id);

        base.send(&mut sat, Message::from("hello"));
    }
}
