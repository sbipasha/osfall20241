fn farenheight_to_celcius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}
fn celcius_to_farenheight(c: f64) -> f64 {
    (c * 9.0 / 5.0) + 32.0
}
fn main() {
    const FREEZING_TEMP: f64 = 32.0;
    let mut temp_in_faren: f64 = 40.0;
    let celcius_convt: f64 = farenheight_to_celcius(temp_in_faren);
    println!("Converting farenheight to Celcius: {}", celcius_convt);
    println!("Next 5 Temperatures: ");
    for _ in 1..=5 {
        temp_in_faren += 1.0;
        println!("{}", (farenheight_to_celcius(temp_in_faren)));
    }
}
