// Silence some warnings so they don't distract from the exercise.
#![allow(unused_mut, unused_variables)]

fn main() {
    // This fancy stuff either gets the first argument as a String, or prints
    // usage and exits if an argument was not supplied to the program.
    let mut arg: String = std::env::args().nth(1).unwrap_or_else(|| {
        println!("Please supply an argument to this program.");
        std::process::exit(-1);
    });
    //inspect(&arg);

    // 1. Write a function `inspect` that takes a reference to a String, returns nothing, but
    // prints whether the contents of the String is plural or singular. Then uncomment and run this
    // code with `cargo run apple` and `cargo run apples'.  Hint: use `.ends_with("s")` on the
    // String reference
    //
    fn inspect(param : &String){
        if param.ends_with("s") {
           println!("The word {param} is plural"); 
        }
        else {
            println!("The word {param} is singular");
        }
    }

    // 2. Write a function `change` that takes a *mutable* reference to a String and adds an "s" to
    // the String if it doesn't already end with "s". Then uncomment and run the code below with
    // `cargo run apple`.  Hint: use `.push_str("s")` on the mutable String reference to add an "s".
    //
    
    fn change(param: &mut String){
        if !param.ends_with("s") {
            param.push_str("s");
        }
    }
    // change(&mut arg);
    // println!("I have many {}", arg);

    // 3. Write a function `eat` that accepts ownership of (consumes) a String and returns a bool
    // indicating whether or not the String both starts with a "b" AND contains an "a".
    // Hint 1: use `.starts_with("b")` and `.contains("a")`
    // Hint 2: `&&` is the boolean "AND" 
    fn eat(param: String)-> bool{
        if param.starts_with("b") && param.contains("a") {
            return true;
        }
        return false;
    }
    //
    if eat(arg) {
       println!("Might be bananas");
    } else {
       println!("Not bananas");
    }

    // Try running this program with "boat", "banana", and "grapes" as the arguments :-)

    // Challenge: Write a function "bedazzle" that takes a mutable reference to a String and
    // ignores what is in the string and replaces the contents of the string with the String
    // "sparkly". Then uncomment the code below.
    fn bedazzle(param : &mut String) {
        *param = String::from("sparkly");
    }
    // Hint: You will need to dereference the mutable reference in order to assign it a
    // new value.
    //
    let mut material = "mud".to_string();
    println!("This material is just `{}`.", material);
    bedazzle(&mut material);
    println!("Wow! Now the material is `{}`!", material);

    //STRUCT AND CONTROL FLOW
    let args: Vec<String> = std::env::args().skip(1).collect();
 

    // This consumes the `args` vector to iterate through each String
    for arg in args {
        // 1a. Your task: handle the command-line arguments!
        
        // - If arg is "sum", then call the sum() function
        if arg== "sum"{
            sum();
        }
        // - If arg is "double", then call the double() function
        else if arg== "double"{
            double();
        }
        else {
            count(arg);
        }
        // - If arg is anything else, then call the count() function, passing "arg" to it.


        // 1b. Now try passing "sum", "double" and "bananas" to the program by adding your argument
        // after "cargo run".  For example "cargo run sum"
    

    //Struct challenge
    let mut carrot = Carrot { percent_left: 100.0 };
    carrot.bite();
    println!("I take a bite: {:?}", carrot);

    let mut grapes = Grapes { amount_left: 100 };
    grapes.bite();
    println!("Eat a grape: {:?}", grapes);
}



//CONTROL FLOW CHALLENGE AND STRUCT CHALLENGE
 // This collects any command-line arguments into a vector of Strings.
    // For example:
    //
    //     cargo run apple banana
    //
    // ...produces the equivalent of
    //
    //     vec!["apple".to_string(), "banana".to_string()]
    

    // Create a generic `bunny_nibbles`
    // function that:
    // - takes a mutable reference to any type that implements Bite
    // - calls `.bite()` several times
    // Hint: Define the generic type between the function name and open paren:
    //       fn function_name<T: Bite>(...)
    fn bunny_nibbles<T: Bite> (item: &mut T){
        item.bite();
        item.bite();
        item.bite();
    }
    //
    bunny_nibbles(&mut carrot);
    println!("Bunny nibbles for awhile: {:?}", carrot);
}


fn sum() {
    let mut sum = 0;
    // 2. Use a "for loop" to iterate through integers from 7 to 23 *inclusive* using a range
    for mut number in 7..=23 {
        sum += number;
    }
    // and add them all together (increment the `sum` variable).  Hint: You should get 255
    // Run it with `cargo run sum`


    println!("The sum is {}", sum); 
}

fn double() {
    let mut count = 0;
    let mut x = 1;
    // 3. Use a "while loop" to count how many times you can double the value of `x` (multiply `x`
    // by 2) until `x` is larger than 500.  Increment `count` each time through the loop. Run it
    // with `cargo run double`  Hint: The answer is 9 times.
    while x <= 500  {
        x *=2;
        count +=1;
    }


    println!("You can double x {} times until x is larger than 500", count);
}

fn count(arg: String) {
    // Challenge: Use an unconditional loop (`loop`) to print `arg` 8 times, and then break.

    // You will need to count your loops, somehow.  Run it with `cargo run bananas
    let mut count = 1;

    loop{
        if count>=9{
            break;
        }
        print!("{} ",arg);
        count +=1;
    }
    // print!("{} ", arg); // Execute this line 8 times, and then break. `print!` doesn't add a newline.


    println!(); // This will output just a newline at the end for cleanliness.
}

//STRUCT CHALLENGE FNs
// 1. Define a trait named `Bite`
//
trait Bite{
    fn bite(self : &mut Self);
}
// Define a single required method, `fn bite(self: &mut Self)`.  We will call this method when we
// want to bite something.  Once this trait is defined, you should be able to run the program with
// `cargo run` without any errors.
//
//  trait Bite...


// 2. Now create a struct named Grapes with a field that tracks how many grapes are left.  If you
// need a hint, look at how it was done for Carrot at the bottom of this file (you should probably
// use a different field, though).

#[derive(Debug)]
struct Grapes{
   amount_left : i32,
}


// 3. Implement Bite for Grapes.  When you bite a Grapes, subtract 1 from how many grapes are left.
// If you need a hint, look at how it was done for Carrot at the bottom of this file.
//
// impl Bite for...
impl Bite for Grapes{
    fn bite(self: &mut Self){
        self.amount_left -= 1;
    }
}

#[derive(Debug)] // This enables using the debugging format string "{:?}"
struct Carrot {
    percent_left: f32,
}

impl Bite for Carrot {
    fn bite(self: &mut Self) {
        // Eat 20% of the remaining carrot. It may take awhile to eat it all...
        self.percent_left *= 0.8;
    }
}

