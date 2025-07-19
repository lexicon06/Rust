fn main() {
    let mut num:i32 = 0;
    loop{
        num += 1;
        println!("Line {}", num);

        if num > 19 {
            break;
        }
    }
}
