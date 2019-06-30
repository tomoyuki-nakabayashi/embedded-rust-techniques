#!/bin/sh

PW="raspberry"

expect -c "
set timeout 5
spawn env LANG=C /usr/bin/scp $1 pi@\[fc0f:1d0:bb8:4cbe:1093:bc77:c131:df0c\]:/home/pi/raspi
expect \"password:\"
send \"${PW}\n\"
expect \"$\"

spawn env LANG=C /usr/bin/ssh pi@fc0f:1d0:bb8:4cbe:1093:bc77:c131:df0c ./raspi
expect \"password:\"
send \"${PW}\n\"
expect \"$\"
exit 0
"
