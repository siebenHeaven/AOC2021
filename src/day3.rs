pub(crate) fn solve(input: &[String]) {
    let n = input.len();
    let width = input[0].len();

    let get_counts = |input: &[String], f: Option<&[bool]>| -> Vec<usize> {
        input
            .iter()
            .enumerate()
            .filter(|(i, _)| f.map_or(true, |f| f[*i]))
            .fold(vec![0usize; width], |mut acc, (_, line)| {
                line.chars()
                    .rev()
                    .enumerate()
                    .for_each(|(i, c)| acc[i] += c.to_digit(10).unwrap() as usize);
                acc
            })
    };

    {
        let gamma: u64 = get_counts(input, None)
            .iter()
            .enumerate()
            .fold(0, |acc, (i, x)| acc | if 2 * x >= n { 1 << i } else { 0 });

        let epsilon = !gamma & (u64::MAX >> gamma.leading_zeros());

        println!("Part 1: {}", gamma * epsilon);
    }

    {
        let mut o2_filter = vec![true; input.len()];
        let mut co2_filter = vec![true; input.len()];

        let populate_filter = |bit: usize, filter: &mut [bool], flag: bool| {
            let valid_count = filter.iter().cloned().filter(|&x| x).count();
            if valid_count == 1 {
                return;
            }
            let counts = get_counts(input, Some(filter));
            filter.iter_mut().enumerate().for_each(|(j, b)| {
                let (majority, minority) = if 2 * counts[width - bit - 1] >= valid_count {
                    ('1', '0')
                } else {
                    ('0', '1')
                };
                *b = *b
                    && input[j].chars().nth(bit).unwrap() == if flag { majority } else { minority };
            });
        };

        for i in 0..width {
            populate_filter(i, &mut o2_filter, true);
            populate_filter(i, &mut co2_filter, false);
        }

        let calculate_rating = |input: &[String], filter: &[bool]| {
            u64::from_str_radix(
                input
                    .iter()
                    .enumerate()
                    .find_map(|(i, s)| if filter[i] { Some(s) } else { None })
                    .unwrap(),
                2,
            )
            .unwrap()
        };

        println!(
            "Part 2: {}",
            calculate_rating(input, &o2_filter) * calculate_rating(input, &co2_filter)
        );
    }
}
