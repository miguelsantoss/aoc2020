mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;

use std::time::Instant;

fn measure(day: &str, f: &dyn Fn()) {
    let now = Instant::now();
    f();
    let elapsed = now.elapsed();
    println!("day {}: {:.2?}", day, elapsed);
    println!("------------------------------");
}

fn main() {
    let now = Instant::now();

    // measure("01", &day_01::day_01);
    // measure("02", &day_02::day_02);
    // measure("03", &day_03::day_03);
    // measure("04", &day_04::day_04);
    // measure("05", &day_05::day_05);
    measure("06", &day_06::day_06);

    let elapsed = now.elapsed();
    println!("total: {:.2?}", elapsed);
}
