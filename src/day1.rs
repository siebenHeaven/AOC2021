pub(crate) fn solve(input: &[String]) {
    let nums: Vec<u32> = input.iter().map(|x| x.parse().unwrap()).collect();

    let result = nums
        .iter()
        .zip(nums.iter().skip(1))
        .filter(|(a, b)| a < b)
        .count();

    println!("Part 1: {}", result);

    let result = nums
        .windows(3)
        .zip(nums[1..].windows(3))
        .filter(|(a, b)| a.iter().sum::<u32>() < b.iter().sum())
        .count();

    println!("Part 2: {}", result);
}
