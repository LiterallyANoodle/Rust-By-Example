use std::fmt;


fn main() {
	
	// Let's list the primitives of the Scalar type: 
	// Signed integers: 
	//	- i8
	// 	- i16
	// 	- i32
	// 	- i64
	// 	- i128
	// 	- isize (pointer size)
	// 	- Defaults to size i32
	// Unsigned integers: 
	//	- u8
	// 	- u16
	// 	- u32
	// 	- u64
	// 	- u128
	// 	- usize (pointer size)
	// Floating Point:
	// 	- f32
	// 	- f64
	// 	- Defaults to size f64
	// char 
	// 	- Any valid unicode character.
	// bool
	// 	- Either true or false
	// Unit
	// 	- ()
	// 	- The unit has no other value than the empty tuple. 

	// Compound types:
	// Array []
	// 	- The array is an ordered sequence of instances of the same type. 
	// Tuples ()
	// 	- The tuple is an ordered sequence of instances of any type. 

	// Using a colon allows you to type annotate your variables. 
	// Much like ML, however, Rust is able to infer data types of parameters based on how it's used. 

	// Integer addition and subtraction: 
	println!("1 + 2 = {}", 1u32 + 2u32 );
	println!("1 - 2 = {}", 1i32 - 2i32 );
	// The types must be exactly the same for these to work. 

	// Short-circuiting boolean logic
	println!("true AND false is {}", true && false);
	println!("true OR false is {}", true || false);
	println!("NOT true is {}", !true);

	// Bitwise operations
	println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
	println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
	println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
	println!("1 << 5 is {}", 1u32 << 5);
	println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);



	/*
		Tuples 
	*/

	// Tuples can be used as inputs and outputs 
	fn reverse(pair: (i32, bool)) -> (bool, i32) {
		// 'let' can be used to bind the parts of a tuple to specific names
		let (int_param, bool_param) = pair;

		return (bool_param, int_param);
	}

	#[derive(Debug)]
	struct Matrix(f32, f32, f32, f32);

	// Tuples can be accessed with indexes.
	// Tuples can be inside tuples. 
	// Tuples are printable up to 12 elements. 

	let pair = (73i32, false);
	println!("Pair in order: {:?}", pair);
	println!("Pair in reverse order: {:?}", reverse(pair));

	// Lets make a Display for the Matrix
	impl fmt::Display for Matrix {
		fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
			// use self to refer to each positional argument:
			write!(f, "[ {}, {} ]\n[ {}, {} ]", self.0, self.1, self.2, self.3)
		}
	}

	let m = Matrix(1f32, 2f32, 3f32, 4f32);

	println!("{}", m);

	fn transpose(matrix: Matrix) -> Matrix {
		
		let Matrix(i1, i2, i3, i4) = matrix;

		return Matrix(i1, i3, i2, i4)
	}

	println!("Transposed Matrix: \n{}", transpose(m));



	/*
		Arrays and Slices
	*/

	// An array is a contiguous space in memory of fixed length which contains objects of the same type 
	// An array's length is known at compile time. 
	// The type signature of an array is [T; length] where T is the type of stored object. 

	// A slice is similar but the length is not known at compile time. 
	// A slice is a two word object. The first word is the pointer to its start in memory.
	// The second word is the length of the slice. 
	// Slices can be used to borrow a section of data from an existing array. 
	// The type signature of a slice is &[T]

	let array: [i32; 5] = [1, 2, 3, 4, 5]; // array signature declaration is superfluous in this example. 

	// print the whole array 
	println!("The whole array: {:?}", array);

	// make a slice of the array and print that
	println!("A slice: {:?}", &array[2..5]);

	// Arrays can be safely accessed with .get() which returns an option. 
	// Arrays can be safely accessed using `.get`, which returns an
    // `Option`. This can be matched as shown below, or used with
    // `.expect()` if you would like the program to exit with a nice
    // message instead of happily continue.
    for i in 0..array.len() + 1 { // Oops, one element too far!
        match array.get(i) {
            Some(val) => println!("{}: {}", i, val),
            None => println!("Slow down! {} is too far!", i),
        }
    }

}