const Telegraf = require(`telegraf`);
const Attheme = require(`attheme-js`);
const request = require(`request-promise`);
const jimp = require(`jimp`);

const { token, options } = require(`./config`);
const localization = require(`./localization`);

const JIMP_EXTENSIONS = [`png`, `bmp`];
const ATTHEME_EXTENSION_LENGTH = `.attheme`.length;

const bot = new Telegraf(token, options);

const getBuffer = (imageProcession) => new Promise((resolve, reject) => {
  imageProcession.getBuffer(jimp.MIME_JPEG, (error, result) => {
    if (error) {
      reject(error);
    } else {
      resolve(result);
    }
  });
});

bot.context.sendChatAction = function (action) {
  bot.telegram.sendChatAction(this.chat.id, action);
};

bot.context.startSendingChatAction = function (action = `typing`) {
  const interval = setInterval(() => this.sendChatAction(action), 4 * 1000);

  this.sendChatAction(action);

  const stop = () => clearInterval(interval);

  return { stop };
};

bot.context.downloadFile = async function (isFromReply) {
  const file = isFromReply ?
    await bot.telegram.getFile(this.message.reply_to_message.document.file_id)
    : await bot.telegram.getFile(this.document.file_id);

  const result = await request({
    encoding: null,
    uri: `http://api.telegram.org/file/bot${token}/${file.file_path}`,
  });

  return result;
};

bot.context.downloadImage = async function () {
  const file = await bot.telegram.getFile(
    this.message.photo.slice().pop().file_id,
  );

  const result = await request({
    encoding: null,
    uri: `http://api.telegram.org/file/bot${token}/${file.file_path}`,
  });

  return result;
};

/* eslint-disable camelcase */
bot.context.MARKDOWN = { parse_mode: `Markdown` };
/* eslint-enable camelcase */

bot.use((context, next) => {
  if (`document` in context.message) {
    context.document = context.message.document;
  }
  next();
});

bot.command(`start`, (context) => {
  context.reply(localization.en.startMessage(), context.MARKDOWN);
});

bot.command(`help`, (context) => {
  context.reply(localization.en.helpMessage(), context.MARKDOWN);
});

bot.on(`document`, async (context) => {
  const { message } = context;
  const { document } = message;
  const uploadStatus = context.startSendingChatAction(`upload_document`);

  if (message.reply_to_message) {
    if (
      !(`document` in message.reply_to_message) ||
      !message.reply_to_message.document.file_name.endsWith(`.attheme`)
    ) {
      context.reply(localization.en.noThemeInReply(), context.MARKDOWN);
    }

    const themeContent = (await context.downloadFile(true)).toString(`binary`);
    const theme = new Attheme(themeContent);

    let image;

    const extension = document.file_name
      .split(`.`)
      .pop()
      .toLowerCase();

    // JPG to JPG convertion is useless, so it's processed separately
    if (extension == `jpg` || extension == `jpeg`) {
      image = (await context.downloadFile()).toString(`binary`);
    } else if (JIMP_EXTENSIONS.includes(extension)) {
      const imageProcession = await jimp.read(await context.downloadFile());

      image = (await getBuffer(imageProcession)).toString(`binary`);
    } else {
      context.reply(localization.en.unknownFileExtension());

      return;
    }

    theme[Attheme.IMAGE_KEY] = image;
    delete theme.chat_wallpaper;

    context.replyWithDocument(
      {
        filename: message.reply_to_message.document.file_name,
        source: Buffer.from(Attheme.asText(theme), `binary`),
      },
      { caption: localization.en.themeCaption() },
    );
  } else if (document.file_name.endsWith(`.attheme`)) {
    const themeName = document.file_name.slice(0, -ATTHEME_EXTENSION_LENGTH);
    const themeContent = (await context.downloadFile()).toString(`binary`);
    const theme = new Attheme(themeContent);

    if (Attheme.IMAGE_KEY in theme) {
      const themeImage = theme[Attheme.IMAGE_KEY];

      await context.replyWithDocument(
        {
          filename: localization.en.imageFileName(themeName),
          source: Buffer.from(themeImage, `binary`),
        },
        {
          caption: localization.en.imageCaption(),
          ...context.MARKDOWN,
        },
      );
    } else {
      context.reply(localization.en.themeWithNoImage(), context.MARKDOWN);
    }
  } else {
    context.reply(localization.en.imageWithNoReply(), context.MARKDOWN);
  }

  uploadStatus.stop();
});

bot.on(`photo`, async (context) => {
  const { message } = context;
  const uploadStatus = context.startSendingChatAction(`upload_document`);

  if (
    `reply_to_message` in message &&
    message.reply_to_message.document.file_name.endsWith(`.attheme`)
  ) {
    const themeContent = (await context.downloadFile(true)).toString(`binary`);
    const theme = new Attheme(themeContent);
    const image = (await context.downloadImage()).toString(`binary`);

    theme[Attheme.IMAGE_KEY] = image;
    delete theme.chat_wallpaper;

    context.replyWithDocument(
      {
        filename: message.reply_to_message.document.file_name,
        source: Buffer.from(Attheme.asText(theme), `binary`),
      },
      { caption: localization.en.themeCaption() },
    );
  } else {
    context.reply(localization.en.imageWithNoReply(), context.MARKDOWN);
  }

  uploadStatus.stop();
});

bot.startPolling();
console.log(`@${options.username} is running…`);