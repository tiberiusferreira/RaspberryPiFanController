#! /bin/bash
sudo cp ./temperature_monitor.sh /etc/init.d/temperature_monitor
sudo chmod +x /etc/init.d/temperature_monitor
sudo update-rc.d temperature_monitor defaults
sudo update-rc.d temperature_monitor enable