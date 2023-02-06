use serenity::builder::CreateApplicationCommand;

pub fn help() -> String {
	let part1 = "**The following is a list of available commands.**";
    let part2 = "- `/help` - Show this message.";
    let part3 = "- `/whoami` - See some info about you.";
    let part4 = "- `/checkin <description>` - Register as \"Team seeking\". Provide a description of yourself so others can read up on you!";
    let part5 = "- `/list_available_teams` - List all groups.";
    let part6 = "- `/list_available_users` - List all available registered users.";
    let part7 = "- `/create_team <teeam name> <description>` - Create a group and join it.";
    let part8 = "- `/join <team id>` - Join an existing group.";
    let part9 = "- `/describe <team id>` - Show group summary.";

    format!("{}\n\t{}\n\t{}\n\t{}\n\t{}\n\t{}\n\t{}\n\t{}\n\t{}\n", part1, part2, part3, part4, part5, part6, part7, part8, part9)
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("help").description("Show the help message.")
}
