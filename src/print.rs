fn main()
{
	println!("{} days", 31); // 31 is default i32, can use 32i32 to be explicit
	
	println!("{0}, this is {1}. {1}, this is {0}.","Alice","Bob");
	
	println!("{subject} {verb} {object}",
			 object = "the lazy dog",
			 subject = "the quick brown fox",
			 verb = "jumps over");
			 
	println!("{} of {:b} people know binary, the other half doesn't", 1, 255);
	
	
}