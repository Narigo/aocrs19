mod aoc01;
mod aoc02;
mod aoc03;
mod aoc04;
mod aoc05;
mod aoc06;
mod aoc07;
mod aoc08;
mod aoc09;
mod intcode_compute;

fn main() {
    println!("Hello, world!");
    println!("aoc01_01 = {}", aoc01::sum_fuel_01());
    println!("aoc01_02 = {}", aoc01::sum_fuel_02());
    println!("aoc02_01 = {}", aoc02::program_1202_01());
    println!("aoc02_02 = {}", aoc02::program_1202_02());
    // println!("aoc03_01 = {}", aoc03::compute_crossing());
    // println!("aoc03_02 = {}", aoc03::find_lowest_amount_of_steps());
    println!("aoc04_01 = {}", aoc04::find_amount_of_possible_passwords());
    println!("aoc04_02 = {}", aoc04::find_possible_passwords());
    println!("aoc05_01 = {}", aoc05::program_1205_01());
    println!("aoc05_02 = {}", aoc05::program_1205_02());
    println!("aoc06_01 = {}", aoc06::orbit_checksum());
    println!("aoc06_02 = {}", aoc06::orbit_transfers());
    println!("aoc07_01 = {}", aoc07::program_1207_01());
    println!("aoc07_02 = {}", aoc07::program_1207_02());
    println!("aoc08_01 = {}", aoc08::space_image_format());
    println!("aoc08_02 =\n{}", aoc08::draw_image());
    println!("aoc09_01 = {:?}", aoc09::boost_01());
    println!("aoc09_02 = {:?}", aoc09::boost_02());
}
