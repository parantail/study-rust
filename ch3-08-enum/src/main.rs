// # 3.5 열거형! (enum)

// 어느 언어에서나 흔히 볼 수 있는 열거자 선언과 비슷.
#[derive(Debug)] // 역시 이걸 해 줘야 {:?} 이걸로 프린트 할 수 있음
enum Event {
    Update,
    Delete,
    Unknown,
}
// impl도 할 수 있음

// 로그를 분석하는 코드 예시

type Message = String;

fn parse_log(line: &str) -> (Event, Message) {
    let parts: Vec<_> = line // _ 는 Rust에게 직접 타입을 찾아내라는 뜨
        .splitn(2, ' ')
        .collect(); // iterator를 collection으로 변경해 줌.
                    // [의문] collect는 general한데 언제 Vec으로 변환되지? Rust가 해주나? VecDeque를 해보니.. ㅇ_ㅇ
    if parts.len() == 1 {
        // 여기서는 Error를 enum 값중 하나로 처리..
        return (Event::Unknown, String::from(line));
    }

    let event = parts[0];
    let rest = String::from(parts[1]);

    // match와 enum의 만남.
    match event {
        "UPDATE" | "update" => (Event::Update, rest), // <9>
        "DELETE" | "delete" => (Event::Delete, rest), // <9>
        _ => (Event::Unknown, String::from(line)),    // <10>
    }
}

fn main() {
    let log = "BEGIN Transaction XK342
UPDATE 234:LS/32231 {\"price\": 31.00} -> {\"price\": 40.00}
DELETE 342:LO/22111";

    for line in log.lines() {
        let parse_result = parse_log(line);
        println!("{:?}", parse_result);
    }
}
