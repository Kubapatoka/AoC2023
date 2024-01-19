use std::{fs, collections::HashMap};


fn main() {
    let  file_path = "/home/jakub/Desktop/Rust/AoC2023/day11_1/dane.txt";
    let contents = fs::read_to_string(file_path)
            .expect("Should have been able to read the file");


    let mut board : Vec<String> = contents.split('\n').map(|s| s.to_string() ).collect();
    let _ = board.remove(board.len()-1);
    let dane = board.iter().map(|s| Vec::from_iter(s.chars()) ).collect::<Vec<Vec<char>>>();

    let mut galactics = vec![];

    let h = dane.len();
    let w = dane[0].len();


    for i in 0..h
    {
        for j in 0..w
        {
            if dane[i][j] == '#'
            {
                galactics.push((i,j));
            }
        }
    }

    let mut map_h = HashMap::new();
    let mut map_w = HashMap::new();

    let mut ix = 0;
    for i in 0..h
    {
        if !dane[i].contains(&'#')
        {
            ix += 1000000-1;
        }
        map_h.insert(i, ix);

        ix+=1;
    }

    let mut iy = 0;
    for j in 0..w
    {
        let mut empty = true;
        for i in 0..h
        {
            if dane[i][j] == '#'
            {
                empty = false;
                break;
            }
        }

        if empty
        {
            iy+=1000000-1;
        }
        
        map_w.insert(j, iy);

        iy+=1;
    }

    let mut res = 0;
    let num = galactics.len();
    //println!("{num}");


    for i in 0..num
    {
        let (a,b) = galactics[i];
        for j in i+1..num
        {
            let (x,y) = galactics[j];

            res += i64::abs(map_h[&a]-map_h[&x]) + i64::abs(map_w[&b]-map_w[&y]);
        }
    }

    println!("{res}");
}