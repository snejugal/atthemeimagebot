mod localization;

use attheme::{Attheme, ColorSignature::Hex};
use image::ImageOutputFormat::JPEG;
use localization::*;
use std::{path::Path, sync::Arc};
use tbot::{
    connectors::Https,
    contexts,
    prelude::*,
    types::{input_file, message, parameters::Text, Document},
    Bot,
};

const SUPPORTED_EXTENSIONS: [&str; 6] = ["png", "jpg", "jpeg", "bmp", "tiff", "webp"];

fn download_document(
    bot: &Arc<Bot<Https>>,
    document: &Document,
) -> impl Future<Item = Vec<u8>, Error = ()> {
    let bot = Arc::clone(bot);

    bot.get_file(document)
        .into_future()
        .map_err(|err| {
            dbg!(err);
        })
        .and_then(move |file| {
            bot.download_file(&file).map_err(|err| {
                dbg!(err);
            })
        })
}

fn set_wallpaper(context: &contexts::Document<Https>, document: &Document) {
    let bot = Arc::clone(&context.bot);
    let chat_id = context.chat.id;
    let reply_id = context.message_id;
    let theme_name = match &document.file_name {
        Some(filename) => filename.clone(),
        None => return,
    };
    let theme_caption = Text::markdown(&theme_caption(&context.from));

    let download_image = download_document(&context.bot, &context.document);
    let download_theme = download_document(&context.bot, document);

    let download = download_image
        .join(download_theme)
        .map(move |(image, theme)| {
            let mut theme = Attheme::from_bytes(&theme);

            let image = match image::load_from_memory(&image) {
                Ok(image) => image,
                Err(err) => {
                    dbg!(err);
                    return;
                }
            };

            let mut wallpaper = Vec::new();

            if let Err(error) = image.write_to(&mut wallpaper, JPEG(255)) {
                eprintln!("Error while writing to wallpaper: {:#?}", error);
                return;
            }

            theme.variables.remove("chat_wallpaper");
            theme.wallpaper = Some(wallpaper);

            let reply = bot
                .send_document(
                    chat_id,
                    input_file::Document::bytes(&theme_name, &theme.to_bytes(Hex))
                        .caption(theme_caption),
                )
                .reply_to_message_id(reply_id)
                .into_future()
                .map_err(|err| {
                    dbg!(err);
                });

            tbot::spawn(reply);
        });

    tbot::spawn(download);
}

fn extract_wallpaper(context: &contexts::Document<Https>, document: &Document) {
    let bot = Arc::clone(&context.bot);
    let chat_id = context.chat.id;
    let reply_id = context.message_id;
    let theme_name = match &document.file_name {
        Some(filename) => filename.clone(),
        None => return,
    };
    let image_file_name = image_file_name(&context.from, theme_name);
    let image_caption = Text::markdown(image_caption(&context.from));
    let theme_with_no_image = Text::markdown(theme_with_no_image(&context.from));

    let document = download_document(&context.bot, document).map(move |bytes| {
        let theme = Attheme::from_bytes(&bytes);

        match theme.wallpaper {
            Some(image) => {
                let reply = bot
                    .send_document(
                        chat_id,
                        input_file::Document::bytes(&image_file_name, &image)
                            .caption(image_caption),
                    )
                    .reply_to_message_id(reply_id)
                    .into_future()
                    .map_err(|err| {
                        dbg!(err);
                    });

                tbot::spawn(reply);
            }
            None => {
                let reply = bot
                    .send_message(chat_id, theme_with_no_image)
                    .reply_to_message_id(reply_id)
                    .into_future()
                    .map_err(|err| {
                        dbg!(err);
                    });

                tbot::spawn(reply);
            }
        };
    });

    tbot::spawn(document);
}

fn main() {
    let mut bot = tbot::bot!("BOT_TOKEN").event_loop();

    bot.start(|context| {
        let message = Text::markdown(start_message(&context.from));

        let reply = context.send_message(message).into_future().map_err(|err| {
            dbg!(err);
        });

        tbot::spawn(reply);
    });

    bot.help(|context| {
        let message = Text::markdown(help_message(&context.from));

        let reply = context.send_message(message).into_future().map_err(|err| {
            dbg!(err);
        });

        tbot::spawn(reply);
    });

    bot.document(|context| {
        let filename = match &context.document.file_name {
            Some(name) => name,
            None => return,
        };

        let path = Path::new(&filename);
        let extension: String = match path.extension() {
            Some(extension) => extension.to_string_lossy().to_lowercase().to_string(),
            None => "".into(),
        };

        if extension == "attheme" {
            extract_wallpaper(&context, &context.document);
            return;
        }

        if SUPPORTED_EXTENSIONS.contains(&extension.as_str()) {
            let reply_to = match &context.reply_to {
                Some(reply_to) => reply_to,
                None => {
                    let reply = context
                        .send_message_in_reply(Text::markdown(image_with_no_reply(&context.from)))
                        .into_future()
                        .map_err(|err| {
                            dbg!(err);
                        });

                    tbot::spawn(reply);
                    return;
                }
            };

            let no_theme_in_reply = || {
                let reply = context
                    .send_message_in_reply(Text::markdown(no_theme_in_reply(&context.from)))
                    .into_future()
                    .map_err(|err| {
                        dbg!(err);
                    });

                tbot::spawn(reply);
            };

            let document = match &reply_to.kind {
                message::Kind::Document(document, ..) => document,
                _ => {
                    no_theme_in_reply();
                    return;
                }
            };

            let filename = match &document.file_name {
                Some(filename) => filename,
                None => return,
            };

            let path = Path::new(&filename);
            let extension: String = match path.extension() {
                Some(extension) => extension.to_string_lossy().to_string(),
                None => "".into(),
            };

            if extension != "attheme" {
                no_theme_in_reply();
                return;
            }

            set_wallpaper(&context, &document);
            return;
        }

        let reply = context
            .send_message_in_reply(Text::markdown(unknown_file_extension(&context.from)))
            .into_future()
            .map_err(|err| {
                dbg!(err);
            });

        tbot::spawn(reply);
    });

    bot.polling().start();
}
