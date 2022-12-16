use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;

use super::super::schema::team_seaking::dsl::*;
use super::super::models::*;
use diesel::prelude::*;
use super::pq;

pub fn get_available_users(_command_interaction: &mut ApplicationCommandInteraction) -> String {
    let connection = &mut pq::connect::establish_connection();

    let res = team_seaking.load::<TeamSeaking>(connection).expect("Error loading");
    let mut return_val = String::new();

    for item in res {
        return_val += &format!("**Name**: {} ({})\n**Description**: {}\n\n", item.name, item.discordname, item.description).to_owned();
    }

	format!("{}\n", return_val)
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("list_available_users")
        .description("Returns the complete list of available users")
}
