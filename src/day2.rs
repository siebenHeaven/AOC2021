pub(crate) fn solve(input: &[String]) {
    let (x, y) = input
        .iter()
        .map(|s| {
            let (dir, val_str) = s.split_once(' ').unwrap();
            let val: i32 = val_str.parse().unwrap();
            match dir {
                "forward" => (val, 0),
                "up" => (0, -val),
                "down" => (0, val),
                _ => unimplemented!(),
            }
        })
        .fold((0, 0), |(x, y), (dx, dy)| (x + dx, y + dy));

    println!("Part1: {}", x * y);

    let (x, y, _) = input
        .iter()
        .map(|s| {
            let (dir, val_str) = s.split_once(' ').unwrap();
            let val: i32 = val_str.parse().unwrap();
            match dir {
                "forward" => (val, 0),
                "up" => (0, -val),
                "down" => (0, val),
                _ => unimplemented!(),
            }
        })
        .fold((0, 0, 0), |(x, y, aim), (dx, daim)| {
            (x + dx, y + aim * dx, aim + daim)
        });

    println!("Part2: {}", x * y);
}
