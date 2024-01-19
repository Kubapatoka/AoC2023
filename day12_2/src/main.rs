use std::fs;

fn rec(wip : Vec<char>, n: usize,mut num: Vec<i32>, pat : Vec<char>) -> Vec<Vec<char>>
{

    // print!("nums: {} ", num.len());

    // for i in wip.clone()
    // {
    //     print!("{i}");
    // }
    // print!("\n");


    if num.is_empty()
    {
        let wip_1 = wip.iter().map(|&c| if c == '*' {return '.'} else {return c}).collect::<Vec<char>>();
        let mut ok = true;
        let l = pat.len();
        for k in 0..l
        {
            if pat[k] != '?' && pat[k] != wip_1[k]
            {
                ok = false;
                // println!("roznica na k: {k}");
            }
        }
        
        // println!("{ok}");
        // for i in wip_1.clone()
        // {
        //     // print!("{i}");
        // }
        // print!("\n");

        if ok
        {
            return vec![wip; 1];
        }
        return vec![];
    }

    let dl = wip.len();

    let min_dl : i32 = num.iter().sum::<i32>() + num.len() as i32 - 1i32;
    let fi = num[0].clone() as usize;
    let _ = num.remove(0);

    let mut res :Vec<Vec<char>> = vec![];

    let pos_opt = dl as i32 - min_dl as i32 - n as i32;

    if pos_opt <= 0
    {
        return vec![];
    }

    for i in n ..n+pos_opt as usize
    {
        let mut wip_cpy = wip.clone();

        for j in n..i
        {
            wip_cpy[j] = '.';
        }

        for j in i..i+fi
        {
            wip_cpy[j] = '#';
        }

        let mut last = i+fi-1;
        if i+fi < dl
        {
            wip_cpy[i+fi] = '.';
            last += 1;
        }

        let mut ok = true;
        for k in 0..=last
        {
            if pat[k] != '?' && pat[k] != wip_cpy[k]
            {
                ok = false;
            }
        }

        if ok
        {
            let res1 = rec(wip_cpy.clone(), last+1, num.clone(), pat.clone());
            res.extend(res1);
        }


    }

    res
}



fn do_thing(s : String) -> usize
{
    let tmp : Vec<_> =  s.split(' ').collect();

    let patt : Vec<char> = Vec::from_iter(tmp[0].to_string().chars());
    let nums : Vec<i32>  = tmp[1].to_string().split(',').map(|c| c.parse::<i32>().unwrap() ).collect::<Vec<i32>>();

    let mut new_patt = patt.clone();
    new_patt.extend(patt.clone().iter());
    new_patt.extend(patt.clone().iter());
    new_patt.extend(patt.clone().iter());
    new_patt.extend(patt.clone().iter());

    

    let mut new_nums = nums.clone();
    new_nums.extend(nums.clone().iter());
    new_nums.extend(nums.clone().iter());
    new_nums.extend(nums.clone().iter());
    new_nums.extend(nums.clone().iter());


    let p = new_patt.len();
    let _n = nums.len();

    let possible_strings = rec(vec!['*'; p], 0, new_nums.clone(), new_patt.clone() );

    possible_strings.len()
}

fn main() {
    let  file_path = "/home/jakub/Desktop/Rust/AoC2023/day12_1/dane.txt";
    let contents = fs::read_to_string(file_path)
            .expect("Should have been able to read the file");

    //let contents = "???#??.?#??.? 2,2\n";

    let mut board : Vec<String> = contents.split('\n').map(|s| s.to_string() ).collect();
    let _ = board.remove(board.len()-1);
    let dane = board.iter().map(|s| do_thing(s.clone())).collect::<Vec<usize>>();



    let res : usize = dane.iter().sum();


    println!("{res}");
}