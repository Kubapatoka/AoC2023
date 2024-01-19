use std::fs;


    //      3
    // 2 <= + => 0
    //      1


fn dfs(dane: Vec<Vec<char>>, i: usize, j: usize, d: usize, v:&mut Vec<Vec<Vec<bool>>>)
{
    let h = dane.len();
    let w = dane[0].len();

    if i >= h || j >= w || d > 3
    {
        return;
    }

    if v[i][j][d]
    {
        return;
    }

    v[i][j][d] = true;

    let c = dane[i][j];

    let mut nx = j as i32;
    let mut ny = i as i32;
    let mut nd = d;


    if c == '.'
    {
        if d == 0
        {
            nx += 1;
        }
        else if d == 1
        {
            ny += 1;
        }
        else if d == 2
        {
            nx -= 1;
        }
        else if d == 3
        {
            ny -= 1;
        }

        if nx >= 0 && ny >= 0
        {
            dfs(dane.clone(), ny as usize, nx as usize, nd,v);
        }
    }

    if c == '/'
    {
        if d == 0
        {
            ny -= 1;
            nd = 3;
        }
        else if d == 1
        {
            nx -= 1;
            nd = 2;
        }
        else if d == 2
        {
            ny += 1;
            nd = 1;
        }
        else if d == 3
        {
            nx += 1;
            nd = 0;
        }

        if nx >= 0 && ny >= 0
        {
            dfs(dane.clone(), ny as usize, nx as usize, nd,v);
        }
    }

    if c == '\\'
    {
        if d == 0
        {
            ny += 1;
            nd = 1;
        }
        else if d == 1
        {
            nx += 1;
            nd = 0;
        }
        else if d == 2
        {
            ny -= 1;
            nd = 3;
        }
        else if d == 3
        {
            nx -= 1;
            nd = 2;
        }

        if nx >= 0 && ny >= 0
        {
            dfs(dane.clone(), ny as usize, nx as usize, nd,v);
        }
    }

    if c == '|'
    {
        if d == 0 || d == 2
        {
            dfs(dane.clone(), i+1, j, 1,v);
            if i > 0 {dfs(dane.clone(), i-1, j, 3,v);}
        }
        else
        {
            if d == 1
            {
                ny += 1;
            }
            else if d == 3
            {
                ny -= 1;
            }
    
            if nx >= 0 && ny >= 0
            {
                dfs(dane.clone(), ny as usize, nx as usize, nd,v);
            }
        }
    }

    if c == '-'
    {
        if d == 1 || d == 3
        {
            dfs(dane.clone(), i, j+1, 0,v);
            if j > 0 {dfs(dane.clone(), i, j-1, 2,v);}
        }
        else
        {
            if d == 0
            {
                nx += 1;
            }
            else if d == 2
            {
                nx -= 1;
            }
    
            if nx >= 0 && ny >= 0
            {
                dfs(dane.clone(), ny as usize, nx as usize, nd,v);
            }
        }
    }
}

fn main() {
    let  file_path = "/home/jakub/Desktop/Rust/AoC2023/day16_1/dane.txt";
    let contents = fs::read_to_string(file_path)
            .expect("Should have been able to read the file");


    let mut board : Vec<String> = contents.split('\n').map(|s| s.to_string() ).collect();
    let _ = board.remove(board.len()-1);
    //let _ = board.remove(board.len()-1);
    let dane = board.iter().map(|s| Vec::from_iter(s.chars()) ).collect::<Vec<Vec<char>>>();

    let h = dane.len();
    let w = dane[0].len();
    // println!("dane w: {w}, h: {h}");
    // for i in 0..h
    // {
    //     println!("{}", String::from_iter(dane[i].iter()));
    // }


        //      3
        // 2 <= + => 0
        //      1

    let mut real_res = 0;

    for k in 0..h
    {
        let mut vis = vec![vec![vec![false; 4]; w]; h];

        dfs(dane.clone(), k,0,0, &mut vis);

        let mut res = 0;

        for i in vis
        {
            res += i.iter().filter(|&b| b[0] || b[1] || b[2] || b[3]).count();
        }

        real_res = std::cmp::max(real_res, res);

    }


    for k in 0..h
    {
        let mut vis = vec![vec![vec![false; 4]; w]; h];

        dfs(dane.clone(), k,w-1,2, &mut vis);

        let mut res = 0;

        for i in vis
        {
            res += i.iter().filter(|&b| b[0] || b[1] || b[2] || b[3]).count();
        }

        real_res = std::cmp::max(real_res, res);
    }

    
    for k in 0..w
    {
        let mut vis = vec![vec![vec![false; 4]; w]; h];

        dfs(dane.clone(), 0,k,1, &mut vis);

        let mut res = 0;

        for i in vis
        {
            res += i.iter().filter(|&b| b[0] || b[1] || b[2] || b[3]).count();
        }

        real_res = std::cmp::max(real_res, res);
    }


    for k in 0..w
    {
        let mut vis = vec![vec![vec![false; 4]; w]; h];

        dfs(dane.clone(), h-1,k,3, &mut vis);

        let mut res = 0;

        for i in vis
        {
            res += i.iter().filter(|&b| b[0] || b[1] || b[2] || b[3]).count();
        }

        real_res = std::cmp::max(real_res, res);
    }


    println!("{real_res}");
}