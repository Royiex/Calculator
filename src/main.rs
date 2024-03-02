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
    ArcSin,
    ArcCos,
    ArcTan,
    Div,
    Add,
    Sub,
    Mul,
    Exp,
    Sqrt,
    Equal,
    Percent,
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
            "asin"=>MathType::ArcSin,
            "acos"=>MathType::ArcCos,
            "atan"=>MathType::ArcTan,
            "/"=>MathType::Div,
            "+"=>MathType::Add,
            "-"=>MathType::Sub,
            "*"=>MathType::Mul,
            "exp"=>MathType::Exp,
            "sqrt"=>MathType::Sqrt,
            "="=>MathType::Equal,
            "%"=>MathType::Percent,
            _=>MathType::NoMath,
        };

        let one_arg=match &math_option{
            MathType::Sin|MathType::Cos|MathType::Tan|MathType::ArcSin|MathType::ArcCos|MathType::ArcTan|MathType::Sqrt=>true,
            _=>false,
        };

        //reach all the way into the if statement tree and grab answer_option via implicit return
        let answer_option=if one_arg{
            //one arg
            if let Ok(n1)=n1.trim().parse::<f32>(){
                match math_option{
                    MathType::Sin=>Answer::Float(n1.sin()),
                    MathType::Cos=>Answer::Float(n1.cos()),
                    MathType::Tan=>Answer::Float(n1.tan()),
                    MathType::ArcSin=>Answer::Float(n1.asin()),
                    MathType::ArcCos=>Answer::Float(n1.acos()),
                    MathType::ArcTan=>Answer::Float(n1.atan()),
                    MathType::Sqrt=>Answer::Float(n1.sqrt()),
                    _=>Answer::NoAnswer,
                }
            }
            else{
                //return no answer if parsing fails
                Answer::NoAnswer
            }
        }
        else{
            //two args
            print!("Number2: ");
            stdout().flush().unwrap();
            stdin().read_line(&mut n2).unwrap();

            if let (Ok(n1),Ok(n2))=(n1.trim().parse::<f32>(),n2.trim().parse::<f32>()){
                match math_option{
                    MathType::Div=>Answer::Float(n1/n2),
                    MathType::Add=>Answer::Float(n1+n2),
                    MathType::Sub=>Answer::Float(n1-n2),
                    MathType::Mul=>Answer::Float(n1*n2),
                    MathType::Exp=>Answer::Float(n1.powf(n2)),
                    MathType::Equal=>Answer::Bool(n1==n2),
                    MathType::Percent=>Answer::Float((n1/100.0)*n2),
                    _=>Answer::NoAnswer,
                }
            }else{
                //return no answer if parsing fails
                Answer::NoAnswer
            }
        };
        match answer_option{
            Answer::Float(answer)=>println!("Result: {}",answer),
            Answer::Bool(boolean)=>println!("Bool result: {}",boolean),
            Answer::NoAnswer=>println!("Encountered an unknown action: {}",a),
        }
    };
}