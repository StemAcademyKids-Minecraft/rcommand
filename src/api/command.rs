use serde::{Serialize, Deserialize};



#[derive(Serialize, Deserialize)]

struct Command {
    command:Option<String>,
    delay:Option<usize>,
    api:Option<Vec<rAPI>>,

}



#[derive(Serialize, Deserialize)]
enum rAPI {
    scoreboard(String,String),

}



#[test]
fn test() {
    let api = vec![rAPI::scoreboard("t".to_owned(), "a".to_owned())];
    let book = Command {command:None,delay:None,api:Some(api) };
    let json = serde_json::to_string(&book).unwrap();
    println!("{}", json);
}