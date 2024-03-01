use std::io::{stdin,stdout,Write};

fn main() {
    loop{
        let mut n1 = String::new();
        let mut n2 = String::new();
        let mut a = String::new();

        print!("Number1: ");
        let _=stdout().flush();
        stdin().read_line(&mut n1);

        print!("Action: ");
        let _=stdout().flush();
        stdin().read_line(&mut a);

        print!("Number2: ");
        let _=stdout().flush();
        stdin().read_line(&mut n2);

        let (n1,n2)=(n1.trim().parse::<f32>().unwrap(),n2.trim().parse::<f32>().unwrap());

        let answer=match a.trim(){
            "/"=>n1/n2,
            "+"=>n1+n2,
            "-"=>n1-n2,
            "*"=>n1*n2,
            _=>0.0,
        };

        println!("Result: {}",answer);
    };
}