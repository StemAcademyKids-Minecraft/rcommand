use crate::commands::COMMANDS;

struct ScoreBoard {}

impl ScoreBoard {
    fn get(name: &str,score:&str) -> String {
        let api = format!("&score:{}:{}:@:",name,score,);

        //unsafe { COMMANDS.push(api) };

        api
    }
}
