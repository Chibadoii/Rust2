use std::io;
use std::io::stdin;
use std::task::Context;
use sha2::{Digest, Sha256};
use sha2::digest::{DynDigest, Output, Update};
use sha2::digest::core_api::CoreWrapper;

fn main() {
   let (zero, num_quentity) = input();
    let mut str_hash_num = String::new();
    let mut num: i64  = 0;
    let mut quantity = 0;
    loop{
        num +=1;
        str_hash_num = num.to_string();
        let res_hash = calc_hesh(str_hash_num);
        println!("{}", &res_hash);

        let (num_zero) =iter_char_hash((&res_hash).to_string(),zero);

        if num_zero {
            println!("{} ", num);
            println!("{}", res_hash);
            quantity +=1;
        }
        if quantity == num_quentity {
            break
        }

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
fn calc_hesh (num: String) -> String
{
    let hash = Sha256::new().chain(num).finalize();
    let s = format!("{:x}", hash);
    println!("{}", &s);

    s
}
fn iter_char_hash (res_hash: String, zero: i64) -> bool {
    let mut res_num= false;
    //Кол во
    let mut num_zero = 0;
    let char_zero = '0';

        for c in res_hash.chars().rev() {
            if c == char_zero {
                num_zero += 1;
            } else { break; }

    }
    if num_zero == zero{
        res_num = true;
    }

    //вывод счетчика для завершения луп
    res_num
}
