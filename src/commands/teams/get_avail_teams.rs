use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;

use super::super::super::models::*;
use super::super::super::schema::teams::dsl::*;
use super::super::pq;
use diesel::prelude::*;

pub fn get_available_teams(_command_interaction: &mut ApplicationCommandInteraction) -> String {
    let connection = &mut pq::connect::establish_connection();

    let res: Vec<Teams>;

    match teams.load::<Teams>(connection) {
        Ok(good) => res = good,
		Err(bad) => return format!("{}", bad),
    }
    let mut return_val = String::new();

    for item in res {
        return_val += &format!(
                "**Team ID**: {}\n**Team Name**: {}\n**Leader**: {}\n**Members**: {}\n\n",
            item.id, item.team_name, item.leader, item.members
        )
        .to_owned();
    }

    format!("{}\n", return_val)
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("list_available_teams")
        .description("Returns the complete list of available teams")

}
