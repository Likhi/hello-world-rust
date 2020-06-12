fn main()
{
	println!("{} days", 31); // 31 is default i32, can use 32i32 to be explicit

	println!("{0}, this is {1}. {1}, this is {0}.","Alice","Bob");

	println!("{subject} {verb} {object}",
			 object = "the lazy dog",
			 subject = "the quick brown fox",
			 verb = "jumps over");

	println!("{} of {:b} people know binary, the other half doesn't", 1, 255);

	println!("{number:>0width$}",number=1,width=5);

	println!("My name is {0}, {1} {0}", "Bond", "James");
	// FIXME ^ Add the missing argument: "James"

	// Create a structure named `Structure` which contains an `i32`.
	#[allow(dead_code)]
	struct Structure(i32);

	// println!("This struct `{}` won't print...", Structure(3));
	// FIXME ^ Comment out this line.

	let pi = 3.141592;
	println!("Pi is roughly {:.3}", pi);

}
