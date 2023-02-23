use serenity::builder::CreateApplicationCommand;
use serenity::prelude::*;
use super::super::pq;

use super::super::super::{
    models::Teams,
	schema::teams::dsl::*
};

use diesel::{
    prelude::*,
	query_dsl::QueryDsl
};

use serenity::model::prelude::{
    command::CommandOptionType,
	interaction::application_command::{ ApplicationCommandInteraction, CommandDataOptionValue }
};

// TODO: Give user role from role id stored in the db
pub async fn join(ctx: &Context, command_interaction: &mut ApplicationCommandInteraction) -> String {
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
        return format!("{:?} is not a valid team ID", id_loc_option);
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

    let guild_id_loc = command_interaction.guild_id.unwrap();
    let role_id_loc = serenity::model::id::RoleId(result.role_id as u64);
    let user_id = command_interaction.user.id;

    let mut member = guild_id_loc.member(&ctx, user_id).await.unwrap();
    let role_success = member.add_role(&ctx, role_id_loc).await;

	// let members_list, a = diesel::update(teams.select(id_loc_option)).set(members.)

	let update_result = diesel::update(result).set(members.eq(new_members_list)).execute(connection);

    format!("{:?}{:?}\n", update_result, role_success)
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("join")
        .description("Allows you to join a team (Ask the leader if you can join first)")
		.create_option(|option| {
            option
                    .name("id")
                    .description("The ID of the team to join")
                    .kind(CommandOptionType::Integer)
                    .required(true)
        })
}
