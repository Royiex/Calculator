use std::io::{stdin,stdout,Write};

fn main() {
    loop{
        let mut n1 = String::new();
        let mut n2 = String::new();
        let mut a = String::new();

        print!("Number1: ");
        stdout().flush().unwrap();
        stdin().read_line(&mut n1);

        print!("Action: ");
        stdout().flush().unwrap();
        stdin().read_line(&mut a);

        print!("Number2: ");
        stdout().flush().unwrap();
        stdin().read_line(&mut n2);

        if let (Ok(n1),Ok(n2))=(n1.trim().parse::<f32>(),n2.trim().parse::<f32>()){
            let answer=match a.trim(){
                "/"=>n1/n2,
                "+"=>n1+n2,
                "-"=>n1-n2,
                "*"=>n1*n2,
                _=>0.0,
            };

            println!("Result: {}",answer);
        }else{
            println!("One or more invalid integers");
        }
    };
}