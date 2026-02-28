fn main() {
    println!("Hello, world!");
    function_three();
    
    fn function_three(){
        println!("function_three");
    }

    { //look it works in an inner scope
        function_three();
        function_two();
    }
    function_two();

    let my_string: String = "Meow".to_string();
    function_four(32, my_string);

    /*
    Statements are instructions that perform some action and do not return a value.
    Expressions evaluate to a resultant value.

    Expressions do not include ending semicolons. If you add a semicolon to the end of an expression, you turn it into a statement, and it will then not return a value.
 */
    let x: i32 = function_five(5, -3);
    println!("return value of function5 = {x}");

}

fn function_two() { 
    println!("bonkers dude this is function_two");
}

fn function_four(var1: i32, var2: String){
    println!("var1 = {var1}");
    println!("var2 = {var2}");
}

fn function_five(x: i32, y: i32) -> i32{
    x+y
}