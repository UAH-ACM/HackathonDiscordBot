use serenity::builder::CreateApplicationCommand;
use super::super::pq;

use serenity::model::prelude::{
    command::CommandOptionType,
	interaction::application_command::{ ApplicationCommandInteraction, CommandDataOptionValue }
};

pub fn checkin(command_interaction: &mut ApplicationCommandInteraction) -> String {
    let user_parent = command_interaction.member.as_mut().unwrap();

    let options = &command_interaction.data.options;

    let name_option = options
        .get(0)
        .expect("Expected user option")
        .resolved
        .as_ref()
        .expect("Expected user object");

    let description_option = options
        .get(1)
        .expect("Expected user option")
        .resolved
        .as_ref()
        .expect("Expected user object");

    let connection = &mut pq::connect::establish_connection();

    let id = user_parent.user.id.0 as i64;
    let discord_name = format!(
        "{}#{}",
        user_parent.user.name, user_parent.user.discriminator
    );

    let name: &str;
    let description: &str;

    if let CommandDataOptionValue::String(value) = name_option {
        name = value;
    } else {
        return format!("{:?} is not a valid name", name_option);
    }

    if let CommandDataOptionValue::String(value) = description_option {
        description = value;
    } else {
        return format!("{:?} is not a valid name", description_option);
    }

    match pq::interface::insert_user(connection, &id, name, &discord_name[..], description) {
        Err(bad) => return format!("{}", bad),
		Ok(_) => return format!("User inserted"),
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("checkin")
        .description("Register as \"Team seeking\". Provide a description of yourself so others can read up on you!")
        .create_option(|option| {
            option
                    .name("name")
                    .description("Your real name, for example \"John Doe\"")
                    .kind(CommandOptionType::String)
                    .required(true)
        })
        .create_option(|option| {
            option
                    .name("description")
                    .description("A description of yourself for teams")
                    .kind(CommandOptionType::String)
                    .required(true)
        })
}
