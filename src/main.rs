use std::io;
use std::io::stdin;
use std::task::Context;
use sha2::{Digest, Sha256};
use sha2::digest::{DynDigest, Output, Update};
use sha2::digest::core_api::CoreWrapper;

fn main() {
   //let (zero, num_quentity) = input();
    let mut str_hash_num = String::new();
    let mut num: i64  = 0;
    calc_hesh()
    /*loop{
        num +=1;
        let  byte_str_num = num.to_le_bytes();
        let res_hash = calc_hesh(&byte_str_num);
        //calc_last_num_hash(res_hash);
        /*for i in res_hash{
            //if i != 0 {
                str_hash_num.push(i.to_string().parse().unwrap());
                println!("{}", &str_hash_num);
            //}
        }*/

    }*/

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
fn calc_hesh ()//(num: &[u8]) -> [u8; 32]
{
    /*let mut calc_hash = Sha256::new();
    Digest::update(&mut calc_hash, &num);
    let res =calc_hash.finalize();
    let mut re_out = [0; 32];
    re_out.copy_from_slice(&res);
    println!("{:x?}", &re_out);*/

    let mut bb = 1;
    let bb: String = bb.to_string().parse().unwrap();
    let mut hasher = Sha256::new();
    Digest::update(&mut hasher, &bb);
    let result = hasher.finalize();
    println!("{:?}", result);
    //re_out
}
/*fn calc_last_num_hash  (res_hash: Output<i64>){
    //println!("{:?}", array);
    let var = res_hash.to_string().expect("Err hash to string");

}*/

