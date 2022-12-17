use serenity::builder::CreateApplicationCommand;

pub fn help() -> String {
    let help_message = "I can help you explore and join groups.
    **The following is a list of available commands.**
    `!help`: Show this message.
    `!whoami`: See some info about you.
    `!checkin <description>`: Register as \"Team seeking\". Provide a description of yourself so others can read up on you!
    `!ls`: List all groups.
    `!ls_free`: List all available registered users.
    `!create <group_name> <description>`: Create a group and join it. Your `group_name` may not contain spaces.
    `!join <group_name>`: Join an existing group.
    `!describe <group_name>`: Show group summary.";

    String::from(help_message)
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("help").description("Show the help message.")
}
