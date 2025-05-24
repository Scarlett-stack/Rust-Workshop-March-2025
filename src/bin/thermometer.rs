// +---------------------------------------------------------------------------+
// |                            Embassy March 2025                             |
// |                            My own thermometer!                            |
// +---------------------------------------------------------------------------+

#![no_std]
#![no_main]

// use embassy_executor::Spawner;
// // use embassy_mar_2025::bmp280::{self, Control};
// use bmp280_ehal::{self, BMP280, Control, Oversampling, PowerMode};
// // use embassy_rp::i2c::I2c;
// use embassy_time::Timer;
// // Use the `panic_probe` crate to provided the panic handler and the 
// // `defmt_rtt` to import the runtime for defmt, to be able to use
// // the print macros.
// // use embassy_rp::{adc::InterruptHandler, bind_interrupts};
// use embassy_rp::pwm::Config as ConfigPwm; 
// use embassy_rp::pwm::{self, Pwm, SetDutyCycle};
// use embassy_rp::peripherals::I2C1;
// use bmp280_ehal;
// use embedded_hal_async::i2c::I2c;
// use embassy_rp::i2c::{self, Config, InterruptHandler};
// // You can import here and alias the handlers.


// use {defmt_rtt as _, panic_probe as _};
// // Use the logging macros provided by defmt.
// #[allow(unused)]
// use defmt::*;

// embassy_rp::bind_interrupts!(struct Irqs {
//     // Do not forget to bind the I2C peripheral interrupt to its handler
//     I2C1_IRQ => InterruptHandler<I2C1>;
// });
// // bind_interrupts!(
// //     pub(super) struct Irqs {
// //         // Here you add all you bindings following the example from the module's top.
// //         // You can see all interrupt ids defined here: https://datasheets.raspberrypi.com/rp2350/rp2350-datasheet.pdf
// //         // in the "3.2. Interrupts" section.
// //         ADC_IRQ_FIFO => InterruptHandler;
// //     }
// // );

use bmp280_ehal::{self, BMP280, Control, Oversampling, PowerMode};
use core::array;
use defmt::{error, info};
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_rp::bind_interrupts;
use embassy_rp::i2c::{self, Config, InterruptHandler};
use embassy_rp::peripherals::I2C1;
use embassy_rp::pwm::{Config as ConfigPwm, Pwm};
use embassy_time::Timer;
use embedded_hal_async::i2c::I2c;
use fixed::traits::ToFixed;
use panic_probe as _;

bind_interrupts!(struct Irqs {
    I2C1_IRQ => InterruptHandler<I2C1>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    // Get a handle to the RP's peripherals.
    let p = embassy_rp::init(Default::default());
    

    // let mut rg_cfg : ConfigPwm = Default::default();
    // rg_cfg.top =  0x9088;
    // let mut rg_pwm = Pwm::new_output_ab(
    //     p.PWM_SLICE0,
    //     p.PIN_0, //o sa am ab si pini 0 si 1 combined ca asa rezolva embassy chestia asta 
    //     p.PIN_1,
    //     rg_cfg.clone() //top o sa fie acelasi pt aceleasi 
    // );
    let mut r_cfg : ConfigPwm = Default::default();
    
    let mut r_pwm = Pwm::new_output_b(
        p.PWM_SLICE1,
        p.PIN_3,
        r_cfg.clone()
    );
    let mut g_cfg : ConfigPwm = Default::default();
    
    let mut g_pwm = Pwm::new_output_a(
        p.PWM_SLICE2,
        p.PIN_4,
        g_cfg.clone()
    );

    let mut b_cfg : ConfigPwm = Default::default();
    b_cfg.top =  0x9088;
    let mut b_pwm = Pwm::new_output_a(
        p.PWM_SLICE3,
        p.PIN_6,
        b_cfg.clone()
    );

    let sda = p.PIN_14;
    let scl = p.PIN_15;


    let i2c = i2c::I2c::new_blocking(p.I2C1, scl, sda, Config::default());
    let mut bmp = bmp280_ehal::BMP280::new(i2c).unwrap();

    let mut color = 0;

    bmp.reset();
    bmp.set_control(Control {
        osrs_t: Oversampling::x2,
        osrs_p: Oversampling::x2,
        mode: PowerMode::Normal,
    });

    loop {
        info!("{}", bmp.temp());
        match color {
            0 => {
                r_cfg.compare_b = 0;
                r_pwm.set_config(&r_cfg);
                b_cfg.compare_a = b_cfg.top;  //oprit
               b_pwm.set_config(&b_cfg);
            

            }
            1 => {
                r_cfg.compare_b = r_cfg.top;
                r_pwm.set_config(&r_cfg);
                b_cfg.compare_a = 0;
               b_pwm.set_config(&b_cfg);
                
            }
            // 2 => {
            //     r_cfg.compare_b = 0;
            //     r_pwm.set_config(&r_cfg);
            //     b_cfg.compare_a = b_cfg.top;  //oprit
            //    b_pwm.set_config(&b_cfg);
            //     b_pwm.set_config(&b_cfg);
            // }
            _ => { 
                r_cfg.compare_b = 0;
                r_pwm.set_config(&r_cfg);
                b_cfg.compare_a = 0;
               b_pwm.set_config(&b_cfg);
               g_cfg.compare_a = 0;
               g_pwm.set_config(&g_cfg);
               
            }

        }
        color = (color + 1) % 2;
        Timer::after_millis(100).await
    }
}
