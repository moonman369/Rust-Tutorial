fn main() {
    println!("I like rusty spoons!");

    //Data-types
    let mut a: i32 = 10;
    let b: i32 = 20;
    let c: f32 = 173.1313;
    let d: u32 = 15;
    let e: char = 'a';
    let f: bool = false;

    a = 23;

    println!("a = {}; b = {}; c = {}; d = {}", a, b, c, d);
    println!("char e = {}", e);
    println!("bool f = {}", f);

    // Tuples: Predefined collection of heterogenous typed data
    let g: (i32, f32, u32, char, bool, &str) = (-34, 2983.24, 86, 's', true, "Ruzzt"); 
    println!("tuple g = {:?}", g);

    // Arrays (Static): Static array, Homogenous datatype
    let arr: [i32; 6] = [2,4,1,5,8,-3];
    println!("array arr = {:?}", arr);

    // Vector (Dynamic arrays)
    let mut vec: Vec<&str> = vec!["string1"];
    vec.push("Hello");
    vec.push("Hello123");
    vec.push("Hello3321");
    vec.push("popthis");
    vec.push("pop");
    vec.pop();
    println!("vector vec = {:?}", vec);

}
