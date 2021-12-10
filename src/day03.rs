pub fn part1(input: String) {
    let queries: Vec<&str> = input.split("\n").collect();
    let n = queries.len();
    let mut ones: Vec<i32> = Vec::new();
    let m = queries[0].len();
    ones.resize(m, 0);
    for i in queries.iter() {
        for (j, c) in i.chars().enumerate() {
            if c == '1' {
                ones[j] += 1;
            }
        }
    }
    let mut gamma = 0;
    let mut eps = 0;
    let mut val = 1;
    for i in (0..m).rev() {
        if 2 * ones[i] > n as i32 {
            gamma += val;
        } else {
            eps += val;
        }
        val *= 2;
    }
    println!("{}", gamma * eps);
}

pub fn part2(input: String) {
    let queries: Vec<&str> = input.split("\n").collect();
    let m = queries[0].len();
    let mut curr = queries.clone();
    let mut s1 = 0;
    let mut s2 = 0;
    let mut val: i32;
    let mut val2 = 1;
    for _i in 0..m - 1 {
        val2 *= 2;
    }
    val = val2;
    for i in 0..m {
        let mut cnt = 0;
        let mut temp: Vec<&str> = vec![];
        let mut temp2: Vec<&str> = vec![];
        for j in curr.iter() {
            if j.chars().nth(i).unwrap() == '1' {
                cnt += 1;
                temp.push(j);
            } else {
                temp2.push(j);
            }
        }
        if 2 * cnt >= curr.len() {
            s1 += val;
            curr = temp;
        } else {
            curr = temp2;
        }
        val /= 2;
    }
    curr = queries.clone();
    val = val2;
    for i in 0..m {
        if curr.len() == 1 {
            if curr[0].to_string().chars().nth(i).unwrap() == '1' {
                s2 += val;
            }
            val /= 2;
            continue;
        }

        let mut cnt = 0;
        let mut temp: Vec<&str> = vec![];
        let mut temp2: Vec<&str> = vec![];
        for j in curr.iter() {
            if j.chars().nth(i).unwrap() == '1' {
                cnt += 1;
                temp.push(j);
            } else {
                temp2.push(j);
            }
        }
        if 2 * cnt < curr.len() {
            s2 += val;
            curr = temp;
        } else {
            curr = temp2;
        }
        val /= 2;
    }
    println!("{}", s1 * s2);
}
