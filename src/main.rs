use std::io;
use std::io::stdin;
use sha2::{Digest, Sha256};
use sha2::digest::DynDigest;

fn main() {
   //let (zero, num_quentity) = input();

    let mut num = 0;
    loop{
        num +=1;
        println!("{}", &num);
    }

}

fn input () -> (i64, i64){

    let mut imput_zero = String::new();
    println!("Введите колличество нулей");
    stdin().read_line(&mut imput_zero).expect("Err read str");
    let  imput_zero: i64 = imput_zero.trim().parse().expect("Err convert");

    let mut imput_num = String::new();
    println!("Введите колличество чисел");
    stdin().read_line(&mut imput_num).expect("Err read str");
    let imput_num: i64 = imput_num.trim().parse().expect("Err convert");

    (imput_zero, imput_num)
}
/*fn calc_hesh (zero: i64, num: String){
    let mut calc_hash = Sha256::new();
    calc_hash.update(num.to_string().parse().expect("Err parse to hash"));
    let res = calc_hash.finalize();
    println!("{:?}", res)

}*/

