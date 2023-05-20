// This is a comment 
/*
	This is also a comment 
*/

// This is the main function which is the automatic starting point when a compiled program is run
fn main() {

	/*
		Formatted Print
	*/

	println!("Hello, world!"); // This is a macro which prints to the console (stdout)
	// You can tell it's a macro because of the bang (!) at the end of the name

	// to compile, we do `rustc <filename>` in the console to produce an executable binary like C

	// to print with formatting (much like C's printf), use the same println! macro with {} around your variables
	// The empty {} will use the next argument by default, but you can specify the index like {index}
	let x = "Rustacean!";
	println!("I'm a {0}", x);

	// You can also use named arguments such as this 
	println!("Who is {name}?", name="Alice");

	// numbers can be formatted with different forms according to the format character
	// If a particular index or name is required, that can still be specified before the colon
	println!("My number is {:b}. My number is also {0:x}.", 110);

	// You can also right-justify or left-justify text with a > or < sign. 
	// In both cases, you must specify the number of columns after the sign. 
	// These signs also allow you to add any padding you like. By default, the padding is a space. 
	// This is also a good moment to demonstrate formatting arguments!
	// A format argument acts just like any other named argument but with a $ after the name.
	println!("Sliiiide to the {l:-<10}", l="left!"); // left-justify with - as padding
	println!("Sliiiide to the {r:>10}", r="right!"); // right-justify with default space padding
	println!("{criss:-<width$}> <{cross:->width$}!", cross="cross", criss="criss", width=10);

	// Only types which implement std::fmt::Display are able to be used with formatting. 
	// User-defined types do not have a format by default. This is similar to Java classes needing a toString().

	// For Rust 1.58 and above, you can directly capture the argument from a
	// surrounding variable. Just like the above, this will output
	// "    1", 4 white spaces and a "1".
	let number: f64 = 1.0;
	let width: usize = 5;
	println!("{number:>width$}");

	// std::fmt has many traits, but the most common are fmt::Debug (marked by {:?}) and fmt::Display (marked by {})
	// The following print uses the Debug trait:
	println!("What is this {value:?}", value="variable?");

	// Finally, most special escape characters behave the way you would expect except for '{' and '}'
	// These characters are escaped by themselves as seen below:
	println!("Gimme a {{! \nGimme a }}! \nWhat's that spell? {{}}!");


	/*
		Debug
	*/


	// When making user-defined types, they are not printable by default 
	// giving a type the std::fmt::Debug trait makes this very straighforward as a format is automatically derived:
	// This works for any depth of user-made type 
	#[derive(Debug)]
	struct DebugPrintable(i32);
	// This struct is still not printable with Display, only Debug 
	println!("I can print this guy like this: {:?}", DebugPrintable(100));
	// This will fail!
	// println!("But not like this: {}", DebugPrintable(100));

	// std::fmt::Display must be manually implemented for every user-made type 

	// The automatic Debug print can be made a little prettier with the {:#?} syntax
	println!("Or I can print this guy like this: {:#?}", DebugPrintable(100));

	// The tradeoff of Debug is that your print will either be compact or pretty, but usually not both. 



	/*
		Display 
	*/

	// In order to make a custom display for a type, we have to do a few things: 
	// 1. import the std::fmt module to make it available 
	// Normally this would be imported at the top of the file. 
	use std::fmt;
	// 2. Define the structure for which you want to define the Display
	struct Structure1(i32);
	// 3. Implement the directions for printing 
	impl fmt::Display for Structure1 {
		// This trait requires 'fmt' with this exact signature: 
		fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
			// Write strictly the first element into the supplied output stram 'f'.
			// Returns a value of type 'fmt::Result' which indicates whether the operation succeeded or failed.
			// Note that 'write!' uses syntax very similar to 'println!'.
			write!(f, "{}", self.0)
		}

	}

	// In the case of containers with generic types, std::fmt::Display cannot be used since
	// the method of printing the generic type cannot be defined at compile time. 
	// For generic containers, fmt::Debug is required for printing. 

	// However, most containers are not generic and can be made to print how the programmer deems fit.
	// Here's a container with multiple internal values: 
	#[derive(Debug)]
	struct MinMax(i64, i64);

	// implement Display for MinMax:
	impl fmt::Display for MinMax {
		fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
			// use self to refer to each positional argument:
			write!(f, "({} {})", self.0, self.1)
		}
	}

	// Let's see a structure with named fields for comparison: 
	#[derive(Debug)]
	struct Point2D {
		x: f64,
		y: f64,
	}

	// Also implement Display for Point2D 
	impl fmt::Display for Point2D {
		fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
			// use self to refer to each named argument:
			write!(f, "({} {})", self.x, self.y)
		}
	}

	// Comparison time:
	let minmax = MinMax(0, 14);

	println!("Compare structures!");
	println!("Display: {}", minmax);
	println!("Debug: {:?}", minmax);

	let big_range = MinMax(-100, 100);
	let small_range = MinMax(-1, 1);

	println!("The big range is {big} and the small range is {small}", big=big_range, small=small_range);

	let point = Point2D { x: 1.1, y: 2.2 };

	println!("Compare points: ");
	println!("Display: {}", point);
	println!("Debug: {:?}", point);

}