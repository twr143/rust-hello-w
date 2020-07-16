fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn higer_order_fn<F>(value: i32, step: F) -> i32
    where F: Fn(i32) -> i32 {
    step(value)
}

fn main() {
    let fn_variable = add;
    println!("calling using function variable {}", fn_variable(10, 20));
    fn add_one(x:i32)->i32 { x+1}
    let result = higer_order_fn(20, add_one);
    println!("result =  {}", result);
}
