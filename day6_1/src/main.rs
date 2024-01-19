

fn main() 
{
    let data = vec![(38947970i64,241154910741091i64)];

    let mut res = 1;

    for (a,b) in data
    {
        let mut temp = 0;

        for i in 1..a
        {
            if i*(a-i) > b
            {
                temp += 1;
            }
        }

        res *= temp;
    }

    println!("{res}");
}
