fn part_1(presents: usize) -> usize {
    let mut houses = vec![0; presents];

    for elf in 0..presents {
        (elf..presents).step_by(elf + 1).for_each(|house_number| {
            houses[house_number] += (elf + 1) * 10;
        });
    }

    houses.iter().position(|&p| p >= presents).unwrap() + 1
}

fn part_2(presents: usize) -> usize {
    let mut houses = vec![0; presents];

    for elf in 0..presents {
        (elf..presents)
            .step_by(elf + 1)
            .take(50)
            .for_each(|house_number| {
                houses[house_number] += (elf + 1) * 11;
            });
    }

    houses.iter().position(|&p| p >= presents).unwrap() + 1
}

fn main() {
    let presents = 33_100_000;
    let r1 = part_1(presents);
    let r2 = part_2(presents);

    println!("Result1: {}", r1);
    println!("Result2: {}", r2);
}
