mod commands;

use anyhow::anyhow;
use serenity::{ async_trait, model::prelude::GuildId };
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use shuttle_secrets::SecretStore;
use tracing::info;
use commands::{
    help,
    whoami
};
struct Bot;

#[async_trait]
impl EventHandler for Bot {
    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);

        let guild_id = GuildId(356901968653844484);

        let commands = GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
            commands
                .create_application_command(|command| { command.name("help").description("Show the help message.") })
                .create_application_command(|command| { command.name("whoami").description("See some info about you.") })
        }).await.unwrap();

        info!("{:#?}", commands);
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        let message_component = interaction.clone();

        if let Interaction::ApplicationCommand(command) = interaction {
            let response_content = match command.data.name.as_str() {
                "help" => help().await.to_owned(),
                "whoami" => whoami(message_component.message_component().unwrap()).await.to_owned(),
                command => unreachable!("Unknown command: {}", command),
            };

            let create_interaction_response =
            command.create_interaction_response(&ctx.http, |response| {
                response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message| message.content(response_content))
            });

            if let Err(why) = create_interaction_response.await {
                eprintln!("Cannot respond to slash command: {}", why);
            }
        }
    }
}

#[shuttle_service::main]
async fn serenity(#[shuttle_secrets::Secrets] secret_store: SecretStore) -> shuttle_service::ShuttleSerenity {
    // Get the discord token set in `Secrets.toml`
    let token = if let Some(token) = secret_store.get("DISCORD_TOKEN") {
        token
    } else {
        return Err(anyhow!("'DISCORD_TOKEN' was not found").into());
    };

    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let client = Client::builder(&token, intents)
        .event_handler(Bot)
        .await
        .expect("Err creating client");

    Ok(client)
}