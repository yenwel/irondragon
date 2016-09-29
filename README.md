![alt tag](https://travis-ci.org/yenwel/irondragon.svg?branch=master)
# irondragon
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

## architecture
* FE: reactjs (possibly compile to android app (react native)
* BE: rust on rpi (iron) + Sqlite 
 * wit.ai?
 * mesh networking via project byzantium
 * actor or channel rust libraries for concurrency

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
