use std::{time::Instant, env::args};

use number_triangle::number_triangle as trig;

fn main(){
    let mut args = args().skip(1);
    let size = args.next().unwrap();
    let size = size.parse().unwrap();
    let func:String = args.next().unwrap();
    let t = trig::create_triangle(size);
    t.print();
    if func==String::from("r")||func==String::from("R"){
        let time = Instant::now();
        let max = t.solve_recursively();
        println!("Took {:?} to solve a triangle of size {}",time.elapsed(),size);
        t.indexed_print(max);
    }else if func==String::from("d")||func==String::from("D"){
        let time = Instant::now();
        let max = t.solve_dynamically();
        println!();
        println!("Took {:?} to solve a triangle of size {}",time.elapsed(),size);
        t.indexed_print(max);
    }else{
        println!("Specify algorithm type");
    }
}