use std::fs;

fn dfs(mut dane: Vec<Vec<char>>, i: usize, j: usize) -> Vec<Vec<char>>
{
    let h = dane.len();
    let w = dane[0].len();

    if i != 0
    {
        let c = dane[i-1][j];
        if c == '.'
        {
            dane[i-1][j] = '1';
            dane = dfs(dane, i-1, j);
        }
    }

    if i != h-1
    {
        let c = dane[i+1][j];
        if c == '.'
        {
            dane[i+1][j] = '1';
            dane = dfs(dane, i+1, j);
        }
    }

    if j != 0
    {
        let c = dane[i][j-1];
        if c == '.'
        {
            dane[i][j-1] = '1';
            dane = dfs(dane, i, j-1);
        }
    }

    if j != w-1
    {
        let c = dane[i][j+1];
        if c == '.'
        {
            dane[i][j+1] = '1';
            dane = dfs(dane, i, j+1);
        }
    }

    if i != 0 && j != 0
    {
        let c = dane[i-1][j-1];
        if c == '.'
        {
            dane[i-1][j-1] = '1';
            dane = dfs(dane, i-1, j-1);
        }
    }

    if i != 0 && j != w-1
    {
        let c = dane[i-1][j+1];
        if c == '.'
        {
            dane[i-1][j+1] = '1';
            dane = dfs(dane, i-1, j+1);
        }
    }

    if i != h-1 && j != 0
    {
        let c = dane[i+1][j-1];
        if c == '.'
        {
            dane[i+1][j-1] = '1';
            dane = dfs(dane, i+1, j-1);
        }
    }

    if i != h-1 && j != w-1
    {
        let c = dane[i+1][j+1];
        if c == '.'
        {
            dane[i+1][j+1] = '1';
            dane = dfs(dane, i+1, j+1);
        }
    }

    dane
}

fn main() {
    let  file_path = "/home/jakub/Desktop/Rust/AoC2023/day10_1/dane.txt";
    let contents = fs::read_to_string(file_path)
            .expect("Should have been able to read the file");


    let mut board : Vec<String> = contents.split('\n').map(|s| s.to_string() ).collect();
    let _ = board.remove(board.len()-1);
    let mut dane = board.iter().map(|s| Vec::from_iter(s.chars()) ).collect::<Vec<Vec<char>>>();

    let mut res = 0;

    let h = dane.len();
    let w = dane[0].len();

    let mut pos = (0,0);

    for i in 0..h
    {
        for j in 0..w
        {
            if dane[i][j] == 'S'
            {
                pos = (i,j);
            }
        }
    }

    dane[pos.0][pos.1] = 'x';
    pos.1 += 1;

    loop
    {
        //res += 1;

        let c = dane[pos.0][pos.1];
        dane[pos.0][pos.1] = 'x';

        match c
        {
            '|' =>
            {
                if dane[pos.0-1][pos.1] != 'x'
                {
                    pos.0-=1;
                }
                else 
                {
                    pos.0+=1;
                }
            }
            '-' =>
            {
                if dane[pos.0][pos.1-1] != 'x'
                {
                    pos.1-=1;
                }
                else 
                {
                    pos.1+=1;
                }
            }
            'L' =>
            {
                if dane[pos.0-1][pos.1] != 'x'
                {
                    pos.0-=1;
                }
                else 
                {
                    pos.1+=1;
                }
            }
            'J' =>
            {
                if dane[pos.0-1][pos.1] != 'x'
                {
                    pos.0-=1;
                }
                else 
                {
                    pos.1-=1;
                }
            }
            '7' =>
            {
                if dane[pos.0+1][pos.1] != 'x'
                {
                    pos.0+=1;
                }
                else 
                {
                    pos.1-=1;
                }
            }
            'F' =>
            {
                if dane[pos.0+1][pos.1] != 'x'
                {
                    pos.0+=1;
                }
                else 
                {
                    pos.1+=1;
                }
            }
            _ => panic!()
        }

        if dane[pos.0][pos.1] == 'x'
        {
            break;
        }

    }

    for i in 0..h
    {
        for j in 0..w
        {
            if dane[i][j] != 'x'
            {
                dane[i][j] = '.';
            }
            //let c = dane[i][j];
            //print!("{c}");
        }
        //print!("\n");
    }

    for i in 0..h
    {
        if dane[i][0] == '.'
        {
            dane[i][0] = '1';
            dane = dfs(dane, i , 0);
        }

        if dane[i][w-1] == '.'
        {
            dane[i][w-1] = '1';
            dane = dfs(dane, i , w-1);
        }
    }


    for j in 0..w
    {
        if dane[0][j] == '.'
        {
            dane[0][j] = '1';
            dane = dfs(dane, 0 , j);
        }

        if dane[h-1][j] == '.'
        {
            dane[h-1][j] = '1';
            dane = dfs(dane, h-1 , j);
        }
    }

    for i in 0..h
    {
        for j in 0..w
        {
            let c = dane[i][j];
            if c == '.'
            {
                res +=1;
            }
            print!("{c}");
        }
        print!("\n");
    }


    println!("{res}");
}