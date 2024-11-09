// Add a macro to make the test pass.

fn main() {
    println!("Hello, world!");
}

fn average(nums: &[i32]) -> i32 {
    if nums.len() == 0 {
        panic!("Empty number list");
    }
    let mut sum = 0;
    for num in nums {
        sum += num;
    }
    sum / nums.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic] // this macro is used to check if the function panics
    fn it_panics() {
        let nums = [];
        let _avg = average(&nums);
    }
}
