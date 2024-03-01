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

// floating point in action
let number:f64=10.9;
println!("number is {number}");

// calculations i action
let addition=30+20;
let difference=30/2;
let multiply=4*6;
let remainder=13%2;
println!("result of addition is :{addition},\n result of difference is :{difference},\nresult of multiplication is :{multiply},\nthe ramainder is :{remainder}");
}
