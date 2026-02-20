fn main() {
    // problem_2(4000000);
    //problem_3(600851475143);
    //problem_4(1000);
    //let _ = problem_5(20);
    //problem_6(100);
    //println!("{}",problem_7(10001));
    problem_8(13);
}

fn problem_0(num_odds: u64) {
    let mut sum: u64 = 0;
    for n in 1..=num_odds {
        let num_square = n * n;
        if num_square % 2 == 1 {
            sum += num_square;
        }
    }

    println!("Sum of the first {} odd square numbers: {}", num_odds, sum);
}

fn problem_1(max_num: u64) {
    let mut sum: u64 = 0;
    for n in 1..max_num {
        if n % 3 == 0 || n % 5 == 0 {
            sum += n;
        }
    }

    println!("Sum of the numbers less than {} that are both multiples of 3 and 5: {}", max_num, sum);
}

fn problem_2(max_num: u64) {
    let mut sum: u64 = 2;
    let mut a: u64 = 1;
    let mut b: u64 = 2;

    while b < max_num {
        let next = a + b;
        if next >= max_num {
            break;
        }
        if next % 2 == 0 {
            sum += next;
        }
        a = b;
        b = next;
    }

    println!("Sum of even fibonacci numbers less than {}: {}", max_num, sum);
}

fn problem_3(num2check: u64) {
    let mut lastprime: u64 = 0;
    let mut leftovernum: u64 = num2check;
    let mut curriter: u64 = 2;

    while leftovernum > 1 {
        let mut is_prime = true;
        for i in 2..curriter {
            if curriter % i == 0 {
                is_prime = false;
                break;
            }
        }

        if is_prime {
            while leftovernum % curriter == 0 {
                leftovernum /= curriter;
                lastprime = curriter;
            }
        }

        curriter += 1;
    }

    println!("Largest prime factor of {}: {}", num2check, lastprime);
}

fn problem_4(maxnum: u64) {
    let mut maxpalindrome = String::new();
    let mut priorpalindrome = 0;

    for i in 1..maxnum {
        for j in 1..maxnum {
            let currproduct = i * j;
            if currproduct > priorpalindrome {
                let strproduct: String = format!("{:?}", currproduct);
                let reversed = strproduct.chars().rev().collect::<String>();
                if strproduct == reversed {
                    priorpalindrome = currproduct;
                    maxpalindrome = strproduct;
                }
            }
        }
    }
    println!("Largest palindrome that is a square of numbers less than {} is {}", maxnum, maxpalindrome);
}

fn problem_5(max_num: u64) -> Option<u64> {
    let mut currnum = max_num;
    if max_num < 2 {
        eprintln!("Error: max_num must be at least 2");
        return None;
    }
    let mut stopcondition = false;
    while !stopcondition {
        currnum = currnum + 1;
        stopcondition = check(currnum,max_num);

    }
    fn check(currnum: u64, max_num: u64) -> bool {
        let mut passedlogic = true;
        let mut curriter = 0;
        while passedlogic & (curriter <= max_num) {
                curriter = curriter + 1;
                passedlogic = passedlogic & (currnum % curriter == 0);
        }
        passedlogic
    }

    println!("Smallest positive number divisible by all numbers from 1 to {}: {}", max_num, currnum);
    Some(currnum)
}

fn problem_6(n: u64) -> u64 {
    let mut sum_of_squares: u64 = 0;
    let mut sum: u64 = 0;

    for i in 1..=n {
        sum_of_squares += i * i;
        sum += i;
    }

    let square_of_sum: u64 = sum * sum;
    let difference: u64 = square_of_sum - sum_of_squares;

    println!("Sum of squares of 1 to {}: {}", n, sum_of_squares);
    println!("Square of sum of 1 to {}: {}", n, square_of_sum);
    println!("Difference: {}", difference);

    difference
}

fn is_prime(num: u64) -> bool {
    if num < 2 {
        return false;
    }
    if num == 2 {
        return true;
    }
    if num % 2 == 0 {
        return false;
    }
    let sqrt = (num as f64).sqrt() as u64;
    for i in (3..=sqrt).step_by(2) {
        if num % i == 0 {
            return false;
        }
    }
    true
}

fn problem_7(n: u64) -> u64 {
    let mut count = 0;
    let mut candidate = 2;

    while count <= n {
        if is_prime(candidate) {
            count += 1;
            if count == n {
                return candidate;
            }
        }
        candidate += 1;
    }

    candidate
}

fn problem_8(m: usize) -> u64 {
    assert!(n >= 1);
    let digits = parse_number_block(include_str!("../data/p8_load.txt"));
    assert!(digits.len() >= n);

    if m > digits.len() {
        return 0;
    }

    let mut max_product = 0;

    for i in 0..=(digits.len() - m) {
        let product: u64 = digits[i..i + m].iter().product();
        if product > max_product {
            max_product = product;
        }
    }

    max_product
}

fn parse_number_block(s: &str) -> Vec<u32> {
    s.chars().filter_map(|c| c.to_digit(10)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem_5_with_invalid_input() {
        let result = problem_5(1);
        assert!(result.is_none(), "problem_5(1) should return None (error)");
    }

    #[test]
    fn test_problem_5_with_20() {
        let result = problem_5(20);
        assert_eq!(result, Some(232792560), "problem_5(20) should equal 232792560");
    }

    #[test]
    fn test_problem_6_with_100() {
        let result = problem_6(100);
        assert_eq!(result, 25164150, "problem_6(100) should equal 25164150");
    }

    #[test]
    fn test_problem_7_first_prime() {
        let result = problem_7(1);
        assert_eq!(result, 2, "The 1st prime should be 2");
    }

    #[test]
    fn test_problem_7_second_prime() {
        let result = problem_7(2);
        assert_eq!(result, 3, "The 2nd prime should be 3");
    }

    #[test]
    fn test_is_prime_7() {
        assert!(is_prime(7), "7 should be prime");
    }

    #[test]
    fn test_is_prime_8() {
        assert!(!is_prime(8), "8 should not be prime");
    }
}
