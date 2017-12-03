pub fn aoc3() {
    use std::collections::HashMap;
    use std::panic;
    println!("aoc3");

    let mut mem = HashMap::new();

    let mut cur_x: i32 = 0;
    let mut cur_y: i32 = 0;

    let mut dir_x: i32 = 1;
    let mut dir_y: i32 = 0;

    let mut iter_x: i32 = 1;
    let mut iter_y: i32 = 1;

    let mut dist_x: i32 = 0;
    let mut dist_y: i32 = 0;

    let distance: i32 = 289326;

    fn next_seq(x: i32, y: i32) -> (i32, i32) {
        match (x,y) {
            (1, 0) => (0,1),
            (0, 1) => (-1,0),
            (-1, 0) => (0,-1),
            (0, -1) => (1,0),
            (_, _) => panic!("invalid input")
        }
    }

    fn sum_around(x: i32, y: i32, mem: &HashMap<(i32, i32), i32>) -> (i32) {
        let vec: Vec<(i32, i32)> = vec![(1,0), (0,1), (-1,0), (0,-1), (1,1), (1,-1), (-1,1), (-1,-1)];
        panic::catch_unwind(|| {
            vec
                .iter()
                .map(|&(dx, dy)| {
                    mem.get(&(x + dx, y + dy)).unwrap_or(&0)
                })
                .sum()
        }).unwrap_or(0)
    }

    let mut searching = true;
    for i in 1.. distance {
        if i == 1 {
            mem.insert((cur_x, cur_y), i);
        } else {
            let s = sum_around(cur_x, cur_y, &mem);
            if searching && s > distance{
                searching = false;
                println!("found: {}", s);
            }
            mem.insert((cur_x, cur_y), s);
        }
        // println!("cur: ({}, {}) dir: ({}, {}) iter: ({}, {}), dist: ({}, {}), distance: {}, mem_cur: {:?}, mem_val: {}", cur_x, cur_y, dir_x, dir_y, iter_x, iter_y, dist_x, dist_y, i, (cur_x, cur_y), mem[&(cur_x, cur_y)]);
        cur_x += dir_x;
        cur_y += dir_y;

        dist_x += dir_x.abs();
        dist_y += dir_y.abs();

        if dist_x == iter_x || dist_y == iter_y {
            let (x, y) = next_seq(dir_x, dir_y);
            dir_x = x;
            dir_y = y;
        }
        if dist_x == iter_x {
            iter_x += 1;
            dist_x = 0;
        }
        if dist_y == iter_y {
            iter_y += 1;
            dist_y = 0;
        }
    }
    println!("cur: ({}, {}) dir: ({}, {}) iter: ({}, {}), dist: ({}, {}), distance: {}", cur_x, cur_y, dir_x, dir_y, iter_x, iter_y, dist_x, dist_y, distance);
    println!("manhattan_distance: {}", cur_x.abs() + cur_y.abs());
}
