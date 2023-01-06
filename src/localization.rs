use lazy_static::lazy_static;
use tbot::{
    markup::{bold, inline_code, markdown_v2, Formattable},
    types::{parameters::Text, User},
};

fn images(comma: &'static str) -> impl Formattable {
    (
        inline_code([".jpg"]),
        comma,
        inline_code([".png"]),
        comma,
        inline_code([".bmp"]),
        comma,
        inline_code([".tiff"]),
        comma,
        inline_code([".webp"]),
    )
}

fn get_language(user: Option<&User>) -> Option<&str> {
    match user {
        Some(User {
            language_code: Some(language),
            ..
        }) => Some(language.as_str()),
        _ => None,
    }
}

pub fn help_message(user: Option<&User>) -> Text<'_> {
    lazy_static! {
        static ref ENGLISH: String = markdown_v2((
            bold("Hello!"),
            " I'm a bot that can extract images from ",
            inline_code([".attheme"]),
            " files or put images in them. To extract a wallpaper, just send \
             me a theme. If you want to put a wallpaper, send me the theme and \
             then send the image (as an image or a ",
            images(", "),
            " document) in reply to that theme. I'll reply you with the new \
             theme removing the old wallpaper.",
        )).to_string();

        static ref RUSSIAN: String = markdown_v2((
            bold("Привет!"),
            " Я — бот, который может вытаскивать обои из ",
            inline_code([".attheme"]),
            " файлов или ставить их. Чтобы вытащить обои, просто отправь тему. \
             Если хотите поставить обои, то отправьте мне тему, а затем \
             картинку (как фото или документом ",
            images(", "),
            ") ответом на эту тему. В ответ я отправлю тему с новыми обоями.",
        )).to_string();

        static ref PERSIAN: String = markdown_v2((
            "سلام! من یک ربات هستم که میتونم تصویر زمینه ی فایل های تم رو استخراج کنم و با یک تصویر زمینه ی جدید در تم ها قرار بدم. برای استخراج تصویر زمینه ی یک تم فقط کافیه اون تم رو به من ارسال کنی. اگر میخوای یک تصویر زمینه ی جدید در تم اعمال کنی، تم مورد نظرتو به من ارسال کن و بعدش در پاسخ به تم ارسالی تصویر زمینه ی مورد نظرتو ارسال کن. یادت نره فرمت مجاز تصاویر ",
            images(" و "),
            " هستش. بعدش من تصویر زمینه ی قدیمی تم رو حذف و سپس تصویر زمینه مورد نظر شمارو در تم قرار میدم و ارسال میکنم.",
        )).to_string();
    }

    match get_language(user) {
        Some("ru") => Text::with_markdown_v2(&RUSSIAN),
        Some("fa") => Text::with_markdown_v2(&PERSIAN),
        Some("en") | _ => Text::with_markdown_v2(&ENGLISH),
    }
}

pub fn image_caption(user: Option<&User>) -> Text<'_> {
    match get_language(user) {
        Some("ru") => Text::with_plain(
            "Вот обои!\n\
             \n\
             Если хочешь поменять обои в теме, отправь картинку в ответ \
             на свою тему.",
        ),
        Some("fa") => Text::with_plain(
            "بفرما، اینم تصویر زمینه تم!\n\
             \n\
             اگر میخوای پس زمینه رو تغییر بدی، یک تصویر در پاسخ به فایل تم بفرست.",
        ),
        Some("en") | _ => Text::with_plain(
            "Here's the wallpaper!\n\n
             \n\
             If you want to change the wallpaper, send a picture in reply \
             to your theme.",
        )
    }
}

pub fn image_file_name(user: Option<&User>, theme_name: &str) -> String {
    match get_language(user) {
        Some("ru") => format!("Обои из {}.jpg", theme_name),
        Some("fa") => format!("تصویر زمینه {}.jpg", theme_name),
        Some("en") | _ => format!("Wallpaper of {}.jpg", theme_name),
    }
}

pub fn image_with_no_reply(user: Option<&User>) -> Text<'_> {
    match get_language(user) {
        Some("ru") => Text::with_plain(
            "Если ты хочешь поставить обои в теме, то картинку надо отправить \
            ответом на тему.",
        ),
        Some("fa") => Text::with_plain(
            "اِهِم! اگر میخوای تصویر زمینه جدید در تم قرار بدی، باید تصویر زمینه رو در پاسخ به فایل تم ارسال کنی :)",
        ),
        Some("en") | _ => Text::with_plain(
            "Ehm, if you want me to with_put a wallwith_pawith_per inside a theme, you should \
            rewith_ply to the message with that theme.",
        ),
    }
}

pub fn no_theme_in_reply(user: Option<&User>) -> Text<'_> {
    match get_language(user) {
        Some("ru") => Text::with_plain(
            "Хм, кажется, что в сообщении, на которое ты ответил, нет темы. \
            Попробуй снова.",
        ),
        Some("fa") => Text::with_plain(
            "هومم، به نظر میرسه پیام شما در پاسخ به فایل تم نیست! دوباره امحتان کن.",
        ),
        Some("en") | _ => Text::with_plain(
            "Hmm, doesn't seem the message you replied to has a theme. \
             Try again.",
        ),
    }
}

pub fn start_message(user: Option<&User>) -> Text<'_> {
    match get_language(user) {
        Some("ru") => Text::with_plain(
            "Привет! Я — бот, который может вытаскивать обои из тем \
            для Telegram на Android или ставить их. Просто отправь тему, \
            а я отправлю обои.",
        ),
        Some("fa") => Text::with_plain(
            "سلام! من یک ربات هستم که میتونم تصاویر رو در تم های اندروید تلگرام استخراج کنم و یا تصویر زمینه ای رو در تم ها قرار بدم. فقط کافیه یک تم به من ارسال کنی، بعدش من تصویر زمینه ی تم رو بهتون میفرستم.",
        ),
        Some("en") | _ => Text::with_plain(
            "Hello! I'm a bot that can extract images from Android Telegram \
            themes or put them. Just send me one, I'll send back \
            the wallpaper.",
        ),
    }
}

pub fn theme_caption(user: Option<&User>) -> Text<'_> {
    match get_language(user) {
        Some("ru") => Text::with_plain("Классные обои, поставил их в тему!"),
        Some("fa") => Text::with_plain(
            "تصویر زمینه مناسبیه! من این تصویر زمینه رو در تم قرار دادم!",
        ),
        Some("en") | _ => {
            Text::with_plain("Nice wallpaper, I've added it to the theme!")
        }
    }
}

pub fn theme_with_no_image(user: Option<&User>) -> Text<'_> {
    match get_language(user) {
        Some("ru") => Text::with_plain(
            "Хм, кажется, что в этой теме нет обоев. Но я могу поставить обои \
            в неё! Просто отправь сообщение ответом на эту тему.",
        ),
        Some("fa") => Text::with_plain(
            "عجب! به نظر میرسه تم ارسالی تصویر زمینه ای نداره. ولی نگران نباش! من میتونم یک تصویر زمینه بهش اضافه کنم. کافیه یک تصویر زمینه در پاسخ به این تم به من ارسال کنی.",
        ),
        Some("en") | _ => Text::with_plain(
            "Hmm, it looks like your theme doesn't have a wallpaper. But I can \
            put one inside it! Just send an image in reply to this theme.",
        ),
    }
}

pub fn unknown_file_extension(user: Option<&User>) -> Text<'_> {
    lazy_static! {
        static ref ENGLISH: String = markdown_v2((
            "Hmm, I don't know such theme or image extension. For themes, \
            it must be ",
            inline_code([".attheme"]),
            "; for images, it must be a ",
            images(", "),
            " file.",
        ))
        .to_string();
        static ref RUSSIAN: String = markdown_v2((
            "Хм, я не знаю такую такое расширение темы или картинки. У тем \
            должно быть расширение ",
            inline_code([".attheme"]),
            ", у картинок — ",
            images(", "),
            "."
        ))
        .to_string();
        static ref PERSIAN: String = markdown_v2((
            "من بعضی تمها یا فایلها و تصاویر رو نمیشناسم! فرمت فایلهای تم ",
            inline_code([".attheme"]),
            " هست. فرمت تصاویر هم ",
            images(" و "),
            " میتونم قبول کنم.",
        ))
        .to_string();
    }
    match get_language(user) {
        Some("ru") => Text::with_markdown_v2(&RUSSIAN),
        Some("fa") => Text::with_markdown_v2(&PERSIAN),
        Some("en") | _ => Text::with_markdown_v2(&ENGLISH),
    }
}
