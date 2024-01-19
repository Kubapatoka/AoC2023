use std::fs;

fn is_number(c :char) ->bool
{
    c >= '0' && c <= '9'
}


fn is_symbol(c :char) ->bool
{
    c == '*'
}


fn main() 
{
    let  file_path = "/home/jakub/Desktop/Rust/AoC2023/day_3_1/dane.txt";
    let contents = fs::read_to_string(file_path)
            .expect("Should have been able to read the file");


    let mut board : Vec<String> = contents.split('\n').map(|s| s.to_string()).collect();
    let _ = board.remove(board.len()-1);

    let w = board[0].len();
    let h = board.len();

    // let t1 = String::from(board[0].nth(7).unwrap());
    // let t2 = String::from(board[0].nth(0).unwrap());
    // let t3 = String::from(board[0].nth(0).unwrap());
    
    // println!("{t1} {t2} {t3}");

    let mut array = vec![vec![]; w*h];

    println!("wys: {h}, szer: {w}");

    let mut temp = 0;
    let mut pos_start = 0;

    for i in 0..h
    {
        for j in 0..w
        {

            let c = board[i].clone().chars().nth(j).unwrap();
                        println!("pwys: {i}, pszer: {j}, znak: {c}");


            if is_number(c)
            {
                if temp == 0 {pos_start = j;}
                temp = temp*10 + c.to_string().parse::<i32>().unwrap();
            }
            else if temp > 0
            {
                if is_symbol(c) {array[i*w+j].push(temp);}

                if pos_start > 0 {pos_start -= 1;}

                println!("POS_start: {pos_start}");

                if is_symbol(board[i].clone().chars().nth(pos_start).unwrap()) {array[i*w+pos_start].push(temp);}

                if i > 0
                {
                    for k in pos_start..=j
                    {
                        if is_symbol(board[i-1].clone().chars().nth(k).unwrap()) {array[i*w-w+k].push(temp);}
                    }
                }

                if i < h-1
                {
                    for k in pos_start..=j
                    {
                        //let t = String::from(board[i+1].as_str());
                        //println!("{t} {k}");
                        if is_symbol(board[i+1].clone().chars().nth(k).unwrap()) {array[i*w+w+k].push(temp);}
                    }
                }

                temp = 0;
            }
        }

        if temp > 0
        {
            if pos_start > 0 {pos_start -= 1;}

            if is_symbol(board[i].clone().chars().nth(pos_start).unwrap()) {array[i*w+pos_start].push(temp);}

            if i > 0
            {
                for k in pos_start..w
                {
                    if is_symbol(board[i-1].clone().chars().nth(k).unwrap()) {array[i*w-w+k].push(temp);}
                }
            }

            if i < h-1
            {
                for k in pos_start..w
                {
                    if is_symbol(board[i+1].clone().chars().nth(k).unwrap()) {array[i*w+w+k].push(temp);}
                }
            }
        }
    }

    let res :i32 = array.iter().filter(|&a| a.len() == 2).map(|a| a[0]*a[1]).sum();

    println!("{res}");
}
