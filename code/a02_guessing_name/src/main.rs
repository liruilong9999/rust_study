use std::io;
use std::cmp::Ordering;
use rand::RngExt; 
 
fn main() {
    println!("猜数字！"); 
    let secret_number = rand::rng().random_range(1..=100);
    loop
    {
        println!("请输入数字：");
        let mut guess = String::new(); 
        io::stdin()
        .read_line(&mut guess)
        .expect("读取数字行失败"); 
        println!("你的数字为：{guess} ");
        let guess: u32 =match guess.trim().parse(){
            Ok(num)=>num,
            Err(_)=>continue,
        };
        match guess.cmp(&secret_number)
        {
            Ordering::Less=>println!("小了"),
            Ordering::Greater=>println!("大了"),
            Ordering::Equal=>{
                println!("一样");
                break;
            }
        }
    }
   

}
