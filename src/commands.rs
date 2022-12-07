use serenity::model::application::interaction::message_component::MessageComponentInteraction;

pub async fn help() -> String {
    let help_message = "I can help you explore and join groups.
    Following is the list of available commands.
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

pub async fn whoami(msg: MessageComponentInteraction) -> String {
    let help_message = format!("Hi, you are {}#{}!", msg.user.name, msg.user.discriminator);
    String::from(help_message)
}