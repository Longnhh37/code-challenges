fn greet(input : &str) -> String {
  if input == "Johnny" {
    return "Hello, my love!".to_string();
  };
  format!("Hello, {}!", input)
}