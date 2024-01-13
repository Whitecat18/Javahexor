<div align="center">
  <img width="180px" src="share/javahexor.png" />
  <h1>Javahexor </h1>
  <p><i> An automated malicious javascript payload builder to receive Public IP address , Live Location , Browser Usage , screen width x height , Bookmars , cookie status , Network Monitoring etc through phishing.</i><br>
  <br>
    Created by <a href="https://twitter.com/Smukx07"> @Smukx</a> .</i></p>
   <img src="share/javahexor-result_1.png" width=70%>
   <img src="share/javahexor-result.png" width=70%/>
</div>

### Features 

* Register your key only time . It saves automatically for future operations .
* Fast Server Requests on high level network latency.
* Bind Custom websites and tunnel with it . 
* Multiple tunneling options with HTTPS.
* Get Fast Results using your Telegram bot.
* Can handle many requests at the same time.

### Testing

Tested at Arch and Kali Linux .

### Installation 

*clone the repository and run the ./build.sh*

```
git clone https://github.com/Whitecat18/Javahexor.git
cd Javahexor
./build.sh
```

## Quick Start
### Telebot Help

To Run Javahexor you need 2 things . Telegram bot API_KEY and CHAT_ID of your account. 

Guide to obtain Telegram bot [API_KEY](https://www.siteguarding.com/en/how-to-get-telegram-bot-api-token) <br>
Guide to obtain Telegram [chat_id](https://t.me/raw_data_bot)

### How to use this Tool !

There are 4 options that you can use it

1. Bind custom website
    * Bind to your custom website . Just Download or save the page into payload
      folder with the name ( index.html ).
    * Before using this option . Make sure to save your file to the /payload dir .
    * Remember . Save or rename your main .html file into **index.html** .

2. Create a New Site from Scratch
    * This will Download an site Only(index.html) file and serve it !
    * You can use it to serve simple site such as example.com etc..

3. Start and serve HTTP Server 
    * If you want to reuse the payload without anychanges . you can use this method .
    * This will automatically will start and fetch the file to localhost .

4. Help
    * Displays Help 

5. Exit
    * Exit the Program in an safe way.

### Demo Video

Hexor 3.0 Demo [Custom Binding]

Hexor 2.0 Demo

### Tunnel Usages

There will be 3 opensource tunneling options .

* Tunnel Mole
* Telebit
* Bore
* Tunnel Pyjam (manual)

You need to register only for Telebit. Its an free opensource reverse proxy. <br>

If you face any issues on Telebit Installations . Visit the site [Telebit](https://telebit.cloud/) and install it manually 

Follow the instructions Telebit Instructions while installing using ./build.sh.

### New Features Included

* Custom Binding on Websites . Save your site on /payload directory & build and bind with your site and start tunneling . 
* Accurate latitude longitude marker (Perfectly works on Mobile Devices). For PC's shows the location of ISP Providers.
* Server written in Rust for faster Req and response to work at High latency.
* Just an Click bait is enough to send the information .
* Clone any site by providing URLS to the site with `https://` or `http://` extension for 2nd Option . You can watch the Javahexor 2.0 Video to understand the module well !.

### Extensibility

* Add custom HTTPS Reverse Proxies by editing tunnel.rs .
* Edit new_locate.js file to add new functions.

### Next Update ? 

* Integrating all Social Media Sited into javahexor.
* Converting all js code into Rust.
* Adding some more opensource tunnels.


### Note 

*The location tracker will only works on https , not on http while tunneling.*

### Credits

Program created by [@smukx](https://twitter.com/Smukx07) . If you like this work please leave an star on this repo. so that i can motivate and do more nerdy works! 











