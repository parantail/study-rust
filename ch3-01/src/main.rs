// 이걸 없애면 사용하지 않는 variable이 있다는 경고가 나옵니다.
// intentional하면 _ 를 붙이면 됨.
#![allow(unused_variables)]

type File = String; // String을 File로 Alias해서 사용하기

fn open(f: &mut File) -> bool {
    true
}

fn close(f: &mut File) -> bool {
    true
}

// 사용되지 않는 코드에 대한 경고를 지우기 위해 사용함
#[allow(dead_code)]
fn read(f: &mut File, save_to: &mut Vec<u8>) -> ! {
    unimplemented!() // panic으로 종료시켜버림
}

fn main() {
    let mut f1 = File::from("f1.txt");
    open(&mut f1);
    // let mut v = Vec::new();
    // read(&mut f1, &mut v);
    close(&mut f1);
}
