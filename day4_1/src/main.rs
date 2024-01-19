use std::fs;


fn pow_or_zero(n :usize) -> i32
{
    if n == 0
    {
        return 0;
    }

    2i32.pow(n as u32 - 1)
}

fn main() {
    let  file_path = "/home/jakub/Desktop/Rust/AoC2023/day4_1/dane.txt";
    let contents = fs::read_to_string(file_path)
            .expect("Should have been able to read the file");


    let mut board : Vec<String> = contents.split('\n').map(|s| s.to_string()).collect();
    let _ = board.remove(board.len()-1);

    let pairs2 = board.iter()
    .map(|s| s.split(|c| c == ':' || c == '|')
                    .map(|r| r.to_string())
                    .collect::<Vec<_>>() ).collect::<Vec<Vec<_>>>();


    // for k in pairs2
    // {
    //     let r = k.len();
    //     println!("{r}");

    //     let s1 = k[0].clone();
    //     let s2 = k[1].clone();
    //     let s3 = k[2].clone();
    
    // println!("{s1}");
    // println!("{s2}");
    // println!("{s3}");


    // }
    
    let pairs = board.iter()
        .map(|s| s.split(|c| c == ':' || c == '|')
                        .map(|r| r.to_string())
                        .collect::<Vec<_>>() )
        .map(|s| (s[1].clone()
                        .split(' ')
                        .map(|st| st.to_string().parse::<i32>().unwrap_or(-1))
                        .filter(|&c| c != -1 )
                        .collect::<Vec<i32>>(),
                    s[2].clone()
                        .split(' ')
                        .map(|st| st.to_string().parse::<i32>().unwrap_or(-1))
                        .filter(|&c| c != -1 )
                        .collect::<Vec<i32>>()))
        .collect::<Vec<(Vec<i32>,Vec<i32>)>>();


    let res :i32= pairs.iter().map(|(a,b)|  pow_or_zero(b.iter().filter(|&c| a.contains(c)).count())).sum();

    println!("{res}");

}
