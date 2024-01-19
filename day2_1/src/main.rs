use std::env;
use std::fs;
use std::cmp::max;


fn main() 
{
    let  file_path = "/home/jakub/Desktop/Rust/AoC2023/day2_1/dane.txt";
    let contents = fs::read_to_string(file_path)
            .expect("Should have been able to read the file");
    

    let tab = contents.split('\n')
        .map(|s| s.split(|c| c == ':' || c == ';').collect::<Vec<_>>())
        //.map(|v| v[0]=String::from_iter(v[0].chars().filter(|&c| c >= '0' && c <= '9')).as_str()  )
        .collect::<Vec<_>>();

        struct Game
        {
            id : i32,
            r: i32,
            g: i32,
            b: i32,
        }

    let mut array : Vec<Game> = vec![];

    for t in tab
    {
        let id = String::from_iter(t[0].chars().filter(|&c| c >= '0' && c <= '9')).parse::<i32>().unwrap_or(0);
        println!("{id}");
        let mut r : i32 = 0;
        let mut g : i32 = 0;
        let mut b : i32 = 0;

        for e in t
        {
            let es :Vec<_> = e.split(',').collect();
            for s in  es
            {
                if s.contains("blue") { b = max(b, String::from_iter(s.chars().filter(|&c| c >= '0' && c <= '9')).parse::<i32>().unwrap())}


                if s.contains("red") { r = max(r, String::from_iter(s.chars().filter(|&c| c >= '0' && c <= '9')).parse::<i32>().unwrap())}


                if s.contains("green") { g = max(g, String::from_iter(s.chars().filter(|&c| c >= '0' && c <= '9')).parse::<i32>().unwrap())}
            }
        }

        array.push(Game { id: (id), r: (r), g: (g), b: (b) })
    }

    let res : i32 = array.iter().filter(|&x| x.r<=12 && x.g<=13 && x.b<=14).map(|d| d.id).sum();

    println!("{res}")
}
