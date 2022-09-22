// # File의 impl 기능 추가

// impl은 struct의 동작을 정의하는 것 (method)
// - read(f, buffer) 하던 것을
// - f.read(buffer) 로 할 수 있게 하는 것.

#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
}

// 이런 식으로 impl로 시작하는 메서드들 추가
impl File {
    // - new는 reserved가 아님. 그냥 관례로 사용하는 것.
    // 이건 말하자면 static
    fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new(),
        }
    }

    // [의문] overloading은 없나..?

    fn new_with_data(name: &str, data: &Vec<u8>) -> File {
        let mut f = File::new(name);
        f.data = data.clone();
        f
    }

    // 이렇게 self가 있어야 instance method
    fn read(self: &File, save_to: &mut Vec<u8>) -> usize {
        let mut tmp = self.data.clone();
        let read_length = tmp.len();
        save_to.reserve(read_length);
        save_to.append(&mut tmp);
        read_length
    }
}

fn open(_f: &mut File) -> bool {
    true
}

fn close(_f: &mut File) -> bool {
    true
}

fn main() {
    let f3_data: Vec<u8> = vec![114, 117, 115, 116, 33];

    let mut f3 = File::new_with_data("f3.txt", &f3_data);

    let mut buffer: Vec<u8> = vec![];

    open(&mut f3);
    let f3_length = f3.read(&mut buffer); // 이렇게 바뀜!
    close(&mut f3);

    let text = String::from_utf8_lossy(&buffer);

    println!("{:?}", f3);
    println!("{} is {} bytes long", &f3.name, f3_length);
    println!("{}", text);
}
