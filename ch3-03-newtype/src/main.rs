// # newtype 패턴에 대한 설명
// - type alias 와 유사한 효과를 내면서, 완전히 별개의 타입으로 취급되게 하는 방법
// - "부정확한 문맥에서 묵시적으로 사용되는 것을 방지하여 프로그램을 견고하게 만들 수 있다."
// - 단점은.. 모든 동작을 다 구현을 또 해줘야한다..

struct Hostname(String);

//type Hostname = String;   // 이렇게 하면 String을 쓰든 Hostname을 쓰든 프로그램이 정상 동자

fn connect(host: Hostname) {
    println!("connected to {}", host.0);
}

fn main() {
    let ordinary_string = String::from("localhost");
    //connect(ordinary_string); // 이것 불가

    let host = Hostname(ordinary_string.clone());

    connect(host);
}
