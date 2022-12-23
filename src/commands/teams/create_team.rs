use super::super::super::models::*;
use super::super::super::schema::teams::dsl::*;
use super::super::pq;
use diesel::RunQueryDsl;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::prelude::interaction::application_command::CommandDataOptionValue;
use serenity::prelude::*;


// TODO: Create a ChannelCategory with 3 Channels: general, git, voice
pub async fn create_team(ctx: &Context, command_interaction: &mut ApplicationCommandInteraction) -> String {
    let user_parent = command_interaction.member.as_mut().unwrap();
    let connection = &mut pq::connect::establish_connection();
    let options = &command_interaction.data.options;
    let discord_name = format!(
        "{}#{}",
        user_parent.user.name, user_parent.user.discriminator
    );

    let id_unwrapped = teams.load::<Teams>(connection);
    let id_loc: i64;

    match &id_unwrapped {
        Ok(good) => id_loc = good.len() as i64 + 1,
		Err(bad) => return format!("{}", bad),
    }

	for item in id_unwrapped.unwrap() {
        if item.leader == discord_name {
            return String::from("You cannot be a member of a team you are a leader of");
        }
    }

    let team_option = options.get(0).expect("Expected user option").resolved.as_ref().expect("Expected user object");
    let description_option = options.get(1).expect("Expected user option").resolved.as_ref().expect("Expected user object");
    let color_option = options.get(2).expect("Expected user option").resolved.as_ref().expect("Expected user object");

    let name_loc: &str;
    let description_loc: &str;
    let role_color: u64;

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

	if let CommandDataOptionValue::Integer(value) = color_option {
        role_color = * value as u64;
    } else {
        return format!("{:?} is not a valid color", team_option);
    }

	let guild_id = command_interaction.guild_id.unwrap();
    let role = guild_id.create_role(&ctx, |role| {
        role.name(format!("{}", name_loc))
			.colour(role_color)
			.mentionable(true)
    }).await;

    let new_role_id = role.as_ref().unwrap().id;

    let res = pq::interface::insert_team(
        connection,
        &id_loc,
        name_loc,
        description_loc,
        &discord_name[..],
		&(new_role_id.0 as i64)
    );

    let user_id = command_interaction.user.id;

    let mut member = guild_id.member(&ctx, user_id).await.unwrap();
    match member.add_role(&ctx, new_role_id).await {
        Err(e) => format!("{}", e),
		Ok(good) => {
            match res {
                Ok(_) => format!("Team created with role {:?}", good),
				Err(e) => format!("{}", e),
			}
        }
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
		.create_option(|option| {
            option
                    .name("team_color")
                    .description("The color of the team role")
                    .kind(CommandOptionType::Integer)
                    .required(true)
        })
}
