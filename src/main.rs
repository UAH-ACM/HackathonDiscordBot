pub mod commands;
pub mod models;
pub mod schema;

use dotenv::dotenv;
use serenity::async_trait;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::prelude::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(mut command) = interaction {
            let content = match command.data.name.as_str() {
                "help" => commands::help::help(),
                "whoami" => commands::whoami::whoami(&mut command),
                "checkin" => commands::checkin::checkin(&mut command),
                "get_table" => commands::get_table::get_table(&mut command),
                "delete_table" => commands::delete::delete_table(&mut command),
                "delete_row_by_id" => commands::delete::delete_record_by_id(&mut command),
                "delete_row_by_username" => commands::delete::delete_record_by_username(&mut command),
				"list_available_users" => commands::ls_avail::get_available_users(&mut command),
                _ => "not implemented :(".to_string(),
            };

            if content.len() > 2000 {
                use std::fs;
                let _ = fs::create_dir("./tmp");

                fs::write("./tmp/message.txt", &content).expect("Unable to write file");

                if let Err(why) = command
                    .create_interaction_response(&ctx.http, |response| {
                        response
                            .kind(InteractionResponseType::ChannelMessageWithSource)
                            .interaction_response_data(|message| message.add_file("./tmp/message.txt"))
                    })
                    .await
                {
                    println!("Cannot respond to slash command: {}", why);
                }
				let _ = fs::remove_file("./tmp/message.txt");
            } else {
                if let Err(why) = command
                    .create_interaction_response(&ctx.http, |response| {
                        response
                            .kind(InteractionResponseType::ChannelMessageWithSource)
                            .interaction_response_data(|message| message.content(content))
                    })
                    .await
                {
                    println!("Cannot respond to slash command: {}", why);
                }
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let main_discord_id_str =
            std::env::var("MAIN_DISCORD_GUILD_ID").expect("MAIN_DISCORD_GUILD_ID must be set.");
        let main_discord_id = GuildId(main_discord_id_str.parse::<u64>().unwrap());
        let test_discord_id_str =
            std::env::var("TEST_DISCORD_GUILD_ID").expect("TEST_DISCORD_GUILD_ID must be set.");
        let test_discord_id = GuildId(test_discord_id_str.parse::<u64>().unwrap());

        let _commands =
            GuildId::set_application_commands(&main_discord_id, &ctx.http, |commands| {
                commands
                    .create_application_command(|command| commands::help::register(command))
                    .create_application_command(|command| commands::whoami::register(command))
                    .create_application_command(|command| commands::checkin::register(command))
                    .create_application_command(|command| commands::get_table::register(command))
                    .create_application_command(|command| {
                        commands::delete::register_row_delete_id(command)
                    })
                    .create_application_command(|command| {
                        commands::delete::register_row_delete_username(command)
                    })
                    .create_application_command(|command| {
                        commands::delete::register_table_delete(command)
                    })
					.create_application_command(|command| {
                        commands::ls_avail::register(command)
                    })
            })
            .await;

        let _commands =
            GuildId::set_application_commands(&test_discord_id, &ctx.http, |commands| {
                commands
                    .create_application_command(|command| commands::help::register(command))
                    .create_application_command(|command| commands::whoami::register(command))
                    .create_application_command(|command| commands::checkin::register(command))
                    .create_application_command(|command| commands::get_table::register(command))
                    .create_application_command(|command| {
                        commands::delete::register_row_delete_id(command)
                    })
                    .create_application_command(|command| {
                        commands::delete::register_row_delete_username(command)
                    })
                    .create_application_command(|command| {
                        commands::delete::register_table_delete(command)
                    })
					.create_application_command(|command| {
                        commands::ls_avail::register(command)
                    })
            })
            .await;
    }
}

#[tokio::main]
async fn main() {
    // Configure the client with your Discord bot token in the environment.
    dotenv().ok();
    let discord_token = std::env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN must be set.");

    // Build our client.
    let mut client = Client::builder(discord_token, GatewayIntents::empty())
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
