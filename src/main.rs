use std::io;
use std::io::stdin;
use std::task::Context;
use sha2::{Digest, Sha256};
use sha2::digest::{DynDigest, Output, Update};
use sha2::digest::core_api::CoreWrapper;

fn main() {
   //let (zero, num_quentity) = input();
    let mut res_hash;
    let mut num: i64 = 0;
    loop{
        num +=1;
        res_hash = calc_hesh(num);
        //calc_last_num_hash(res_hash);
        println!("{:x}", &res_hash);
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
fn calc_hesh <T: sha2::digest::OutputSizeUser> (num: T) ->Output<T>  {
    let mut calc_hash = Sha256::new();
    let num = num.to_string();
    //println!("{}", &num);
    Digest::update(&mut calc_hash, &num);
    let res =calc_hash.finalize();
    let res = match res{
        Ok(res)=> res,
        Err(e)=> panic!("Err calc hash {}", e)
    };
    //println!("{}, {:x}",num, &res);
    res
}
/*fn calc_last_num_hash  (res_hash: Output<i64>){
    //println!("{:?}", array);
    let var = res_hash.to_string().expect("Err hash to string");

}*/

