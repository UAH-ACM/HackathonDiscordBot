use serenity::builder::CreateApplicationCommand;
use serenity::prelude::*;
use diesel::RunQueryDsl;
use super::super::pq;

use super::super::super::{
    models::*,
	schema::teams::dsl::*
};

use serenity::model::{
    prelude::{
        command::CommandOptionType,
		interaction::application_command::ApplicationCommandInteraction,
		interaction::application_command::CommandDataOptionValue
    },
	channel::*,
	permissions::Permissions
};

// TODO: Create a ChannelCategory with 3 Channels: general, git, voice
pub async fn create_team(ctx: &Context, command_interaction: &mut ApplicationCommandInteraction) -> String {
    let mut result = String::new();

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
            return String::from("You cannot create another team");
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

	let guild_id_loc = command_interaction.guild_id.unwrap();
    let role = guild_id_loc.create_role(&ctx, |role| {
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

    match res {
        Err(e) => {
            result = format!("{}{}", result, e);
        },
		Ok(_) => {
            result = format!("{}{}", result, "Team created\n");
        }
    }

    let user_id = command_interaction.user.id;

    let mut member = guild_id_loc.member(&ctx, user_id).await.unwrap();
    let role_success = member.add_role(&ctx, new_role_id).await;

    match role_success {
        Err(e) => {
            result = format!("{}{}", result, e);
        },
		Ok(_) => {
            result = format!("{}{}", result, "Role created and assgined!\n");
        }
    }

	let everyone_id = guild_id_loc.roles(&ctx).await.ok().unwrap();
    let mut everyone: serenity::model::id::RoleId = serenity::model::id::RoleId(0);
    for (_, item) in everyone_id {
        if item.name == "@everyone" {
			everyone = item.id;
        }
    }


	let permissions = vec![ PermissionOverwrite {
        allow: Permissions::ADD_REACTIONS | Permissions::VIEW_CHANNEL | Permissions::SEND_MESSAGES | Permissions::EMBED_LINKS | Permissions::ATTACH_FILES | Permissions::READ_MESSAGE_HISTORY | Permissions::USE_EXTERNAL_EMOJIS | Permissions::CONNECT | Permissions::SPEAK | Permissions::USE_VAD | Permissions::CHANGE_NICKNAME | Permissions::USE_SLASH_COMMANDS | Permissions::REQUEST_TO_SPEAK | Permissions::CREATE_PUBLIC_THREADS | Permissions::CREATE_PRIVATE_THREADS | Permissions::USE_EXTERNAL_STICKERS | Permissions::SEND_MESSAGES_IN_THREADS | Permissions::USE_EMBEDDED_ACTIVITIES,
		deny: Permissions::empty(),
		kind: PermissionOverwriteType::Role(new_role_id),
	},
	PermissionOverwrite {
        allow: Permissions::empty(),
		deny: Permissions::VIEW_CHANNEL,
		kind: PermissionOverwriteType::Role(everyone)
    }];


	let channel_category = guild_id_loc.create_channel(&ctx, |channel_category| {
        channel_category
			.name(name_loc)
			.nsfw(false)
			.kind(ChannelType::Category)
			.permissions::<Vec<PermissionOverwrite>>(permissions.clone())
    }).await.unwrap();

    let mut general_channel = guild_id_loc.create_channel(&ctx, |channel_category| {
        channel_category
			.name("general")
			.nsfw(false)
			.kind(ChannelType::Text)
			.permissions::<Vec<PermissionOverwrite>>(permissions.clone())
    }).await.unwrap();

    let mut git_channel = guild_id_loc.create_channel(&ctx, |channel_category| {
        channel_category
			.name("git")
			.nsfw(false)
			.kind(ChannelType::Text)
			.permissions::<Vec<PermissionOverwrite>>(permissions.clone())
    }).await.unwrap();

    let mut voice_channel = guild_id_loc.create_channel(&ctx, |channel_category| {
        channel_category
			.name("voice")
			.nsfw(false)
			.kind(ChannelType::Voice)
			.permissions::<Vec<PermissionOverwrite>>(permissions.clone())
    }).await.unwrap();

	general_channel.edit(&ctx, |channel| {
        channel.category(channel_category.id)
    }).await.ok();
    git_channel.edit(&ctx, |channel| {
        channel.category(channel_category.id)
    }).await.ok();
    voice_channel.edit(&ctx, |channel| {
        channel.category(channel_category.id)
    }).await.ok();

	String::from(result)
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("create_team")
        .description("Create a team so others can jon you")
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
