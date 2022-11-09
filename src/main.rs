// Adding the allow lint on the actual enum variant messes with the derive macro
#![allow(clippy::upper_case_acronyms)]

use std::error::Error;

use teloxide::{
    prelude2::*,
    types::{MessageKind, ParseMode},
    utils::{command::BotCommand, markdown::escape},
};

#[derive(BotCommand, Clone)]
#[command(description = "These commands are supported:")]
enum Command {
    #[command(description = "display this text.", rename = "lowercase")]
    Help,
    #[command(description = "apply for ETCG, totally legit.")]
    ETCG,
    #[command(description = "wish everyone farewell.", rename = "lowercase")]
    Goodbye,
}

async fn answer(
    bot: AutoSend<Bot>,
    message: Message,
    command: Command,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    // try to find out who sent this message, feels like there's a better way to do this
    let mut sender_id = None;
    let sender = if let MessageKind::Common(common) = &message.kind {
        if let Some(user) = &common.from {
            sender_id = Some(user.id);
            if let Some(last_name) = &user.last_name {
                format!("{} {last_name}", user.first_name)
            } else {
                user.first_name.clone()
            }
        } else {
            "someone".to_owned()
        }
    } else {
        "someone".to_owned()
    };

    match command {
        Command::Help => {
            bot.send_message(message.chat.id, Command::descriptions())
                .await?;
        }

        // the main command for this bot
        Command::ETCG => {
            log::info!("ETCG message received: {message:?}");

            let text = format!(
                "Do you want to order your very own cards for the *ESTIEM Trading Card Game*? \
                Then sign up by using [this form]\
                (https://docs.google.com/forms/d/e/1FAIpQLSdZFf_fOO7uaA8JKeDUwPCRo4e-S-RnPklU_syOGXCMPll9xA/viewform)\
                \\!\n\
                \n\
                If you have any questions, feel free to bother @erikviktor here or personally at CM\\.\n\
                \n\
                _This message was sent after actions made by {}\\. If you receive it by a mistake, \
                please submit a pull request [on GitHub](https://github.com/o1oo11oo/ETCGBot)\\._",
                escape(&sender)
            );

            bot.parse_mode(ParseMode::MarkdownV2)
                .send_message(message.chat.id, text)
                .disable_web_page_preview(true)
                .await?;
        }

        // say goodbye!
        Command::Goodbye => {
            log::info!("GoodBye message received: {message:?}");

            // only leave the chat if sent by @o1oo11ooo1ooo11o
            if sender_id == Some(709158714) {
                // craft goodbye message
                let text = format!(
                    "Time to leave again\\.\n\
                    \n\
                    Thank you everyone for using the ETCGBot, \
                    and have fun with the *ESTIEM Trading Card Game*\\!\n\
                    \n\
                    _This message was sent after actions made by {}\\. If you receive it by a mistake, \
                    please submit a pull request [on GitHub](https://github.com/o1oo11oo/ETCGBot)\\._",
                    escape(&sender)
                );

                // send the message
                bot.clone()
                    .parse_mode(ParseMode::MarkdownV2)
                    .send_message(message.chat.id, text)
                    .await?;

                // leave the group
                bot.leave_chat(message.chat.id).await?;
            } else {
                // craft warning message
                let text = format!(
                    "Sorry, you are not allowed to use this command\\!\n\
                    \n\
                    _This message was sent after actions made by {}\\. If you receive it by a mistake, \
                    please submit a pull request [on GitHub](https://github.com/o1oo11oo/ETCGBot)\\._",
                    escape(&sender)
                );

                // send the message
                bot.parse_mode(ParseMode::MarkdownV2)
                    .send_message(message.chat.id, text)
                    .disable_web_page_preview(true)
                    .await?;
            }
        }
    };

    Ok(())
}

#[tokio::main]
async fn main() {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }

    pretty_env_logger::init();
    log::info!("Starting ETCGBot...");

    if std::env::var("TELOXIDE_TOKEN").is_err() {
        log::warn!("No API token found, set it as TELOXIDE_TOKEN or enter it here: ");
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("error: unable to read user input");
        std::env::set_var("TELOXIDE_TOKEN", input);
        log::info!("API token received, resuming startup...");
    }

    let bot = Bot::from_env().auto_send();

    teloxide::repls2::commands_repl(bot, answer, Command::ty()).await;
}
