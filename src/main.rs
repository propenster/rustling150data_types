fn main() {
    //INTEGERS
    let mut a:i64 = 2147483647;
    let mut b:i32 = 12121112;
    a = a + 1;
    print!("The value of a is {}", a);

    //FLOATS
    let x = 1.0; //f64
    let y:f32 = 2.0; //f32

    //BOOLEAN
    let t = true;
    let f:bool = false; //explicit type annotation...

    //CHARACTER value in single quotes
    let a = 'a';
    let b = 'A';



    //COMPUND TYPES 
    //Tuples //Arrays


    //TUPLES...
    let tup:(i32, i32, i32) = (1,2,3);
    let (x,y, z) = tup;


    let h = tup.0;
    println!("H = {}", h);

    println!("x = {} , Y = {} and Z = {}", x,y,z);


    //ARRAY...
    //ARrays in rust are allocated on stack rather than heap
    let numbers = [1,2,3,4,5,6,7,8,9,0];
    println!("First value = {} ", numbers[0]);

    println!("12th element = {}", numbers[12]);






}
