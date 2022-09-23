// 3.5.1 파일 상태에 enum 사용

// [생각] 가장 전형적인 enum의 활용 방식

#[derive(Debug, PartialEq)] // PartialEq가 있어야 state != Open 같은 비교 연산이 가능
// Partial 이라서 NAN, NULL 같은.. "동일한 값으로 취급되어서는 안 되는 경우에 쓰일 수 있다."
// For example, in floating point numbers NaN != NaN, so floating point types implement PartialEq but not Eq.

enum FileState {
    Open,
    Closed,
}

#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
    state: FileState, // 이런 식으로 상태를 활용하는 예
}

impl File {
    fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new(),
            state: FileState::Closed,
        }
    }

    fn read(self: &File, save_to: &mut Vec<u8>) -> Result<usize, String> {
        if self.state != FileState::Open {
            return Err(String::from("File must be open for reading"));
        }
        let mut tmp = self.data.clone();
        let read_length = tmp.len();
        save_to.reserve(read_length);
        save_to.append(&mut tmp);
        Ok(read_length)
    }
}

fn open(mut f: File) -> Result<File, String> {
    f.state = FileState::Open;
    Ok(f)
}

fn close(mut f: File) -> Result<File, String> {
    f.state = FileState::Closed;
    Ok(f)
}

fn main() {
    let mut f5 = File::new("f5.txt");

    let mut buffer: Vec<u8> = vec![];

    // is_err는 Result가 Err인지 체크. (panic 내지 않고 처리)
    if f5.read(&mut buffer).is_err() {
        println!("Error checking is working");
    }

    f5 = open(f5).unwrap();
    let f5_length = f5.read(&mut buffer).unwrap();
    f5 = close(f5).unwrap();

    let text = String::from_utf8_lossy(&buffer);

    println!("{:?}", f5);
    println!("{} is {} bytes long", &f5.name, f5_length);
    println!("{}", text);
}
