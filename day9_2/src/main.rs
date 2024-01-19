use std::fs;

fn all_the_same(v: Vec<i32>) -> bool
{
    let mut res = true;
    let d = v[0];
    for e in v
    {
        if e != d
        {
            res = false;
        }
    }
    res
}

fn next_el(v: Vec<i32>) -> i32
{
    if all_the_same(v.clone())
    {
        return v[0];
    }

    let mut diffs = vec![];

    let dl = v.len();
    for e in 1..dl
    {
        diffs.push(v[e]-v[e-1]);
    }

    v[0]-next_el(diffs)
}



fn main() {
    let  file_path = "/home/jakub/Desktop/Rust/AoC2023/day9_1/dane.txt";
    let contents = fs::read_to_string(file_path)
            .expect("Should have been able to read the file");


    let mut board : Vec<String> = contents.split('\n').map(|s| s.to_string() ).collect();
    let _ = board.remove(board.len()-1);
    let dane = board.iter().map(|s| s.split(' ').map(|s| s.to_string().parse::<i32>().unwrap()).collect::<Vec<i32>>() ).collect::<Vec<Vec<i32>>>();

    let mut res = 0;

    for d in dane
    {
        let next = next_el(d);
        res += next;
    }

    println!("{res}");
}