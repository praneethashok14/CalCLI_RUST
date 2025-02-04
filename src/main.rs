use std::io::stdin;
fn main() {
    let mut meth=String::new();
    let mut num=String::new();
    let mut num2=String::new();
    let mut res=0.0;
    println!("Enter number 1: ");
    stdin().read_line(&mut num).unwrap();
    let num: f64 = num.trim().parse().unwrap();
    println!("Enter number 2: ");
    stdin().read_line(&mut num2).unwrap();
    let num2: f64 = num2.trim().parse().unwrap();
    println!("Enter method (1: Add, 2: Subtract, 3: Multiply, 4: Divide).): ");
    stdin().read_line(&mut meth).unwrap();
    let meth: i32 = meth.trim().parse().unwrap();
    if meth==1{
        res=num+num2;
    }
    if meth==2{
        res=num-num2;
    }
    if meth==3{
        res=num*num2;
    }
    if meth==4{
        res=num/num2;
    }
    println!("The result is: {res}");
}