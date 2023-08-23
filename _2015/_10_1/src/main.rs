fn main() {
    println!("Hello, world!");
    let input = "1113222113";
    //let input = "1";
    let mut end = input.to_string();

    //for _ in 0..40 {
    for _ in 0..50 {
        end = look_n_say(&end);
    }
    println!("{}", end.len());
}

fn look_n_say(code: &str) -> String {
    let mut end_num = String::new();
    let mut num_counter: u32 = 0;

    let mut num_buffer: char = code.chars().next().unwrap();
    for num in code.chars() {
        if num == num_buffer {
            num_counter += 1;
        } else {
            end_num += &format!("{}{}", num_counter, num_buffer);
            num_buffer = num;
            num_counter = 1;
        }
    }
    end_num += &format!("{}{}", num_counter, num_buffer);
    end_num 
}
