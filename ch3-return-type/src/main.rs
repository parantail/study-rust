// () 이것은 void와 같은 뜻. (unit type이라고 부름)
fn clear(text: &mut String) -> () {
    *text = String::from("");
}

// // 흔한 실수: 마지막 리턴 값에 ; 를 붙이면..
// fn dead_end_error() -> u32 {
//     3;
// }

// ! 이건 이 함수가 절대 리턴되지 않는다는 뜻 (never라고 부름)
fn dead_end() -> ! {
    loop {
        println!("looping")
    }
}

fn main() {
    // () 설명
    let mut s = String::from("");
    clear(&mut s);

    // ! 설명
    dead_end();
}
