use std::io;

// simple struct to make storing and printing results easier
struct Person {
    name: String,
    total_owed: f32,
}

impl Person {
    fn print(&self) {
        // print with rounding!
        println!("{}'s split is {:.2}", self.name, self.total_owed);
    }
}

// quick way to get float values from user
fn get_float(prompt: &str) -> f32 {
    let result = loop {
        println!("\n{}", prompt);

        let mut input = String::new(); // this will be the input from user
        if io::stdin().read_line(&mut input).is_ok() {
            // only returns input if there is no error
            if let Ok(result) = input.trim().parse::<f32>() {
                break result;
            } else {
                println!("That didn't work, try again")
            }
        } else {
            println!("That didn't work, try again")
        }
    };
    result
}

// collect list of names in form "Name 1, Name2, name3,name4"
fn get_names(prompt: &str) -> Vec<String> {
    let names: Vec<String> = loop {
        println!("{}", prompt); // promt user for input

        let mut input = String::new(); // this will be the input from user
        if io::stdin().read_line(&mut input).is_ok() {
            // only returns input if there is no error
            let names: Vec<&str> = input.trim().split(',').collect();
            break names.iter().map(|s| s.trim().to_string()).collect();
        } else {
            println!("That didn't work, try again")
        }
    };
    names
}

// Creates a vector full of everyone
fn get_everyone(
    names: Vec<String>,
    subtotal: f32,
    total: f32,
    mut everyone: Vec<Person>,
    mut subtotal_counter: f32,
) -> Vec<Person> {
    loop {
        for name in &names {
            let sub_owed = loop {
                println!("\nHow much did {} pay pre tax/tip?", &name);

                let mut input = String::new(); // this will be the input from user
                if io::stdin().read_line(&mut input).is_ok() {
                    // only returns input if there is no error
                    if let Ok(sub_owed) = input.trim().parse::<f32>() {
                        break sub_owed;
                    } else {
                        println!("That didn't work, try again")
                    }
                } else {
                    println!("That didn't work, try again")
                }
            };

            subtotal_counter -= sub_owed; // counter for later

            // creates a new person and adds to vector of all people
            let new_person = Person {
                name: String::from(name),
                total_owed: (sub_owed / subtotal) * total,
            };
            everyone.push(new_person);
        }
        if subtotal_counter >= 0.0 {
            break;
        } else {
            println!("\nThe cost has exceeded the subtotal! Lets try again.")
        } // quick error check to see if the total is too large
    }

    everyone
}

fn main() {
    let names = get_names("Please list the names of everyone to split with seperated by commas.");
    let subtotal = get_float("What was the subtotal (pre tax/tip)?");
    let total = get_float("What was the total (after tax/tip)?");

    let everyone = get_everyone(names, subtotal, total, Vec::new(), subtotal.clone());

    // print all the results
    println!("");
    for individual in everyone {
        individual.print();
    }
    println!("");
}
