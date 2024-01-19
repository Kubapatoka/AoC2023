use std::fs;

fn do_thing(s : String) -> usize
{
    let p : Vec<Vec<char>> = s.split('\n').map(|st| Vec::from_iter(st.to_string().chars()) ).collect();

    let h = p.len();
    let w = p[0].len();

    let mut res = 0;

    //poziome lustra
    for i in 0..h-1
    {
        let mut g = true;
        let ile = std::cmp::min(i+1, h-(i+1));

        for j in 0..ile
        {
            if p[i-j] != p[i+j+1]
            {
                g = false;
            }
        }

        if g
        {
            res += 100*(i+1);
        }
    }

    //pionowe lustra


    for i in 0..w-1
    {
        let mut g = true;
        let ile = std::cmp::min(i+1, w-(i+1));

        for j in 0..ile
        {
            for k in 0..h
            {
                if p[k][i-j] != p[k][i+j+1]
                {
                    g = false;
                }
            }
        }

        if g
        {
            res += (i+1);
        }
    }


    res
}

fn main() {
    let  file_path = "/home/jakub/Desktop/Rust/AoC2023/day13_1/dane.txt";
    let contents = fs::read_to_string(file_path)
            .expect("Should have been able to read the file");


    let mut board : Vec<String> = contents.split("\n\n").map(|s| s.to_string() ).collect();
    let tmp_idx = board[board.len()-1].len() - 1 as usize;
    let tmp_len = board.len()-1;
    board[tmp_len].remove(tmp_idx);
    //println!("{}", board[board.len()-1]);
    //println!("{}", board[board.len()-2]);

    let dane = board.iter().map(|s| do_thing(s.clone())).collect::<Vec<usize>>();

    let res : usize = dane.iter().sum();

    println!("{res}");
}