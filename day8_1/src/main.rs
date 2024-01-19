use std::{fs, collections::HashMap};


fn create_map(v : Vec<String>) -> HashMap<String, (String,String)>
{
    let mut res = HashMap::new();

    for s in v
    {
        res.insert(s[..=2].to_string(), (s[7..=9].to_string(), s[12..=14].to_string()));
    }

    res
}



fn main() {
    let  file_path = "/home/jakub/Desktop/Rust/AoC2023/day8_1/dane.txt";
    let contents = fs::read_to_string(file_path)
            .expect("Should have been able to read the file");


    let mut board : Vec<String> = contents.split('\n').map(|s| s.to_string()).collect();

    let _ = board.remove(board.len()-1);
    let polecenia = board[0].clone().chars().map(|c| match c {'L' => 0, 'R' => 1, _ => -1}).collect::<Vec<_>>();
    let _ = board.remove(0);
    let _ = board.remove(0);

    let map = create_map(board);


    let mut res = 0;
    let mut pos = "AAA".to_string();
    for &i in polecenia.iter().cycle()
    {
        let new_pos = &map[&pos]; 
        if i == 0
        {
            pos = new_pos.0.clone();
        }
        else 
        {
            pos = new_pos.1.clone();   
        }
        res += 1;
        if pos == "ZZZ".to_string()
        {
            break;
        }
    }

    println!("{res}");
}
