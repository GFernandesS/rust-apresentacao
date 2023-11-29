const SECONDS_IN_MINUTE: u32 = 10;

fn main() {
    println!("Hello, world!");
    let total: i32 = 30;
    println!("Trabalhou {} horas", total);

    {
        let total = 40;
        println!("Trabalhou interno {} horas", total);
    }

    println!("Trabalhou final: {} horas", total);
}
