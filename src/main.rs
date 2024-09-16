use std::io;

// simple struct to make storing and printing results easier
struct Person {
	name: String,
	total_owed: f32,
}

impl Person {
    fn print(&self) {	// print with rounding!
        println!("{}'s split is {:.2}", self.name, self.total_owed);
    }
}

fn main() {

	// collect list of names in form "Name 1, Name2, name3,name4"
	let names: Vec<String> = loop {
		println!("Please list the names of everyone to split with seperated by commas.");	// promt user for input

		let mut input = String::new();	// this will be the input from user
		if io::stdin().read_line(&mut input).is_ok() {	// only returns input if there is no error
        	let names: Vec<&str> = input.trim().split(',').collect();
        	break names.iter().map(|s| s.trim().to_string()).collect();
        } else {println!("That didn't work, try again")}
		
    };
    
    let subtotal = loop {
    	println!("\nWhat was the subtotal (pre tax/tip)?");
    	
    	let mut input = String::new();	// this will be the input from user
    	if io::stdin().read_line(&mut input).is_ok() {	// only returns input if there is no error
    		if let Ok(subtotal) = input.trim().parse::<f32>() {
    			break subtotal;
    		} else {println!("That didn't work, try again")}
    	} else {println!("That didn't work, try again")}
    };
    
    let total = loop {
    	println!("\nWhat was the total (after tax/tip)?");
    	
    	let mut input = String::new();	// this will be the input from user
    	if io::stdin().read_line(&mut input).is_ok() {	// only returns input if there is no error
    		if let Ok(total) = input.trim().parse::<f32>() {
    			break total;
    		} else {println!("That didn't work, try again")}
    	} else {println!("That didn't work, try again")}
    };
    
    let mut everyone: Vec<Person> = Vec::new();	// placeholder for later
   	let mut subtotal_counter = subtotal.clone();	// counter for error check later
   	
   	loop {
		for name in &names{
			
			let sub_owed = loop {
				println!("\nHow much did {} pay pre tax/tip?", &name);
				
				let mut input = String::new();	// this will be the input from user
				if io::stdin().read_line(&mut input).is_ok() {	// only returns input if there is no error
					if let Ok(sub_owed) = input.trim().parse::<f32>() {
						break sub_owed;
					} else {println!("That didn't work, try again")}
				} else {println!("That didn't work, try again")}
			};
			
			subtotal_counter -= sub_owed;	// counter for later
			
			// creates a new person and adds to vector of all people
			let new_person = Person {
			    name: String::from(name),
			    total_owed: (sub_owed / subtotal) * total,
			};
			everyone.push(new_person);
		}
		if subtotal_counter >= 0.0 {break;} else {println!("\nThe cost has exceeded the subtotal! Lets try again.")}	// quick error check to see if the total is too large
    }
    
    // print all the results
    for individual in everyone {
    	individual.print();
    }
    
}
