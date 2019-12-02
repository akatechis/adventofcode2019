
fn fuel_for_mass(mass: &usize) -> usize {
    let div = (mass / 3) as f64;
    let fuel = div.floor() - 2.0;
    if fuel < 0.0 {
      0
    } else {
      fuel as usize
    }
}

fn fuel_for_mass_and_fuel(mass: &usize) -> usize {
    let mut total = 0;
    let mut fuel = fuel_for_mass(mass);
    loop {
        total += fuel;
        if fuel > 2 {
            fuel = fuel_for_mass(&fuel);
        } else {
            break;
        }
    }
    total
}

fn main() {
    let data = create_data();
    part_1(&data);
    part_2(&data);
}

fn part_1(data: &Vec<usize>) {
    let mut total = 0;
    for mass in data {
        total += fuel_for_mass(mass);
    }
    println!("(pt1) Fuel needed = {}", total);
}

fn part_2(data: &Vec<usize>) {
    let mut total = 0;
    for mass in data {
        total += fuel_for_mass_and_fuel(mass);
    }
    println!("(pt2) Fuel needed = {}", total);
}

fn create_data() -> Vec<usize> {
    vec![
        139616,
        148675,
        139706,
        89248,
        63979,
        140157,
        80815,
        74613,
        147394,
        117757,
        52711,
        137502,
        83219,
        71821,
        104791,
        104448,
        89677,
        74804,
        128554,
        125346,
        138662,
        136114,
        110521,
        143060,
        117221,
        61827,
        142517,
        119651,
        110249,
        79507,
        126873,
        144314,
        106224,
        68369,
        64974,
        75958,
        54365,
        62977,
        144205,
        141953,
        96671,
        138559,
        84377,
        110649,
        74845,
        103697,
        83778,
        129677,
        65904,
        113836,
        126255,
        114839,
        135004,
        120632,
        103543,
        139442,
        146017,
        107982,
        146149,
        104431,
        133978,
        57697,
        86463,
        122606,
        95701,
        84680,
        139779,
        71970,
        80514,
        83229,
        143172,
        123825,
        79849,
        109438,
        144655,
        130229,
        97853,
        67769,
        62040,
        125972,
        93546,
        77516,
        103971,
        114918,
        84445,
        123466,
        56622,
        135859,
        90966,
        77417,
        125790,
        144466,
        136980,
        147914,
        92955,
        75165,
        144271,
        135509,
        98379,
        118530
    ]
}

#[test]
fn test_fuel_for_mass_and_fuel() {
  assert_eq!(fuel_for_mass_and_fuel(&14), 2);
  assert_eq!(fuel_for_mass_and_fuel(&1969), 966);
  assert_eq!(fuel_for_mass_and_fuel(&100756), 50346);
}
