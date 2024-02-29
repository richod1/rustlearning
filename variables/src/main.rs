fn main() {
let mut x=40;
println!("value of x is {x}");
    x=50;
println!("value of x :{x}");

// using cont
const THREE_HOURS:u32=60*60*3;
println!("THREE HOURS IS :{THREE_HOURS}");

// variable shadowing
let y=5;
let y=y+1;
{
    let y=y*2;
    println!("the value of y in the inner scope is :{y}");
}

println!("value of outer scope variable is {y}");


}
