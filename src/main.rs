use std::io::{stdin,stdout,Write};

enum Answer{
    Float(f32),
    NoAnswer,
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

        print!("Number2: ");
        stdout().flush().unwrap();
        stdin().read_line(&mut n2).unwrap();

        if let (Ok(n1),Ok(n2))=(n1.trim().parse::<f32>(),n2.trim().parse::<f32>()){
            let answer_option=match a.trim(){
                "/"=>Float(n1/n2),
                "+"=>Float(n1+n2),
                "-"=>Float(n1-n2),
                "*"=>Float(n1*n2),
                _=>NoAnswer,
            };

            match answer_option{
                Float(answer)=>println!("Result: {}",answer),
                NoAnswer=>println!("Encountered an unknown action: {}",a),
            }
        }else{
            println!("One or more invalid integers");
        }
    };
}