use std::io;

fn main() {
    println!("Hello, world!");

    let mut mode: String = String::new();
    

    let convert_func = loop{

        println!("Choose [0]F->C or [1]C->F: ");
        io::stdin()
            .read_line(&mut mode)
            .expect("Failed to read line");

        let mode: u32 = match mode.trim().parse() { // parse returns a result enum which is either of type Ok or Err and it internally has a value
            Ok(num) => num,
            Err(_) => {
                println!("Failed to Parse mode integer");
                continue;
            },
        };

        if mode == 0{
            break f_to_c as fn(f32) -> f32;
        } else if mode == 1{
            break c_to_f as fn(f32) -> f32;
        } else {
            println!("Invalid Mode Selected")
        }
    };

    let mut temp_to_convert: String = String::new();

    loop{
        let temp_2_convert: f32 = loop{
            println!("Enter temperature you would like converted: ");
            io::stdin()
                .read_line(&mut temp_to_convert)
                .expect("Failed to read line");

            let temp: f32 = match temp_to_convert.trim().parse() { // parse returns a result enum which is either of type Ok or Err and it internally has a value
                Ok(num) => num,
                Err(_) => {
                    println!("Failed to Parse temperature float");
                    continue;
                },
            };

            break temp;
        };

        let converted_temp: f32 = convert_func(temp_2_convert);
        println!("{temp_2_convert} -> {converted_temp}");
    }


}

fn f_to_c(f: f32) -> f32{
    (f - 32.0)*5.0/9.0
}

fn c_to_f(c: f32) -> f32{
    (c*9.0/5.0)+32.0
}