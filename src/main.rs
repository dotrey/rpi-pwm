use anyhow::Result;
use rppal::pwm::{Channel, Polarity, Pwm};
use std::{env, thread, time::Duration};

/// How often to sample the environment variable.
const POLL_INTERVAL: Duration = Duration::from_secs(15);

/// PWM output frequency in Hz.
const PWM_FREQUENCY_HZ: f64 = 25000.0;

/// Name of env vars to read.
const ENV_VAR_DUTY: &str = "PWM_DUTY";
const ENV_VAR_CHANNEL: &str = "PWM_CHANNEL";

fn main() -> Result<()> {
    // Determine which PWM channel to use (0 or 1)
    let channel = match env::var(ENV_VAR_CHANNEL)
        .as_deref()
        .unwrap_or("0")
    {
        "0" => Channel::Pwm0,
        "1" => Channel::Pwm1,
        other => {
            eprintln!(
                "Invalid {}, defaulting to 0 (got {:?})",
                ENV_VAR_CHANNEL, other
            );
            Channel::Pwm0
        }
    };

    // Initialize the chosen PWM channel at 0% duty cycle.
    let pwm = Pwm::with_frequency(channel, PWM_FREQUENCY_HZ, 0.0, Polarity::Normal, false)?;
    pwm.enable()?;

    println!(
        "Starting PWM controller on channel {:?}. \
         Reading {} (0.0–1.0) every 15 seconds.",
        channel, ENV_VAR_DUTY
    );

    loop {
        // Read and parse the duty cycle from ENV_VAR_DUTY.
        let duty = env::var(ENV_VAR_DUTY)
            .ok()
            .and_then(|s| s.parse::<f64>().ok())
            .map(|v| v.clamp(0.0, 1.0))
            .unwrap_or(0.0);

        pwm.set_duty_cycle(duty)?;
        println!("Set duty cycle to {:.1}%", duty * 100.0);

        thread::sleep(POLL_INTERVAL);
    }
}
