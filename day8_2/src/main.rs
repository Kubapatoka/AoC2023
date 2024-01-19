//use std::{fs, collections::HashMap};

use std::cmp::min;

// fn create_map(v : Vec<String>) -> (HashMap<String, (String,String)>, Vec<String>)
// {
//     let mut res = HashMap::new();
//     let mut res2 = vec![];

//     for s in v
//     {
//         let k = s[..=2].to_string();
//         res.insert(k.clone(), (s[7..=9].to_string(), s[12..=14].to_string()));
//         if k.ends_with('A')
//         {
//             println!("{k}");
//             res2.push(k.clone());
//         }
//     }

//     (res,res2)
// }



fn main() {
    // let  file_path = "/home/jakub/Desktop/Rust/AoC2023/day8_1/dane.txt";
    // let contents = fs::read_to_string(file_path)
    //         .expect("Should have been able to read the file");


    // let mut board : Vec<String> = contents.split('\n').map(|s| s.to_string()).collect();

    // let _ = board.remove(board.len()-1);
    // let polecenia = board[0].clone().chars().map(|c| match c {'L' => 0, 'R' => 1, _ => -1}).collect::<Vec<_>>();
    // let _ = board.remove(0);
    // let _ = board.remove(0);

    // let (map,mut  pos) = create_map(board);

    // let len_of_pos = pos.len();

    // println!("ilosc poz startowych: {len_of_pos}");

    // let mut res = 0;
    // for &i in polecenia.iter().cycle()
    // {
    //     let mut br = true;
    //     for p in 0..len_of_pos
    //     {
    //         let new_pos = &map[&pos[p]];
    //         if i == 0
    //         {
    //             pos[p] = new_pos.0.clone();
    //         }
    //         else
    //         {
    //             pos[p] = new_pos.1.clone();
    //         }
    //     }

    //     for p in pos.clone()
    //     {
    //         //  println!("{p}");
    //         if !p.ends_with('Z')
    //         {
    //             br = false;
    //         }
    //     }

    //     res+=1;

    //     if br
    //     {
    //         break;
    //     }
    //     // println!("")
    // }

    let mut tab = [(277348i64,92449i64),(310788i64, 155393i64),(312756i64, 104251i64),(232110i64, 116053i64),(287187i64, 143591i64), (253749i64, 84581i64)];
    let mut res :i64;

    loop 
    {
        res = min(tab[0].0, min(tab[1].0, min(tab[2].0, min(tab[3].0, min(tab[4].0, tab[5].0)))));
        if res == tab[0].0 && res == tab[1].0 && res == tab[2].0 && res == tab[3].0 && res == tab[4].0 && res == tab[5].0
        {
            break;
        }

        for i in 0..6
        {
            if tab[i].0 == res
            {
                tab[i].0 += tab[i].1;
            }
        }
    }

    println!("{res}");
}
