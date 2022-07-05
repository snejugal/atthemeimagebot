mod localization;

use attheme::{Attheme, ColorSignature::Hex};
use dotenv::dotenv;
use image::ImageOutputFormat::Jpeg;
use std::{path::Path, sync::Arc};
use tbot::{
    contexts::{
        self,
        fields::{MediaMessage, Message},
    },
    prelude::*,
    types::{file::id::AsFileId, input_file, message, Document},
    Bot,
};
use tokio::try_join;

const SUPPORTED_EXTENSIONS: [&str; 6] =
    ["png", "jpg", "jpeg", "bmp", "tiff", "webp"];

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

#[tokio::main]
async fn main() {
    let _ = dotenv();
    let mut bot = Bot::from_env("BOT_TOKEN").event_loop();

    bot.start(start);
    bot.help(help);
    bot.document(document);
    bot.photo(photo);

    if let Ok(url) = std::env::var("WEBHOOK_URL") {
        bot.webhook(&url, 5001).http().start().await.unwrap();
    } else {
        bot.polling().start().await.unwrap();
    }
}

async fn start(context: Arc<contexts::Command<contexts::Text>>) {
    let message = localization::start_message(context.from.as_ref());
    let result = context.send_message(message).call().await;
    if let Err(err) = result {
        dbg!(err);
    }
}

async fn help(context: Arc<contexts::Command<contexts::Text>>) {
    let message = localization::help_message(context.from.as_ref());
    let result = context.send_message(message).call().await;
    if let Err(err) = result {
        dbg!(err);
    }
}

async fn document(context: Arc<contexts::Document>) {
    let file_name = match &context.document.file_name {
        Some(name) => name,
        None => return,
    };
    let path = Path::new(&file_name);
    let extension = match path.extension() {
        Some(extension) => extension.to_string_lossy().to_lowercase(),
        None => {
            let error_text =
                localization::unknown_file_extension(context.from.as_ref());
            let result = context.send_message_in_reply(error_text).call().await;
            if let Err(err) = result {
                dbg!(err);
            }
            return;
        }
    };

    if extension == "attheme" {
        let result = extract_wallpaper(&*context).await;
        if let Err(err) = result {
            dbg!(err);
        }
        return;
    }

    if SUPPORTED_EXTENSIONS.contains(&extension.as_str()) {
        let document = match get_document(&*context).await {
            Some(document) => document,
            None => return,
        };

        let image = &context.document;
        let result = set_wallpaper(&*context, image, document).await;
        if let Err(err) = result {
            dbg!(err);
        }
    }
}

async fn photo(context: Arc<contexts::Photo>) {
    let document = match get_document(&*context).await {
        Some(document) => document,
        None => return,
    };

    let image = context.photo.last().unwrap();
    let result = set_wallpaper(&*context, image, document).await;
    if let Err(err) = result {
        dbg!(err);
    }
}

async fn get_document(context: &impl MediaMessage) -> Option<&Document> {
    let reply_to = match context.reply_to() {
        Some(reply_to) => reply_to,
        None => {
            let error_text = localization::image_with_no_reply(context.from());
            let result = context.send_message_in_reply(error_text).call().await;
            if let Err(err) = result {
                dbg!(err);
            }
            return None;
        }
    };

    let document = match &reply_to.kind {
        message::Kind::Document(document, ..) => document,
        _ => {
            no_theme_in_reply(context).await;
            return None;
        }
    };

    let file_name = document.file_name.as_ref()?;
    if Path::new(&file_name).extension().map(|x| x == "attheme") == Some(true) {
        Some(document)
    } else {
        no_theme_in_reply(context).await;
        None
    }
}

async fn extract_wallpaper(context: &contexts::Document) -> Result<(), Error> {
    let theme_name = match &context.document.file_name {
        Some(theme_name) => theme_name,
        None => Err("No name in document")?,
    };

    let bytes = download(&context.bot, &context.document).await?;
    let theme = Attheme::from_bytes(&bytes);

    if let Some(image) = theme.wallpaper {
        let file_name =
            localization::image_file_name(context.from.as_ref(), theme_name);
        let caption = localization::image_caption(context.from.as_ref());
        let wallpaper =
            input_file::Document::bytes(&file_name, &image).caption(caption);

        context.send_document_in_reply(wallpaper).call().await?;
    } else {
        let error_text =
            localization::theme_with_no_image(context.from.as_ref());
        context.send_message_in_reply(error_text).call().await?;
    }

    Ok(())
}

async fn download(bot: &Bot, file: &impl AsFileId) -> Result<Vec<u8>, Error> {
    let file = bot.get_file(file).call().await?;
    Ok(bot.download_file(&file).await?)
}

async fn set_wallpaper(
    context: &impl Message,
    image: &impl AsFileId,
    theme: &Document,
) -> Result<(), Error> {
    let name = match &theme.file_name {
        Some(name) => name,
        None => Err("Document without a name")?,
    };

    let (image, theme) = try_join!(
        download(context.bot(), image),
        download(context.bot(), theme),
    )?;

    let mut theme = Attheme::from_bytes(&theme[..]);
    let image = image::load_from_memory(&image[..])?;

    let mut wallpaper = Vec::new();
    image.write_to(&mut wallpaper, Jpeg(255))?;

    theme.variables.remove("chat_wallpaper");
    theme.variables.remove("chat_wallpaper_gradient_to");
    theme.wallpaper = Some(wallpaper);

    let caption = localization::theme_caption(context.from());
    let bytes = theme.to_bytes(Hex);
    let new_theme = input_file::Document::bytes(&name, &bytes).caption(caption);
    context.send_document_in_reply(new_theme).call().await?;

    Ok(())
}

async fn no_theme_in_reply(context: &impl Message) {
    let error_text = localization::no_theme_in_reply(context.from());
    let result = context.send_message_in_reply(error_text).call().await;
    if let Err(err) = result {
        dbg!(err);
    }
}
