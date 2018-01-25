![alt tag](https://travis-ci.org/yenwel/irondragon.svg?branch=master)
# irondragon

![alt tag](https://upload.wikimedia.org/wikipedia/commons/2/2c/Zmei_Gorinich_%28colour_fixed%29.jpg)

Trying out a rust riddle microservice on my raspberryPi

## requirements
* serve random riddle
  * answer riddle
    * move body part or light led on rpi immediately
  * upvote/downvote riddle
* accept riddle
* move body part or light led of rpi force directed
* mesh netwerk (project byzantium)
* rugged to off the grid situations?

## sculpture
* drawings and sketches of dragon (creative commons)
* wireframe dragon sculpture
* add servo and leds to sculpture, connect to gpio
* proximity sensor?

![alt tag](https://raw.githubusercontent.com/yenwel/irondragon/master/korhoenstraat.jpg)

http://www.demorgen.be/fotografie/het-mooiste-steegje-van-gent-fec06aa8/

## architecture
* FE: 
  * reactjs (possibly compile to android app (react native) or https://phaser.io/ or https://threejs.org/
  * http://offlinefirst.org/ (progressive webapp)
  * https://auth0.com/blog/creating-offline-first-web-apps-with-service-workers/
* BE: rust on rpi (iron) + Sqlite 
  * wit.ai?
  * mesh networking via project byzantium
  * actor or channel rust libraries for concurrency
  * push notifications or websockets to update UI based on events in actors

![alt tag](https://raw.githubusercontent.com/yenwel/irondragon/master/dragonschema_bb.png)


## links
* https://github.com/iron/staticfile
* https://www.reddit.com/r/rust/comments/31zc9m/new_to_rust_how_well_is_sqlite_supported/
* http://www.instructables.com/id/Install-Project-Byzantium-Linux-to-a-Raspberry-Pi-/
* https://hackaday.com/tag/olsr/
* https://github.com/Byzantium/ByzPi/wiki/Install-Script
* wit.ai (voice or text control)?
* https://github.com/gamazeps/RobotS
* https://github.com/kolloch/actors
* https://doc.rust-lang.org/std/sync/mpsc/fn.channel.html
* http://burntsushi.net/rustdoc/chan/
* http://stackoverflow.com/questions/32203610/how-to-integrate-uml-diagrams-into-gitlab-or-github
* https://github.com/inre/cupi
* https://github.com/rust-embedded/rust-sysfs-gpio
* https://github.com/Nercury/di-rs
* https://github.com/rustless/rustless
* https://github.com/mikeycgto/esper
* https://github.com/cyderize/rust-websocket/blob/1fab3a438a5f11e97760acddfcfd8c9504094987/examples/hyper.rs
* http://stackoverflow.com/questions/19233529/run-bash-script-as-daemon
* https://github.com/servo/rust-mozjs
* https://learn.adafruit.com/adafruits-raspberry-pi-lesson-8-using-a-servo-motor/software
* https://github.com/Ogeon/rust-wiringpi
* https://github.com/rust-embedded/rust-sysfs-pwm
* https://github.com/jonathandturner/rhai
* http://www.jumpnowtek.com/rpi/Using-the-Raspberry-Pi-Hardware-PWM-timers.html
* http://elinux.org/RPi_BCM2835_GPIOs#GPIO18
* https://auth0.com/blog/build-an-app-with-vuejs/
* https://alligator.io/vuejs/rest-api-axios/
* https://stackoverflow.com/questions/35062852/npm-vs-bower-vs-browserify-vs-gulp-vs-grunt-vs-webpack
* http://fountainjs.io
* https://www.npmjs.com/package/generator-elm-spa
* https://blog.sicara.com/a-progressive-web-application-with-vue-js-webpack-material-design-part-1-c243e2e6e402
* https://github.com/housleyjk/ws-rs
* https://github.com/Florob/rust-xmpp
* https://sipjs.com/
* https://www.jsxc.org/
* https://community.letsencrypt.org/t/certificates-for-hosts-on-private-networks/174/30
* http://peerjs.com/
* http://www.ericerfanian.com/mobile-mesh-networks-with-the-raspberry-pi-part-4/
* https://nbaksalyar.github.io/2015/07/10/writing-chat-in-rust.html
* https://serverfault.com/questions/171678/nginx-config-front-end-reverse-proxy-to-another-port
* https://github.com/mpatacchiola/deepgaze
* https://raspberrypi.stackexchange.com/questions/38902/camera-input-from-phone-to-raspberry-pi
* http://teknosrc.com/raspberry-pi-usb-tethering-how-to-make-internet-work-in-raspberry-pi-by-usb-tethering/
* https://howto-android.com/setting-up-hotspot-or-tethering-on-acer-liquid-z520/


On Windows in Powershell: $env:RUST_BACKTRACE = 1
