use super::Game;

pub fn process_command(input: &str) -> String {
    let words: Vec<&str> = input.split_whitespace().collect();
    if words.len() == 0 {
        return "What did you say?".to_string();
    }

    let first_word: &str = &words[0].to_lowercase();
    let mut second_word: String = String::new();
    if words.len() > 1 {
        second_word = words[1].to_lowercase();
    }

    match first_word {
        // Location commands
        "go" => return go(second_word),
        "n" | "north" => return go("North".to_string()),
        "s" | "south" => return go("South".to_string()),
        "e" | "east" => return go("east".to_string()),
        "w" | "west" => return go("West".to_string()),

        "help" => return help(),
        _ => return "unrecognized command".to_string(),
    }
}

fn go(second_word: String) -> String {
    if second_word == "" {
        return "Where?".to_string();
    }

    "You go ".to_string() + &second_word
}

fn help() -> String {
    let game = Game::new();
    let mut response = String::new();

    for i in 0..game.commands.len() {
        response.push_str(game.commands[i]);
        if game.command_args[i] != "" {
            response.push_str(" [");
            response.push_str(game.command_args[i]);
            response.push_str("], ");
            continue;
        }
        response.push_str(game.command_args[i]);
        response.push_str(", ");
    }

    return response;
}
