pub fn part1(input: String) {
    let queries: Vec<&str> = input.split("\n").collect();
    let mut x = 0;
    let mut y = 0;
    for i in queries.iter() {
        let temp: Vec<&str> = i.split(" ").collect();
        let op = temp[0];
        let dis: i32 = temp[1].to_string().parse().unwrap();
        if op == "forward" {
            x += dis;
        } else if op == "up" {
            y -= dis;
        } else {
            y += dis;
        }
    }
    println!("{}", x * y);
}

pub fn part2(input: String) {
    let queries: Vec<&str> = input.split("\n").collect();
    let mut x = 0;
    let mut y = 0;
    let mut aim = 0;
    for i in queries.iter() {
        let temp: Vec<&str> = i.split(" ").collect();
        let op = temp[0];
        let dis: i32 = temp[1].to_string().parse().unwrap();
        if op == "forward" {
            x += dis;
            y += dis * aim;
        } else if op == "up" {
            aim -= dis;
        } else {
            aim += dis;
        }
    }
    println!("{}", x * y);
}
