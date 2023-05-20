
fn main() {
	
	/*
		Structs 
	*/

	// Structs are very easy to understand 
	// There are 3 types of structs: 
	// 		C-style structs 
	//		Tuple Structs 
	// 		Unit Structs 

	// A C-style struct is a type with named fields within it 

	// A tuple struct is a named tuple with positional fields within it

	// A unit struct is a type which contains no fields and is simply itself



	/*
		Enums 
	*/

	// An enum is declared with the keyword 'enum' and is a type which may be certain variants. 
	// These variants may have internal data that is not known at compile time except by type. 
	// Any valid struct is also a valid enum. But not all valid enums are valid structs. 
	// An enum can contain any number of structs within it of varying type. 
	// Enums are very similar to the custom data types of ML 

	// Enums can have aliases 
	// Most commonly, this is Self:: when used in an impl block 
	enum VeryVerboseEnumOfThingsToDoWithNumbers {
	    Add,
	    Subtract,
	}

	type Operations = VeryVerboseEnumOfThingsToDoWithNumbers; // This is an alias 

	impl Operations {
	    fn run(&self, x: i32, y: i32) -> i32 {
	        match self {
	            Self::Add => x + y,
	            Self::Subtract => x - y,
	        }
	    }
	}

    let vv = Operations::Add;
    let w = Operations::Subtract;
    
    println!("{:?}", vv.run(1, 2));
    println!("{:?}", w.run(1, 2));

    // enums can also use implicit discriminators or explicit discriminators 
    // explicit discriminators are much more like c-structs which have names for elements 
    // enum with implicit discriminator (starts at 0)
	enum Number {
	    Zero,
	    One,
	    Two,
	}

	// enum with explicit discriminator
	enum Color {
	    Red = 0xff0000,
	    Green = 0x00ff00,
	    Blue = 0x0000ff,
	}

    // `enums` can be cast as integers.
    println!("zero is {}", Number::Zero as i32);
    println!("one is {}", Number::One as i32);

    println!("roses are #{:06x}", Color::Red as i32);
    println!("violets are #{:06x}", Color::Blue as i32);

}