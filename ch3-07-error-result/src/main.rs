// # 3.4.2 Result를 사용한 Error Handling

use rand::prelude::*; // rand의 모든 것을 사용하겠다는 선언

// (1 / denominator)의 확률로 true를 반환
fn one_in(denominator: u32) -> bool {
    thread_rng().gen_ratio(1, denominator)
}

#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
}
impl File {
    fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new(),
        }
    }

    fn new_with_data(name: &str, data: &Vec<u8>) -> File {
        let mut f = File::new(name);
        f.data = data.clone();
        f
    }

    fn read(self: &File, save_to: &mut Vec<u8>) -> Result<usize, String> {
        let mut tmp = self.data.clone();
        let read_length = tmp.len();
        save_to.reserve(read_length);
        save_to.append(&mut tmp);
        Ok(read_length)
    }
}

// (기존과 다르게) f가 참조가 아니고 ownership 이전됨
// Result는 Ok(File)이거나, Err(String)
fn open(f: File) -> Result<File, String> {
    if one_in(10_000) {
        let err_msg = String::from("Permission denied");
        return Err(err_msg);
    }
    Ok(f)
}

fn close(f: File) -> Result<File, String> {
    if one_in(100_000) {
        let err_msg = String::from("Interrupted by signal!");
        return Err(err_msg);
    }
    Ok(f)
}

fn main() {
    let f4_data: Vec<u8> = vec![114, 117, 115, 116, 33];

    let mut f4 = File::new_with_data("f4.txt", &f4_data);

    let mut buffer: Vec<u8> = vec![];

    // 이제 이 콜은 소유권을 넘겨줬다가 리턴받음 (리턴을 안 받으면?)
    // unwrap: Result가 Ok이면 그 안의 값을 가져오고, Err면 panic
    f4 = open(f4).unwrap();
    let f4_length = f4.read(&mut buffer).unwrap();
    f4 = close(f4).unwrap();

    // 여전히 멋진 에러 처리 방식은 아니지만..
    // 아무튼 컴파일러가 그 핸들링을 체크할 수 있는 수준까지는 만들었음.

    let text = String::from_utf8_lossy(&buffer);

    println!("{:?}", f4);
    println!("{} is {} bytes long", &f4.name, f4_length);
    println!("{}", text);
}
