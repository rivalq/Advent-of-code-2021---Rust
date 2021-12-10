fn check_victory(board: &Vec<Vec<i32>>) -> i32 {
    for i in 0..5 {
        let mut cnt1 = 0;
        let mut cnt2 = 0;
        for j in 0..5 {
            if board[i][j] == 1 {
                cnt1 += 1;
            }
            if board[j][i] == 1 {
                cnt2 += 1;
            }
        }
        if cnt1 == 5 || cnt2 == 5 {
            return 1;
        }
    }
    return 0;
}

fn solve_board(board: &Vec<Vec<&str>>, order: &Vec<&str>) -> (i32, i32) {
    let mut mark: Vec<Vec<i32>> = Vec::new();
    mark.resize(5, Vec::new());
    for i in 0..5 {
        mark[i].resize(5, 0);
    }
    let mut sum = 0;
    for i in 0..5 {
        for j in 0..5 {
            sum += board[i][j].to_string().parse::<i32>().unwrap();
        }
    }
    for i in 0..order.len() {
        for x in 0..5 {
            for y in 0..5 {
                if board[x][y] == order[i] {
                    mark[x][y] = 1;
                    sum -= order[i].to_string().parse::<i32>().unwrap();
                }
            }
        }
        if check_victory(&mark) == 1 {
            return (i as i32, sum * order[i].to_string().parse::<i32>().unwrap());
        }
    }
    return (-1, 0);
}

pub fn part1(input: String) {
    let queries: Vec<&str> = input.split("\n").collect();
    let order: Vec<&str> = queries[0].split(",").collect();
    let n = queries.len();
    let mut i = 2;

    let mut best = order.len() as i32 + 1;
    let mut score = 0;

    while i < n {
        let mut vec: Vec<Vec<&str>> = Vec::new();
        let mut itr = 5;
        while itr > 0 {
            vec.push(queries[i].split(" ").collect());
            itr -= 1;
            i += 1;
        }
        let mut board: Vec<Vec<&str>> = Vec::new();
        for i in vec.iter() {
            let mut temp: Vec<&str> = Vec::new();
            for j in i.iter() {
                if j.to_string() != " " && j.to_string() != "" {
                    temp.push(j);
                }
            }
            board.push(temp)
        }
        let res = solve_board(&board, &order);
        if res.0 != -1 && res.0 < best {
            best = res.0;
            score = res.1;
        }
        i += 1;
    }
    println!("{}", score);
}

pub fn part2(input: String) {
    let queries: Vec<&str> = input.split("\n").collect();
    let order: Vec<&str> = queries[0].split(",").collect();
    let n = queries.len();
    let mut i = 2;

    let mut best = -1;
    let mut score = 0;

    while i < n {
        let mut vec: Vec<Vec<&str>> = Vec::new();
        let mut itr = 5;
        while itr > 0 {
            vec.push(queries[i].split(" ").collect());
            itr -= 1;
            i += 1;
        }
        let mut board: Vec<Vec<&str>> = Vec::new();
        for i in vec.iter() {
            let mut temp: Vec<&str> = Vec::new();
            for j in i.iter() {
                if j.to_string() != " " && j.to_string() != "" {
                    temp.push(j);
                }
            }
            board.push(temp)
        }
        let res = solve_board(&board, &order);
        if res.0 > best {
            best = res.0;
            score = res.1;
        }
        i += 1;
    }
    println!("{}", score);
}
