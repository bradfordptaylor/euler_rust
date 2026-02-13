fn main() {
    // problem_2(4000000);
    //problem_3(600851475143);
    problem_4(100)
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

    for i in (1..maxnum).rev() {
        let currproduct = i * i;
        let strproduct: String = format!("{:?}", currproduct);
        let reversed = strproduct.chars().rev().collect::<String>();

        if strproduct == reversed {
            maxpalindrome = strproduct;
            break;
        }
    }
    println!("Largest palindrome that is a square of numbers less than {} is {}", maxnum, maxpalindrome);
}