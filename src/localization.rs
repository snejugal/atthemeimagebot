use tbot::types::User;

fn get_language(user: &Option<User>) -> Option<&str> {
    match user {
        Some(User {
            language_code: Some(language),
            ..
        }) => Some(language.as_str()),
        _ => None,
    }
}

pub fn help_message(user: &Option<User>) -> &'static str {
    match get_language(user) {
        Some("ru") => "*Привет!* Я — бот, который может вытаскивать обои из .attheme файлов или ставить их. Чтобы вытащить обои, просто отправь тему. Если хотите поставить обои, то отправьте мне тему, а затем картинку (как фото или документом `jpg`, `png`, `bmp`, `tiff`, `webp`) ответом на эту тему. В ответ я отправлю тему с новыми обоями.",
        Some("fa") => "سلام! من یک ربات هستم که میتونم تصویر زمینه ی فایل های تم رو استخراج کنم و با یک تصویر زمینه ی جدید در تم ها قرار بدم. برای استخراج تصویر زمینه ی یک تم فقط کافیه اون تم رو به من ارسال کنی. اگر میخوای یک تصویر زمینه ی جدید در تم اعمال کنی، تم مورد نظرتو به من ارسال کن و بعدش در پاسخ به تم ارسالی تصویر زمینه ی مورد نظرتو ارسال کن. یادت نره فرمت مجاز تصاویر `jpg` و `png` و `bmp` و `tiff` و `webp` هستش. بعدش من تصویر زمینه ی قدیمی تم رو حذف و سپس تصویر زمینه مورد نظر شمارو در تم قرار میدم و ارسال میکنم.",
        Some("en") | _ => "*Hello!* I'm a bot that can extract images from .attheme files or put images in them. To extract a wallpaper, just send me a theme. If you want to put a wallpaper, send me the theme and then send the image (as an image or a `jpg`, `png`, `bmp`, `tiff`, `webp` document) in reply to that theme. I'll reply you with the new theme removing the old wallpaper.",
    }
}

pub fn image_caption(user: &Option<User>) -> &'static str {
    match get_language(user) {
        Some("ru") => {
            "Вот обои!\n\n\

            Если хочешь поменять обои в теме, отправь картинку в ответ на свою тему."
        }
        Some("fa") => {
            "بفرما، اینم تصویر زمینه تم!\n\n\

            اگر میخوای پس زمینه رو تغییر بدی، یک تصویر در پاسخ به فایل تم بفرست."
        }
        Some("en") | _ => {
            "Here's the wallpaper!\n\n\

            If you want to change the wallpaper, send a picture in reply to your theme."
        }
    }
}

pub fn image_file_name(user: &Option<User>, theme_name: String) -> String {
    match get_language(user) {
        Some("ru") => format!("Обои из {}.jpg", theme_name),
        Some("fa") => format!("تصویر زمینه {}.jpg", theme_name),
        Some("en") | _ => format!("Wallpaper of {}.jpg", theme_name),
    }
}

pub fn image_with_no_reply(user: &Option<User>) -> &'static str {
    match get_language(user) {
        Some("ru") => "Если ты хочешь поставить обои в теме, то картинку надо отправить ответом на тему.",
        Some("fa") => "اِهِم! اگر میخوای تصویر زمینه جدید در تم قرار بدی، باید تصویر زمینه رو در پاسخ به فایل تم ارسال کنی :)",
        Some("en") | _ => "Ehm, if you want me to put a wallpaper inside a theme, you should reply to the message with that theme.",
    }
}

pub fn no_theme_in_reply(user: &Option<User>) -> &'static str {
    match get_language(user) {
        Some("ru") => {
            "Хм, кажется, что в сообщении, на которое ты ответил, нет темы. Попробуй снова."
        }
        Some("fa") => {
            "هومم، به نظر میرسه پیام شما در پاسخ به فایل تم نیست! دوباره امحتان کن."
        }
        Some("en") | _ => "Hmm, doesn't seem the message you replied to has a theme. Try again.",
    }
}

pub fn start_message(user: &Option<User>) -> &'static str {
    match get_language(user) {
        Some("ru") => "Привет! Я — бот, который может вытаскивать обои из тем для Telegram на Android или ставить их. Просто отправь тему, а я отправлю обои.",
        Some("fa") => "سلام! من یک ربات هستم که میتونم تصاویر رو در تم های اندروید تلگرام استخراج کنم و یا تصویر زمینه ای رو در تم ها قرار بدم. فقط کافیه یک تم به من ارسال کنی، بعدش من تصویر زمینه ی تم رو بهتون میفرستم.",
        Some("en") | _ => "Hello! I'm a bot that can extract images from Android Telegram themes or put them. Just send me one, I'll send back the wallpaper.",
    }
}

pub fn theme_caption(user: &Option<User>) -> &'static str {
    match get_language(user) {
        Some("ru") => "Классные обои, поставил их в тему!",
        Some("fa") => {
            "تصویر زمینه مناسبیه! من این تصویر زمینه رو در تم قرار دادم!"
        }
        Some("en") | _ => "Nice wallpaper, I've added it to the theme!",
    }
}

pub fn theme_with_no_image(user: &Option<User>) -> &'static str {
    match get_language(user) {
        Some("ru") => "Хм, кажется, что в этой теме нет обоев. Но я могу поставить обои в неё! Просто отправь сообщение ответом на эту тему.",
        Some("fa") => "عجب! به نظر میرسه تم ارسالی تصویر زمینه ای نداره. ولی نگران نباش! من میتونم یک تصویر زمینه بهش اضافه کنم. کافیه یک تصویر زمینه در پاسخ به این تم به من ارسال کنی.",
        Some("en") | _ => "Hmm, it looks like your theme doesn't have a wallpaper. But I can put one inside it! Just send an image in reply to this theme.",
    }
}

pub fn unknown_file_extension(user: &Option<User>) -> &'static str {
    match get_language(user) {
        Some("ru") => "Хм, я не знаю такую такое расширение темы или картинки. У тем должно быть расширение `.attheme`, у картинок — `jpg`, `png`, `bmp`, `tiff` или `webp`.",
        Some("fa") => "من بعضی تمها یا فایلها و تصاویر رو نمیشناسم! فرمت فایلهای تم .attheme هست. فرمت تصاویر هم `jpg` و `png` و `bmp` و `tiff` و `webp` میتونم قبول کنم.",
        Some("en") | _ => "Hmm, I don't know such theme or image extension. For themes, it must be .attheme; for images, it must be `jpg`, `png`, `bmp`, `tiff`, `webp` file.",
    }
}
