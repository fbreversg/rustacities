fn main() {
    let mut s = String::from("hello");

	s.push_str(", world!"); // push_str() appends a literal to a String

	println!("{}", s); // This will print `hello, world!`
}

fn invalid_copy() {
	let s1 = String::from("hello");
	let s2 = s1;	//From now s1 is invalidated and only s2 has the data

	println!("{}, world!", s1);

}