use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;

use super::super::super::schema::team_seaking::dsl::*;
use super::super::super::models::*;
use diesel::prelude::*;
use super::super::pq;

pub fn get_table(_command_interaction: &mut ApplicationCommandInteraction) -> String {
    let connection = &mut pq::connect::establish_connection();

    let res = team_seaking.load::<TeamSeaking>(connection).expect("Error loading");
    let mut return_val = String::new();

    for item in res {
        return_val += &format!("**Id**: {}\n**Name**: {}\n**Username**: {}\n**Description**: {}\n\n", item.id, item.name, item.discordname, item.description).to_owned();
    }

    format!("{}\n", return_val)
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("get_table")
        .description("Returns the complete table of available users")
}