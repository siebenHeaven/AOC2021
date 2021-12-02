pub(crate) fn solve(input: Vec<String>) {
    let (x, y) = input
        .iter()
        .map(|s| {
            let mut split = s.split(' ');
            let dir = split.next().unwrap();
            let val: i32 = split.next().unwrap().parse().unwrap();
            match dir {
                "forward" => (val, 0),
                "up" => (0, -val),
                "down" => (0, val),
                _ => unimplemented!(),
            }
        })
        .fold((0, 0), |(x, y), (xdiff, ydiff)| (x + xdiff, y + ydiff));

    println!("Part1: {}", x * y);

    let (x, y, _) = input
        .iter()
        .map(|s| {
            let mut split = s.split(' ');
            let dir = split.next().unwrap();
            let val: i32 = split.next().unwrap().parse().unwrap();
            match dir {
                "forward" => (val, 0),
                "up" => (0, -val),
                "down" => (0, val),
                _ => unimplemented!(),
            }
        })
        .fold((0, 0, 0), |(x, y, aim), (xdiff, aimdiff)| {
            (x + xdiff, y + aim * xdiff, aim + aimdiff)
        });

    println!("Part2: {}", x * y);
}
