// This is a comment 
/*
	This is also a comment 
*/

// This is the main function which is the automatic starting point when a compiled program is run
fn main() {
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

}