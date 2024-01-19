use std::env;
use std::fs;
use std::vec;



fn convert(s : &str) -> i32
{

    let mut arr1 = vec![];
    let mut arr2 = vec![];

    let T = [("one",1), ("eight",8), ("two",2), ("three",3), ("four",4), ("five",5), ("six",6), ("seven",7),  ("nine",9)];

    let mut s1 = s.to_string();
    let binding = String::from_iter(s.chars().rev());
    let mut s2 = binding;

    arr1.push(s1.clone());
    arr2.push(s2.clone());

    for t in T
    {
        arr1.push(s1.clone().replacen(t.0, t.1.to_string().as_str(), 1));
        arr2.push(s2.clone().replacen(String::from_iter(t.0.chars().rev()).as_str(), t.1.to_string().as_str(), 1));
    }

    let a1 : Vec<_> = arr1.iter()
    .map(|a| a.chars().enumerate().find(|(_a,c)| c >=&'0' && c <=&'9').unwrap_or((0,'0'))).collect();


    let a2 : Vec<_> = arr2.iter()
    .map(|a| a.chars().enumerate().find(|(_a,c)| c >=&'0' && c <=&'9').unwrap_or((0,'0'))).collect();

    let p1 = a1.iter().min_by(|(xa, _xb),(ya,_yb)| xa.cmp(ya)).unwrap();
    let p2 = a2.iter().min_by(|(xa, _xb),(ya,_yb)| xa.cmp(ya)).unwrap();

    // p1.1.to_string().parse().unwrap() *10 + 


    // let a = s1.chars().find(|&c| c >='0' && c <='9');
    // let b = s2.chars().find(|&c| c >='0' && c <='9');

    // if a == None && b == None {return 0}

    // if a == None {return b.unwrap().to_string().parse::<i32>().unwrap()*11 }

    // if b == None {return a.unwrap().to_string().parse::<i32>().unwrap()*11 }

    p1.1.to_string().parse::<i32>().unwrap()*10+p2.1.to_string().parse::<i32>().unwrap()
}

fn main()
{

    //println!("In file {}", file_path);
    let  file_path = "/home/jakub/Desktop/Rust/AoC2023/day1/input.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");


    let tab = contents.split('\n').map(|s| convert(s)).collect::<Vec<i32>>();

    for x in tab.clone()
    {
        println!("{x}");
    }


    let res1:i32 = tab.iter().sum();

    println!("{res1}");
}