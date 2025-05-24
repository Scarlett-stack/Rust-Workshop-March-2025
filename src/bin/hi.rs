// +---------------------------------------------------------------------------+
// |                            Embassy March 2025                             |
// |                            Hi, I am new here!                             |
// +---------------------------------------------------------------------------+

#![no_std]
#![no_main]

use embassy_executor::Spawner;
// Use the `panic_probe` crate to provided the panic handler and the 
// `defmt_rtt` to import the runtime for defmt, to be able to use
// the print macros.
use {defmt_rtt as _, panic_probe as _};
// Use the logging macros provided by defmt.
use embassy_rp::gpio::{Level, Output};
use embassy_time::{Duration, Timer};
use embassy_rp::gpio::{Input, Pull};
use embassy_futures::join::join;
#[allow(unused)]
use defmt::*;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    // Get a handle to the RP's peripherals.
   // let _p = embassy_rp::init(Default::default());
   let p = embassy_rp::init(Default::default());

    let mut sw4 = Input::new(p.PIN_2, Pull::Up);
    let mut sw5 = Input::new(p.PIN_3, Pull::Up);
    let mut sw6 = Input::new(p.PIN_4, Pull::Up);
   // let mut sw7 = Input::new(p.PIN_5, Pull::Up);
    loop {
        // if !sw4.is_high() {
        //     info!("lalala");
        // }
        // sw4.wait_for_falling_edge().await;
        // info!("lalala");
        // sw4.wait_for_rising_edge().await;
       
       //ex1
       /*  sw4.wait_for_falling_edge().await; //conteaza unde pui asta!!!
      
       // sw7.wait_for_falling_edge().await;

        if sw4.is_low() {
            info!("LALA");
        }
        sw5.wait_for_falling_edge().await;
     if sw5.is_low() {
            info!("SW 5 LALAL");
        }
        sw6.wait_for_falling_edge().await;
         if sw6.is_low() {
            info!("SW6 LALA");
        }
        */
        join(sw4.wait_for_falling_edge(), sw5.wait_for_falling_edge()).await;
        if sw4.is_low() && sw5.is_low() {
            info!("sw4 si sw5 ambele apasate");
        }


        Timer::after(Duration::from_millis(100)).await;
    }
}
