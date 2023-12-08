pub mod input {

    pub async fn send_output(content: &str) {
        println!("CLARITY: {}\n", content)
    }

    pub async fn get_input() -> String {
        let mut input = String::new();
        match std::io::stdin().read_line(&mut input) {
            Ok(n) => {
                println!("{} bytes read", n);
                println!("{}", &input);
                return input;
            }
            Err(error) => println!("error: {error}"),
        }
    }
}
