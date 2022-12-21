use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::prelude::interaction::application_command::CommandDataOptionValue;

use super::super::super::models::Teams;
use super::super::super::schema::teams::dsl::*;
use super::super::pq;
use diesel::prelude::*;
use diesel::query_dsl::QueryDsl;

pub fn join(command_interaction: &mut ApplicationCommandInteraction) -> String {
    let connection = &mut pq::connect::establish_connection();
    let user_parent = command_interaction.member.as_mut().unwrap();
    let options = &command_interaction.data.options;
    let discord_name = format!("{}#{}", user_parent.user.name, user_parent.user.discriminator);

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

	let binding;

    match teams.find(id_loc).load::<Teams>(connection) {
        Ok(good) => binding = good,
		Err(bad) => return format!("{}", bad),
    }

    let result: &Teams = &* binding.get(0).unwrap();

	if result.leader == discord_name {
        return String::from("You cannot be a member of a team you are a leader of");
    }
	for sub_str in result.members.split(',').collect::<Vec<&str>>() {
        if String::from(sub_str) == discord_name {
            return String::from("Cannot join another team");
        }
    }

	let res: Vec<Teams>;

    match teams.load::<Teams>(connection) {
        Ok(good) => res = good,
		Err(bad) => return format!("{}", bad),
    }

    for item in res {
        if item.id == id_loc {
            continue;
        }
        if item.leader == discord_name {
            return String::from("You cannot be a member of a team you are a leader of");
        }
		for sub_str in item.members.split(',').collect::<Vec<&str>>() {
            if String::from(sub_str) == discord_name {
                return String::from("Cannot join another team");
            }
        }
    }

	let new_members_list = result.members.to_owned() + &String::from(format!("{}, ", discord_name)).to_owned();

    let update_result = diesel::update(teams).set(members.eq(new_members_list)).execute(connection);

    format!("{:?}\n", update_result)
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("join")
        .description("Returns the complete list of available teams")
		.create_option(|option| {
            option
                    .name("id")
                    .description("The ID of the team to join")
                    .kind(CommandOptionType::Integer)
                    .required(true)
        })
}
