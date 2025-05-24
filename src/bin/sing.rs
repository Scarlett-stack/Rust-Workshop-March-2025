// +---------------------------------------------------------------------------+
// |                            Embassy March 2025                             |
// |                            Sing your own tune                             |
// +---------------------------------------------------------------------------+

#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_mar_2025::music::{Note, OCTAVE};
use embassy_rp::pwm::{self, Pwm, SetDutyCycle};
use embassy_time::{Duration, Timer};
use fixed::traits::ToFixed;
// Use the `panic_probe` crate to provided the panic handler and the
// `defmt_rtt` to import the runtime for defmt, to be able to use
// the print macros.
use {defmt_rtt as _, panic_probe as _};
// Use the logging macros provided by defmt.
use embassy_rp::pwm::Config as ConfigPwm; 
#[allow(unused)]
use defmt::*;


#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    // Get a handle to the RP's peripherals.
    let p = embassy_rp::init(Default::default());
    //let mut config: ConfigPwm = Default::default();

    // TODO: Configure the PWM pin.
    let mut buzzer_cfg : ConfigPwm = Default::default();
    buzzer_cfg.divider = PWM_DIV.to_fixed();
    let mut buzzer = pwm::Pwm::new_output_b(
        p.PWM_SLICE1,
        p.PIN_3,
        buzzer_cfg.clone()
    );

    const TEMPO: u64 = 100;
/// A whole note duration in milliseconds.
const WHOLE_NOTE: u64 = 4 * (60_000 / TEMPO);
/// The microcontroller clock frequency
const CLOCK_FREQ: u64 = 150_000_000;
/// PWM clock divider
const PWM_DIV: u64 = 64;

    for (note, length) in OCTAVE {
        // TODO: Compute the note's duration based on
        // the length variable.
        let base_len = length as u64;
        let duration = WHOLE_NOTE/base_len;
        
        match note {
            Some(note) => {
                let mut freq = note as u64;
                let mut top = (CLOCK_FREQ / (PWM_DIV * freq) ) as u16;
                buzzer_cfg.top = top;
                buzzer_cfg.compare_b = (top / 2u16);
               // buzzer.set_duty_cycle(top/2); reface complet cfg
               buzzer.set_config(&buzzer_cfg);
                // TODO: Configure the `top` and `compare_X` registers
                // based on the note's type and change the PWM's config.
                // Keep in mind that we are aiming for a 50% duty cycle.
                // "Play" the note for 90% of the duration, then insert
                Timer::after(Duration::from_millis(duration * 9/10)).await;
                buzzer.set_duty_cycle(0); //nu mai canta
                // a 10% pause before playing the next note.
                Timer::after(Duration::from_millis(duration /10)).await;
            },
            None => {
                // TODO: Just wait the whole duration.
                Timer::after(Duration::from_millis(duration)).await;
            }
        };
    }
}
