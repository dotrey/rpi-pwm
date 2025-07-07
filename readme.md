# Raspberry Pi PWM Controller
This is a simple controller for PWM on a Raspberry Pi.

Use environment variable `PWM_CHANNEL` to select the PWM channel you want to control (default: `0`).

Use environment variable `PWM_DUTY` (range `0.0` to `1.0`) to set the duty cycle for the selected PWM channel.

The controller will check `PWM_DUTY` every 15 seconds and update accordingly.

After changing `PWM_CHANNEL` you need to restart the controller for the change to become effective.