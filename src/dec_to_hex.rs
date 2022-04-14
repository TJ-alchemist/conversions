
pub fn dec_to_hex(number: u32) -> String {
	// To store the result as String type.
	// The number should be positive or else it will
	// throw an exception
	let mut output: String = String::new();
	// For storing remainder & quotient values on 
	// every iteration till quotient doesn't reach zero
	let mut remainder: u32;
	let mut quotient: u32;
	// Storing a copy of the entered number.
	// Best to not manipulate the original number.
	let mut tmp: u32 = number;
	
	loop {
		// Empty character binding. 
		// The value will renew on each iteration.
		let mut letter: char = '\0';
		// Dividing both the remainder and the quotient
		// till the quotient reaches zero.
		quotient = tmp / 16;
		remainder = tmp % 16;
		// Matching the hexadecimal notations
		// from 10 to 15.
		match remainder {
			10 => { letter = 'A'; },
			11 => { letter = 'B'; },
			12 => { letter = 'C'; },
			13 => { letter = 'D'; },
			14 => { letter = 'E'; },
			15 => { letter = 'F'; },
			_ => ()
		};
		// Updating the output string
		if letter != '\0' {
			output.push(letter);
		} else {
			output.push_str(format!("{}", remainder).as_str());
		}
		// Break the loop when quotient becomes zero
		if quotient == 0 {
			break;
		}
		// If quotient is still not zero 
		// then continue with the computation
		tmp = quotient;
	}
	
	format!("0x{}", output.chars().rev().collect::<String>())
}