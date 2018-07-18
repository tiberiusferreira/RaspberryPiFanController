#! /bin/bash
# /etc/init.d/temperature_monitor

### BEGIN INIT INFO
# Provides:          tiberio
# Required-Start:    $remote_fs $syslog
# Required-Stop:     $remote_fs $syslog
# Default-Start:     2 3 4 5
# Default-Stop:      0 1 6
# Short-Description: Simple script to start the Temperature Monitoring program at boot
# Description:       A simple script to manage Temperature Monitoring which will start / stop it at boot / shutdown.
### END INIT INFO

# If you want a command to always run, put it here

# Carry out specific functions when asked to by the system
case "$1" in
  start)
    echo "Starting Temperature Monitoring"
    # run application you want to start
    sleep 35 && cd /home/pi/TemperatureMonitor && sudo -u pi env RUST_BACKTRACE=1 /home/pi/.cargo/bin/cargo +stable run &
    ;;
  stop)
    echo "Stopping Temperature Monitoring"
    # kill application you want to stop
    killall TemperatureMonitor
    ;;
  *)
    echo "Usage: {start|stop}"
    exit 1
    ;;
esac

exit 0