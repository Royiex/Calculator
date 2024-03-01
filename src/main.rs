use std::io::{stdin,stdout,Write};

enum Answer{
    Float(f32),
    Bool(bool),
    NoAnswer,
}

fn main() {
    let allowed_actions={
        let mut actions=std::collections::HashMap::new();
        actions.insert("/");
        actions.insert("+");
        actions.insert("-");
        actions.insert("*");
        actions.insert("=");
        //implicitly return actions into the allowed_actions variable
        actions
    };

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

        if !allowed_actions.contains(a.trim().as_str()){
            println!("Illegal action!");
        }

        print!("Number2: ");
        stdout().flush().unwrap();
        stdin().read_line(&mut n2).unwrap();

        if let (Ok(n1),Ok(n2))=(n1.trim().parse::<f32>(),n2.trim().parse::<f32>()){
            let answer_option=match a.trim(){
                "/"=>Answer::Float(n1/n2),
                "+"=>Answer::Float(n1+n2),
                "-"=>Answer::Float(n1-n2),
                "*"=>Answer::Float(n1*n2),
                "="=>Answer::Bool(n1==n2),
                _=>Answer::NoAnswer,
            };

            match answer_option{
                Answer::Float(answer)=>println!("Result: {}",answer),
                Answer::Bool(boolean)=>println!("Bool result: {}",boolean),
                Answer::NoAnswer=>println!("Encountered an unknown action: {}",a),
            }
        }else{
            println!("One or more invalid integers");
        }
    };
}