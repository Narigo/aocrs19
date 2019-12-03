mod aoc01;
mod aoc02;
mod aoc03;

fn main() {
    println!("Hello, world!");
    println!("aoc01_01 = {}", aoc01::sum_fuel_01());
    println!("aoc01_02 = {}", aoc01::sum_fuel_02());
    println!("aoc02_01 = {}", aoc02::program_1202_01());
    println!("aoc02_02 = {}", aoc02::program_1202_02());
    println!("aoc03_01 = {}", aoc03::compute_crossing());
}
