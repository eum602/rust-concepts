// Yes, yes, we know. It's an exercise, compiler, we want it that way!
#[allow(unused_mut)]

fn main() {
    // 1. Uncomment the code below. Create a closure that returns the square of an integer (the
    // number multiplied by itself), and assign the closure to the "square" variable. Then run the
    // code and make sure it works.

    let a = 5;
    let square = |x: i32| x * x;
    println!("5 squared is {}", square(a));
    //println!("{}",a);

    // 2. Uncomment the code below.  Finish the .map() iterator adaptor call by passing it a closure
    // which takes a tuple of two integers as a parameter, and returns a tuple with the first
    // integer incremented by 1, and the second integer left alone.  For example, if given the input
    // (0, 1), it should return (1, 1). Run the code and make sure it works.

    let pairs = vec![(0, 1), (2, 3), (4, 5)];
    pairs
        .into_iter()
        //.map( |t| (t.0 +1 ,t.1) )
        .map( |(x,y)| (x +1 ,y) )
        .for_each(|t| println!("{:?}", t));

    // 3. Uncomment the code below. There is a mutable vector named `numbers`. Use an iterator over
    // mutable references to multiply each of the values in `numbers` by 3.
    // Hint 1: You'll need .iter_mut() -- bonus points if you use the shorter, syntactic sugar form!
    // Hint 2: `x` will be a mutable reference, so remember to dereference it to use it

    let mut numbers = vec![1, 2, 3, 4];
    //for x in numbers.iter_mut() {
    for x in &mut numbers {
        *x *= 3; // multiply the value by 3 via the mutable reference x
    }
    println!("{:?}", numbers); // should print [3, 6, 9, 12]

    // 4. Uncomment the code below.  Take the vector of words and
    // - Convert the vector into an iterator with .into_iter()
    // - Use .filter() to remove any word that contains the letter "h" -- use .contains()
    // - Use .map() to convert all the words to uppercase -- use .to_uppercase()
    // - Use .collect() to put the transformed words back into a vector
    //
    // Hint: .to_uppercase() is a method on `str` which returns a String

    let words = vec!["autobot", "beach", "car", "decepticon", "energon", "frothy"];
    let transformed = words
    .into_iter()
    .filter(|w| !w.contains('h'))
    .map(|w| w.to_uppercase())
    .collect::<Vec<_>>() //Telling the values are gonna being collected into a Vect ... <_> means the value is already kown by the map 'cause we are dealing with strings
    ;
    println!("Transformed: {:?}", transformed);
    //println!("{:?}", words) //wont work since word was borrowed and thus is no longet available

    // Challenge:
    //
    // - Rewrite the code in #2 as a for loop
    let pairs = vec![(0, 1), (2, 3), (4, 5)];
    for (x,y) in pairs {
        println!("{:?}", (x+1,y))
    }

    // - Rewrite the code in #3 in functional style (without a for loop).  Hint: There are multiple
    // ways to accomplish this, but they all end with an iterator consumer.
    let mut numbers = vec![1, 2, 3, 4];
    let t: Vec<_> = numbers.into_iter().map(|x| x * 3 ).collect();
    println!("{:?}", t); // should print [3, 6, 9, 12]

    // ***************** iterators and ownership *******************
    let x = 3;
    //x += 2; //cannot assign twice an immutable variable

    let equal_to_x = move |z:i32| z == x;

    println!("I can use x here: {:?}", x);

    let y = 4;
    equal_to_x(y);

    // 
    let x: Vec<i32> = vec![1,2,3];

    let equal_to_x = move |z:Vec<i32>| z == x;

    // println!("I canNOT use x here: {:?}", x); //variable moved due to use in closure

    let y = vec![1,2,3];
    assert!(equal_to_x(y));

    // 
    let x: Vec<i32> = vec![1,2,3];

    let equal_to_x = |a:&Vec<i32>, b: Vec<i32>| *a == b;

    let y = vec![1,2,3];
    assert!(equal_to_x(&x,y));
    println!("I CAN use x here: {:?}", x); //variable passed as reference
}

