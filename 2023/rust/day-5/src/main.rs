mod input;

fn main() {
    let input = match input::read_input_from_file("day-5-input.txt") {
        Ok(input) => input,
        Err(err) => {
            println!("{}", err);
            return
        }
    };

    println!("{:?}", input);
}
