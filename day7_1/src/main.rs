use std::fs;

fn char_idx(c : char) -> usize
{
    match c
    {
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'T' => 10,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => todo!()
    }
}

fn get_type(s: String) -> i32
{
    let mut tmp = vec![0;15];
    s.chars().for_each(|c| tmp[char_idx(c)]+=1);

    if tmp.contains(&5)
    {
       return  6;
    }

    if tmp.contains(&4)
    {
        return 5;
    }

    if tmp.contains(&3) && tmp.contains(&2)
    {
        return 4;
    }

    if tmp.contains(&3)
    {
        return 3;
    }

    tmp.sort();

    if tmp[tmp.len()-1] == 2 && tmp[tmp.len()-2] == 2
    {
        return 2;
    }

    if tmp.contains(&2)
    {
        return 1
    }

    return 0;
}

fn str_to_tuple(s : String) -> (i32, i32)
{
    if s.is_empty()
    {
        return (0,0);
    }
    
    let base = 16;
    let st = s.split(' ').collect::<Vec<_>>();

    let mut res1 = get_type(st[0].to_string());

    for c in st[0].chars()
    {
        res1 = res1*base + char_idx(c) as i32;
    }


    let res3 = st[1].to_string().parse::<i32>().unwrap();

    (res1, res3)

}

fn main() {
    let  file_path = "/home/jakub/Desktop/Rust/AoC2023/day7_1/dane.txt";
    let contents = fs::read_to_string(file_path)
            .expect("Should have been able to read the file");


    let mut board : Vec<(i32, i32)> = contents.split('\n').map(|s| str_to_tuple(s.to_string())).collect();

    let _ = board.remove(board.len()-1);

    board.sort_by(|a,b| a.0.cmp(&b.0));

    let res : i32 = board.iter().enumerate().map(|(a, (_b,c))| c*(a as i32+1)  ).sum();

    println!("{res}");
}
