use std::fs;


fn parse_to_num(s : String) -> (i64, i64, i64)
{
    println!("{s}");
    let res = s.split(' ').map(|f| f.to_string().parse::<i64>().unwrap()).collect::<Vec<_>>();

    (res[0], res[1], res[2])
}

fn main() {
    let  file_path = "/home/jakub/Desktop/Rust/AoC2023/day5_1/dane.txt";
    let contents = fs::read_to_string(file_path)
            .expect("Should have been able to read the file");


    let mut board : Vec<String> = contents.split('\n').map(|s| s.to_string()).collect();
    let _ = board.remove(board.len()-1);
    let  mut seeds = board[0].split(' ').map(|s| s.to_string().parse::<i64>().unwrap_or(-1)).filter(|&p| p != -1).collect::<Vec<_>>();
    let _ = board.remove(0);

    for _ in  0..7
    {
        let _ = board.remove(0);
        let _ = board.remove(0);

        let mut array : Vec<(i64, i64, i64)> = vec![];

        while board[0] !=  ""
        {
            if board[0] ==  "" 
            {
                break;
            }
            let r = parse_to_num(board[0].clone());
            array.push(r);
            let _ = board.remove(0);
        }

        seeds = seeds.iter_mut()
        .map(|s| s.clone() + array.iter()
                                .find_map(|(a,b,c)| if b <= &s && &(b + c) >= s {return Some(a-b)} else {None} )
                                .unwrap_or(0)).collect::<Vec<_>>();

    }

    seeds.sort();
    let wynik = seeds[0];

    println!("{wynik}")

}
