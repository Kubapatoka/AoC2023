use std::env;
use std::fs;

fn main()
{

    //println!("In file {}", file_path);
    let  file_path = "/home/jakub/Desktop/Rust/AoC2023/day1/input.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");


    let tab = contents.split('\n')
                        .map(|x| String::from_iter(x.chars().filter(|&c| c >='0' && c <='9')) )
                        .collect::<Vec<String>>();


    let res = tab.iter()
                .map(|s|
                    if s.len() >= 1
                    {
                     return (s.chars().nth(0).unwrap().to_string().parse::<i32>().unwrap() )*10 + s.chars().nth_back(0).unwrap().to_string().parse::<i32>().unwrap() 
                    }
                    else if s.len() == 1
                    {
                        return 0
                        //return s.parse::<i32>().unwrap()
                    }
                    else
                    {
                        return 0
                    })
                .collect::<Vec<i32>>();

    for c in  res.clone()
    {
        println!("{c}");
    }

    let res1:i32 = res.iter()
                .sum();

    println!("{res1}");
}