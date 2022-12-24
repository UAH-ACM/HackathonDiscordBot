use super::super::pq;

use serenity::{
    builder::CreateApplicationCommand,
	model::prelude::{
        command::CommandOptionType,
		interaction::application_command::{ApplicationCommandInteraction, CommandDataOptionValue}
    }
};

pub fn delete_record_by_username(
    command_interaction: &mut ApplicationCommandInteraction,
) -> String {
    let conn = &mut pq::connect::establish_connection();

    let options = &command_interaction.data.options;

    let name_option = options
        .get(0)
        .expect("Expected user option")
        .resolved
        .as_ref()
        .expect("Expected user object");

    let search: String;

    if let CommandDataOptionValue::String(value) = name_option {
        search = format!("%{}%", value)
    } else {
        return format!("{:?} is not a valid id", name_option);
    }

    let num_deleted;

    match pq::interface::user_delete_row(conn, search) {
        Ok(good) => num_deleted = good,
		Err(bad) => return format!("{}", bad),
    }

    format!("Deleted {} row(s)", num_deleted)
}

pub fn delete_record_by_id(command_interaction: &mut ApplicationCommandInteraction) -> String {
    let conn = &mut pq::connect::establish_connection();

    let options = &command_interaction.data.options;

    let name_option = options
        .get(0)
        .expect("Expected user option")
        .resolved
        .as_ref()
        .expect("Expected user object");

    let table_switch = options
        .get(1)
        .expect("Expected user option")
        .resolved
        .as_ref()
        .expect("Expected user object");

    let search: i64;
    let mut num_deleted: i64 = 0;

    if let CommandDataOptionValue::Integer(choise_id) = table_switch {
        match choise_id {
            1 => {
                if let CommandDataOptionValue::String(value) = name_option {
                    search = value.parse::<i64>().expect("Not a valid integer");
                } else {
                    return format!("{:?} is not a valid string", name_option);
                }

                match pq::interface::user_delete_row_and_return_val(conn, search) {
                    Ok((_, good)) => num_deleted = good,
					Err(bad) => return format!("{}", bad),
                }
            }
            2 => {
                if let CommandDataOptionValue::String(value) = name_option {
                    search = value.parse::<i64>().expect("Not a valid integer");
                } else {
                    return format!("{:?} is not a valid string", name_option);
                }

				match pq::interface::team_delete_row_and_return_val(conn, search) {
                    Ok((_, good)) => num_deleted = good,
					Err(bad) => return format!("{}", bad),
				}
            }
            _ => num_deleted = -1,
        }
    }

    format!("Deleted {} row(s)", num_deleted)
}

pub fn delete_table(command_interaction: &mut ApplicationCommandInteraction) -> String {
    let conn = &mut pq::connect::establish_connection();
    let options = &command_interaction.data.options;

    let table_option = options
        .get(0)
        .expect("Expected user option")
        .resolved
        .as_ref()
        .expect("Expected user object");

    if let CommandDataOptionValue::Integer(choise_id) = table_option {
        match choise_id {
            1 => {
                let _ = pq::interface::user_delete_table(conn);
            }
			2 => {
                let _ = pq::interface::team_delete_table(conn);
            }
			_ => return format!("Invalid option")
		}
    }

    format!("Table deleted")
}

pub fn register_table_delete(
    command: &mut CreateApplicationCommand,
) -> &mut CreateApplicationCommand {
    command
        .name("delete_table")
        .description("Deletes all rows in a table")
        .create_option(|option| {
            option
                .name("switch")
                .description("1 for user table, 2 for teams table")
                .kind(CommandOptionType::Integer)
                .required(true)
        })
}

pub fn register_row_delete_username(
    command: &mut CreateApplicationCommand,
) -> &mut CreateApplicationCommand {
    command
        .name("delete_row_by_username")
        .description("Delete a row with the specified id")
        .create_option(|option| {
            option
                .name("username")
                .description("Discord user's username")
                .kind(CommandOptionType::String)
                .required(true)
        })
}

pub fn register_row_delete_id(
    command: &mut CreateApplicationCommand,
) -> &mut CreateApplicationCommand {
    command
        .name("delete_row_by_id")
        .description("Delete a row with the specified id")
        .create_option(|option| {
            option
                .name("id")
                .description("Discord user's id")
                .kind(CommandOptionType::String)
                .required(true)
        })
        .create_option(|option| {
            option
                .name("switch")
                .description("1 for user table, 2 for teams table")
                .kind(CommandOptionType::Integer)
                .required(true)
        })
}