fn main() {
    println!("Hello, world!");

    let x: i32 = 7;

    if x < 5{
        println!("value less than 5");
    }
    else {
        println!("value greater than or equal to 5");
    }

    if x % 2 == 0 { //needs to eval to a bool will not use truthy/falsey vals
        println!("x is even");
    } else if true { //allows else ifs
        println!("x is odd");
    } else {
        println!("shouldn't get here");
    }


    // if else can be used for assigning value
    let number: i32 = if x%2==0 {10} else {9};// {} are required for results

    println!("the value of number is {number}");


    //3 types of loops
    // loop <- forever/until we do a break statement think while(true)
    // while <- conditial statement
    // for <- for loop and execute some code for each item in a collection

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; // you can return values from a loop
        }
    };

    println!("The result is {result}");

    //You can optionally specify a loop label on a loop that you can then use with break or continue to specify that those keywords apply to the labeled loop instead of the innermost loop
    let mut count = 0;
    let other_result = 'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up 3;
            }
            remaining -= 1;
        }

        count += 1;
    };

    println!("End count = {count}");

    println!("other_result is  = {other_result}");

    let mut number = 3;

    // while loops basically work how you expect based on info about if statements and loop loops
    while number != 0 {
        println!("{number}!");

        number -= 1;
    }
    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    for element in a{
        println!("the value is: {element}");
    }

    for number in (1..4).rev() { // Range is useful 1..4 is basically ints from [1,4) so [1, 2, 3]
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
