use serenity::{
    builder::CreateApplicationCommand,
	model::prelude::{
        command::CommandOptionType,
		interaction::application_command::{ ApplicationCommandInteraction, CommandDataOptionValue }
    }
};

use super::super::{
    pq,
	super::{ models::*, schema::{ team_seaking::dsl::*, teams::dsl::* } }
};

use diesel::prelude::*;

pub fn get_table(command_interaction: &mut ApplicationCommandInteraction) -> String {
    let connection = &mut pq::connect::establish_connection();

    let options = &command_interaction.data.options;

    let option_id = options
        .get(0)
        .expect("Expected user option")
        .resolved
        .as_ref()
        .expect("Expected user object");

    let mut return_val = String::new();

    if let CommandDataOptionValue::Integer(value) = option_id {
        match value {
            1 => {
                let res = team_seaking
                    .load::<TeamSeaking>(connection)
                    .expect("Error loading");

                for item in res {
                    return_val += &format!(
                        "**Id**: {}\n**Name**: {}\n**Username**: {}\n**Description**: {}\n\n",
                        item.id, item.name, item.discordname, item.description
                    )
                    .to_owned();
                }
            }
            2 => {
                let res = teams.load::<Teams>(connection).expect("Error loading");

                for item in res {
                    return_val += &format!(
                        "**Id**: {}\n**Team Name**: {}\n**Leader**: {}\n**Members**: {}\n**Description**: {}\n**Points**: {}\n\n",
                        item.id, item.team_name, item.leader, item.members, item.description, item.points
                    )
                    .to_owned();
                }
            }
            _ => {
                return_val = format!("Invalid choice {}", value);
            }
        }
    }

    format!("{}\n", return_val)
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("get_table")
        .description("Returns the complete table of available users")
        .create_option(|option| {
            option
                .name("id")
                .description("1 for user table, 2 for teams table")
                .kind(CommandOptionType::Integer)
                .required(true)
        })
}
