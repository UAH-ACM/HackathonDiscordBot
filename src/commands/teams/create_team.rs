use super::super::super::models::*;
use super::super::super::schema::teams::dsl::*;
use super::super::pq;
use diesel::RunQueryDsl;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::prelude::interaction::application_command::CommandDataOptionValue;

pub fn create_team(command_interaction: &mut ApplicationCommandInteraction) -> String {
    let user_parent = command_interaction.member.as_mut().unwrap();
    let connection = &mut pq::connect::establish_connection();
    let options = &command_interaction.data.options;
    let discord_name = format!(
        "{}#{}",
        user_parent.user.name, user_parent.user.discriminator
    );

    let id_unwrapped = teams.load::<Teams>(connection);
    let id_loc: i64;

    match id_unwrapped {
        Ok(good) => id_loc = good.len() as i64 + 1,
		Err(bad) => return format!("{}", bad),
    }

    let team_option = options.get(0).expect("Expected user option").resolved.as_ref().expect("Expected user object");
    let description_option = options.get(1).expect("Expected user option").resolved.as_ref().expect("Expected user object");

    let name_loc: &str;
    let description_loc: &str;

    if let CommandDataOptionValue::String(value) = team_option {
        name_loc = value;
    } else {
        return format!("{:?} is not a valid name", team_option);
    }

    if let CommandDataOptionValue::String(value) = description_option {
        description_loc = value;
    } else {
        return format!("{:?} is not a valid description", description_option);
    }

    let res = pq::interface::insert_team(
        connection,
        &id_loc,
        name_loc,
        description_loc,
        &discord_name[..],
    );

    match res {
        Ok(_) => format!("Team created"),
		Err(e) => format!("{}", e),
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("create_team")
        .description("Register as \"Team seeking\". Provide a description of yourself so others can read up on you!")
        .create_option(|option| {
            option
                    .name("team")
                    .description("The team name")
                    .kind(CommandOptionType::String)
                    .required(true)
        })
        .create_option(|option| {
            option
                    .name("description")
                    .description("The name of the leader")
                    .kind(CommandOptionType::String)
                    .required(true)
        })
}
