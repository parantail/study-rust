use core::fmt;
use std::fmt::Display;

#[derive(Debug)]
struct File {
    name: String,
    state: FileState,
}
impl File {
    fn new(name: &str) -> File {
        File {
            name: String::from(name),
            state: FileState::Closed,
        }
    }
}

// impl Trait for Type
// "{}" 이것을 위한 구현
impl Display for File {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<File={} ({})>", self.name, self.state)
    }
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
enum FileState {
    Open,
    Closed,
}
impl Display for FileState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FileState::Open => write!(f, "열림"),
            FileState::Closed => write!(f, "닫힘"),
        }
    }
}

// trait의 정의(?) - interface 정의와 유사
trait Read {
    fn read(
        self: &Self, // Self는 '구현하는 타입에 대한 자리 표시자'
        _save_to: &mut Vec<u8>,
    ) -> Result<usize, String>;
}

// (OOP에서의) abstract, interface를 implements에 해당하는 구현
impl Read for File {
    fn read(
        self: &File, // 이렇게 구체화 되어 쓰임
        _save_to: &mut Vec<u8>,
    ) -> Result<usize, String> {
        Ok(0)
    }
}

fn main() {
    let f = File::new("ch3-10.txt");
    let mut buffer = vec![];
    let n_bytes = f.read(&mut buffer).unwrap();
    println!("{} byte(s) read from", n_bytes);
    println!("{:?}", f); // 이건 Debug
    println!("{}", f); // 이건 Display
}
