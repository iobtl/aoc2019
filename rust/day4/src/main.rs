fn main() {
    let x_min: u32 = 193651;
    let x_max: u32 = 649729;
    let x_curr: u32;
    let mut count: u32 = 0;

    for x_curr in x_min..x_max {
        let mut double_compatible: bool = false;
        let mut repeated_digits: Vec<u32> = vec![];
        let mut repeated: u32 = 0;
        let mut increasing_check: u32 = 0;
        let digits: Vec<u32> = x_curr.to_string()
            .chars()
            .map(|d| d.to_digit(10).unwrap())
            .collect();

        let mut i = 0;
        while i < digits.len() {
            let digit = digits[i];
            if digit > increasing_check {
                repeated_digits.push(repeated);
                repeated = 1;
                increasing_check = digit;
            } else if digit == increasing_check {
                println!("Two adjacent digits in {} are equal", x_curr);
                repeated += 1;
                if i == digits.len() - 1 {
                    repeated_digits.push(repeated);
                }
                double_compatible = true;
            } else {
                println!("{} is not always increasing", x_curr);
                double_compatible = false;
                break;
            }
            i += 1;
        }
        if double_compatible && repeated_digits.contains(&2) {
            println!("{} is compatible", x_curr);
            count += 1;
        }
    }
    println!("Total number of compatible password combinations: {}", count);
}
