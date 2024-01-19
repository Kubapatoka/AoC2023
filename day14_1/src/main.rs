use std::fs;


fn main() {
    let  file_path = "/home/jakub/Desktop/Rust/AoC2023/day14_1/dane.txt";
    let contents = fs::read_to_string(file_path)
            .expect("Should have been able to read the file");


    let mut board : Vec<Vec<char>> = contents.split("\n").map(|s| Vec::from_iter(s.to_string().chars()) ).collect();
    let tmp_len = board.len()-1;
    let _ =board.remove(tmp_len);
    

    let w = board[0].len();
    let h = board.len();

    for kol in 0..w
    {
        let mut new_pos = None;
        for i in 0..h
        {
            let c = board[i][kol];

            if c == '.' && new_pos == None
            {
                new_pos = Some(i);
            }

            if c == '#'
            {
                new_pos = None;
            }

            if c == 'O' && new_pos != None
            {
                board[new_pos.unwrap()][kol] = 'O';
                board[i][kol] = '.';

                if board[new_pos.unwrap() + 1][kol] != '.'
                {
                    new_pos = None;
                }
                else 
                {
                    new_pos = Some(new_pos.unwrap() + 1);
                }

            }
        }
    }

    let mut res = 0;

    for i in 0..h
    {
        let s = board[i].clone();

        res += (h-i)*s.iter().filter(|&c| c == &'O').count();
    }

    println!("{res}");
}