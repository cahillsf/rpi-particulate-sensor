## Putting Together an Air Quality Sensor

#raspberrypi4 #sensirion #breathe #learn

starting point + inspiration: https://www.hackster.io/david-gherghita/air-quality-monitor-using-raspberry-pi-4-sps30-and-azure-03cb42#code

Fun one to walk through, appreciate your work!  I basically stepped through the instructions exactly as they read, took some notes and meandered a bit along the way.  Took up this project because I a raspberry pi laying around that I had bootstrapped as a minimal desktop then left gathering dust for a few years.  The idea is to learn something new, get a bit more hands on.  Luckily I did up learning a lot about embedded systems, electrical engineering, and doing some coding that I wouldn't do at my job.  Definitely still learning, we'll see where it goes.  Overall has been very fun, highly recommend stepping outside of your comfort zone.

Also writing this doc from scratch so I don't forgot how to write and type sentences because of easy it is for LLMs to generate tons of content quickly.  Feels a bit slow but that's kind of the point (not a dig at AI at all, love the stuff).

## Build the OS

I [forked and updated](https://github.com/cahillsf/meta-mylayer/tree/walnascar) the layer from the hackster post to make it compatible with the most recent (at time of writing) [Yocto](https://www.yoctoproject.org/) release `wasnascar`.  To test and build the OS it I used two separate envs, a local one for quick development and then a chunkier machine on AWS to run the build.  Details linked below, definitely did not "overoptimize" here so I'm sure there are improvements that could be made.


### Vagrantbox

[config](../build/local/)

Mounts 


### AWS

[config](../build/clouds/)