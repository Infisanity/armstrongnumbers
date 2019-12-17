use std::io;

fn main() {
    println!("Armstrong Numbers");
    
    
    loop {
        println!("Please input a number.");

        let mut num = String::new();

        io::stdin().read_line(&mut num)
            .expect("Failed to read line");

        let num: u32 = match num.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

            is_it_armstrong(num);

            fn is_it_armstrong(num: u32) -> () {
                let digits: Vec<u32> = num
                  .to_string()
                  .chars()
                  .map(|d| d.to_digit(10).unwrap())
                  .collect();
              
                let pow = digits.len() as u32;
                let sum: u32 = digits.iter().map(|d| d.pow(pow)).sum();
                if sum == num {
                    println!("{} is armstrong", num);
                }
                else {
                    println!("{} is not armstrong", num) ;
                }
            };    
            
        break;
    }

}
