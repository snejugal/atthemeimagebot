/* eslint-disable max-len */
const en = {
  helpMessage:
    () => `*Hello!* I'm a bot that can extract images from .attheme files or put images in them. To extract a wallpaper, just send me a theme. If you want to put a wallpaper, send me the theme and then send the image (as an image or a \`jpg\`, \`png\`, \`bmp\` document) in reply to that theme. I'll reply you with the new theme removing the old wallpaper.`,
  imageCaption:
    () => `Here's the wallpaper!`,
  imageFileName:
    (themeName) => `Wallpaper of ${themeName}.jpg`,
  imageWithNoReply:
    () => `Ehm, if you want me to put a wallpaper inside a theme, you should reply to the message with that theme.`,
  noThemeInReply:
    () => `Hmm, doesn't seem the message you replied to has a theme. Try again.`,
  startMessage:
    () => `Hello! I'm a bot that can extract images from Android Telegram themes or put them. Just send me one, I'll send back the wallpaper.`,
  themeCaption:
    () => `Nice wallpaper, I've added it to the theme!`,
  themeWithNoImage:
    () => `Hmm, it looks like your theme doesn't have a wallpaper. But I can put one inside it! Just send an image in reply to this theme.`,
  unknownFileExtension:
    () => `Hmm, I don't know such theme or image extension. For themes, it must be .attheme; for images, it must be \`jpg\`, \`png\`, \`bmp\` file.`,
};

// TODO: add support for other languages

module.exports = { en };