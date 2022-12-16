use super::pq;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::prelude::interaction::application_command::CommandDataOptionValue;

pub fn delete_record(command_interaction: &mut ApplicationCommandInteraction) -> String {
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

    let num_deleted = pq::interface::delete_row(conn, search);

    format!("Deleted {} post", num_deleted)
}

pub fn delete_table(_command_interaction: &mut ApplicationCommandInteraction) -> String {
    let conn = &mut pq::connect::establish_connection();

    let result = pq::interface::delete_table(conn);

    format!("Delete returned \"{:?}\"", result)
}

pub fn register_table_delete(
    command: &mut CreateApplicationCommand,
) -> &mut CreateApplicationCommand {
    command
        .name("delete_table")
        .description("Deletes all rows in a table")
}

pub fn register_row_delete(
    command: &mut CreateApplicationCommand,
) -> &mut CreateApplicationCommand {
    command
        .name("delete_row")
        .description("Delete a row with the specified id")
        .create_option(|option| {
            option
                .name("username")
                .description("Discord user's username")
                .kind(CommandOptionType::String)
                .required(true)
        })
}
