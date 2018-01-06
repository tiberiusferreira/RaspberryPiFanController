# RaspberryPiFanController
A simple Raspberry Fan Controller written in RUST

It turns on the fan when the Raspberry Pi reaches 65 C and turns off when it reaches 50 C.

The temperature is measured using the Linux command: 
/opt/vc/bin/vcgencmd measure_temp and filtering it using egrep "[0-9.]{4,}" -o:

/opt/vc/bin/vcgencmd measure_temp | egrep "[0-9.]{4,}" -o 

It turns on and off the fan using a GPIO pin (number 21). 

This GPIO pin is connected to a circuit which uses the 5V Raspberry pin to power the fan.



