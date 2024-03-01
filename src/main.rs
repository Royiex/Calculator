use std::io::{stdin,stdout,Write};

enum Answer{
    Float(f32),
    Bool(bool),
    NoAnswer,
}

enum MathType{
    Sin,
    Cos,
    Tan,
    Div,
    Add,
    Sub,
    Mul,
    Equal,
    NoMath,
}

fn main() {

    loop{
        let mut n1 = String::new();
        let mut n2 = String::new();
        let mut a = String::new();

        print!("Number1: ");
        stdout().flush().unwrap();
        stdin().read_line(&mut n1).unwrap();

        print!("Action: ");
        stdout().flush().unwrap();
        stdin().read_line(&mut a).unwrap();

        let math_option=match a.trim(){
            "sin"=>MathType::Sin,
            "cos"=>MathType::Cos,
            "tan"=>MathType::Tan,
            "/"=>MathType::Div,
            "+"=>MathType::Add,
            "-"=>MathType::Sub,
            "*"=>MathType::Mul,
            "="=>MathType::Equal,
            _=>MathType::NoMath,
        };
       
        print!("Number2: ");
        stdout().flush().unwrap();
        stdin().read_line(&mut n2).unwrap();

        if let (Ok(n1),Ok(n2))=(n1.trim().parse::<f32>(),n2.trim().parse::<f32>()){
            let answer_option=match math_option{
                MathType::Sin=>Answer::Float(n1.sin()),
                MathType::Cos=>Answer::Float(n1.cos()),
                MathType::Tan=>Answer::Float(n1.tan()),
                MathType::Div=>Answer::Float(n1/n2),
                MathType::Add=>Answer::Float(n1+n2),
                MathType::Sub=>Answer::Float(n1-n2),
                MathType::Mul=>Answer::Float(n1*n2),
                MathType::Equal=>Answer::Bool(n1==n2),
                _=>Answer::NoAnswer,
            };

            match answer_option{
                Answer::Float(answer)=>println!("Result: {}",answer),
                Answer::Bool(boolean)=>println!("Bool result: {}",boolean),
                Answer::NoAnswer=>println!("Encountered an unknown action: {}",a),
            }
        }
        else{
            println!("One or more invalid integers");
        }
    };
}