fn is_even(n: i32) -> bool {
    if n % 2 == 0 {
        return true;
    } else {
        return false;
    }
}

fn main() {
    let mut l = 9;
    let mut total = 0;
    let numbers: [i32; 10] = [9, 12, 14, 15, 20, 21, 27, 31, 33, 45];
    for n in numbers.iter() {
        if is_even(*n) == true {
            println!("Number {} is even!", n);
        } else {
            println!("Number {} is odd!", n);
        }
        if n % 3 == 0 && n % 5 == 0 {
            println!("FizzBuzz");
        } else if n % 3 == 0 {
            println!("Fizz");
        } else if n % 5 == 0 {
            println!("Buzz");
        }
    }
    while (l >= 0) {
        total = total + numbers[l];
        if l == 0 {
            break;
        }
        l = l - 1;
    }
    println!("Sum: {}", total);
    let mut largest_num = numbers[0];
    let mut i = 0;
    loop {
        i += 1;
        if i == numbers.len() {
            break;
        }
        if numbers[i] > largest_num {
            largest_num = numbers[i];
        }
    }
    println!("Largest number:{}", largest_num);
}
