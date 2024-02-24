fn is_palindrome(i: u32) -> bool {
    i.to_string() == i.to_string().chars().rev().collect::<String>()
}

fn main() {
    let mut result = 0;

    for i in 100..1000 {
        for j in 100..1000 {
            let tmp = i * j;
            if is_palindrome(tmp) {
                result = result.max(tmp)
            }
        }
    }

    println!("{}", result)
}
