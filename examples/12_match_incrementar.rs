fn main(){
    println!("{}", increment_option(Some(5)));
    println!("{}", increment_option(None));
}

fn increment_option(num: Option<i32>) -> i32 {
    let result = match num {
        Some(value) => value + 1,
        none => 0,
    };
    result
}