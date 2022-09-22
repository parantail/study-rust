// # struct의 소개와, 사용 방법에 대한 이해
// - struct는 다른 언어(C, C#...)의 struct와 비슷한 역할

#[derive(Debug)] // 이것만 해 주면 struct를 알아서 string 으로 바꿔줌
struct File {
    // 내부적으로 이 String, Vec<u8>은 별도의 메모리에 존재하고
    // 이 struct는 해당 메모리를 포인터로 참조합니다.
    // primitive도 그런가..? rust에 primitive라는 구분이 있긴 한가...?
    name: String,
    data: Vec<u8>,
}

fn main() {
    // 이런식으로 struct를 instantiate해서 사용합니다.
    let f1 = File {
        name: String::from("f1.txt"),
        data: Vec::new(),
    };

    let f1_name = &f1.name; // f1_name은 f1.name을 borrow함
    let f1_length = &f1.data.len();

    let f2_name = &f1.name; // borrow는 중복해서 할 수 있습니다.

    println!("{:?}", f1);
    println!("{} is {} bytes long", f1_name, f1_length);
    println!("{} is {} bytes long", f2_name, f1_length);
}
