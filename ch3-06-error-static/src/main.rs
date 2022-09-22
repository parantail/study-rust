// # 3.4.1 전역변수를 통한 에러 핸들링
// - 이 구조는 C 에서의 에러 처리 구조를 흉내내기 위한 코딩
// - 결과적으로는 하지 말아야한 것
//   - error handling을 쉽게 누락할 수 있음, 에러 시점과 원인을 정확히 전달하기 어려움.

use rand::random;

// static은 프로그램의 수명 동안 유효
// - static이 아닌 변수를 최상위 레벨에 선언할 방법은 없다?
static mut ERROR: i32 = 0;

#[allow(dead_code)]
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

    fn read(self: &File, save_to: &mut Vec<u8>) -> usize {
        let mut tmp = self.data.clone();
        let read_length = tmp.len();
        save_to.reserve(read_length);
        save_to.append(&mut tmp);

        // if에 있기 때문에 true/false를 랜덤 생성
        if random() && random() && random() {
            unsafe { ERROR = 1 }
        }

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
    f3.read(&mut buffer);

    // unsafe: static mut에 접근하는 것 자체가 에러라서.. 그거에 대해 개발자가 책임진다는 뜻.
    unsafe {
        if ERROR != 0 {
            panic!("An error has occurred while reading the file")
        }
    }

    close(&mut f3);

    // unsafe: static mut에 접근하는 것 자체가 에러라서.. 그거에 대해 개발자가 책임진다는 뜻.
    unsafe {
        if ERROR != 0 {
            panic!("An error has occurred while reading the file")
        }
    }
}
