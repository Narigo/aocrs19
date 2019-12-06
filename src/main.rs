mod aoc01;
mod aoc02;
mod aoc03;
mod aoc04;
mod aoc05;
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
}
