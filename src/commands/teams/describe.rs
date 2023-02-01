use serenity::builder::CreateApplicationCommand;
use super::super::pq;
use diesel::prelude::*;

use serenity::model::prelude::{
	command::CommandOptionType,
	interaction::application_command::ApplicationCommandInteraction,
	interaction::application_command::CommandDataOptionValue
};

use super::super::super::{
    models::*,
	schema::teams::dsl::*
};


pub fn get_team_descriptions(command_interaction: &mut ApplicationCommandInteraction) -> String {
    let connection = &mut pq::connect::establish_connection();
    let options = &command_interaction.data.options;

    let id_loc_option = options
	    .get(0)
        .expect("Expected user option")
        .resolved
        .as_ref()
        .expect("Expected user object");

    let id_loc: i64;

    if let CommandDataOptionValue::Integer(value) = id_loc_option {
        id_loc = * value;
    } else {
        return format!("{:?} is not a valid name", id_loc_option);
    }

	let result: &Teams;

    match teams.find(id_loc).load::<Teams>(connection) {
        Ok(good) =>  {
            let binding = good;
            result = &* binding.get(0).unwrap();
            return format!("{}\n", format!("**Team Name**: {}\n**Description**: {}\n\n", result.team_name, result.description));
        }
		Err(bad) => return format!("{}", bad),
	}
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("describe")
        .description("Will show you the desciotion of a particular team")
		.create_option(|option| {
            option
                    .name("id")
                    .description("The ID of the team to describe")
                    .kind(CommandOptionType::Integer)
                    .required(true)
        })
}
