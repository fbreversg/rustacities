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

fn valid_copy() {
	let x = 5;
	let y = x;		//Valid copy since its fixed size and being on stack, not on heap.

	println!("x = {}, y = {}", x, y);
}

fn clone_copy() {
	let s1 = String::from("hello");
	let s2 = s1.clone(); 	//hard copy. Copies stack and heap!

	println!("s1 = {}, s2 = {}", s1, s2);

}

