//! This example shows how to use SPI (Serial Peripheral Interface) in the RP2040 chip.
//! No specific hardware is specified in this example. If you connect pin 11 and 12 you should get the same data back.

#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::{
    gpio::Output,
    spi::{Config, Spi},
};
use embassy_time::{Duration, Instant, Ticker};
use embedded_graphics::{
    mono_font::{MonoTextStyle, iso_8859_14::FONT_7X14_BOLD},
    primitives::PrimitiveStyle,
};
use embedded_graphics::{prelude::*, primitives::Rectangle, text::Text};
use embedded_graphics_core::draw_target::DrawTarget;
use memory_lcd_spi::{MemoryLCD, displays::LPM009M360A, pixelcolor::Rgb111};
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    info!("Hello World!");

    let cs = Output::new(p.PIN_12, embassy_rp::gpio::Level::High);
    let mosi = p.PIN_11;
    let clk = p.PIN_10;
    let _miso = p.PIN_24;

    // EXTCOMIN is not connected now
    // let extcomin = p.PIN_8;
    // let desired_freq_hz = 140;
    // let clock_freq_hz = embassy_rp::clocks::clk_sys_freq();
    // let divider = 16u8;
    // let period = (clock_freq_hz / (desired_freq_hz * divider as u32)) as u16 - 1;
    // let mut c = embassy_rp::pwm::Config::default();
    // c.top = period;
    // c.divider = divider.into();

    // let mut pwm = Pwm::new_output_a(p.PWM_SLICE4, extcomin, c.clone());
    // pwm.set_duty_cycle(c.top / 2).unwrap();

    let _disp = Output::new(p.PIN_0, embassy_rp::gpio::Level::High);
    let _vdd = Output::new(p.PIN_1, embassy_rp::gpio::Level::High);
    let _vdd2 = Output::new(p.PIN_17, embassy_rp::gpio::Level::High);

    let mut spi_config = Config::default();
    // 2MHz
    spi_config.frequency = 1_000_000;
    let spi = Spi::new_txonly(p.SPI1, clk, mosi, p.DMA_CH0, spi_config);

    let mut display: MemoryLCD<LPM009M360A<Rgb111>, _, _> = MemoryLCD::new(spi, cs);

    display.set_rotation(memory_lcd_spi::framebuffer::Rotation::Deg180);

    display.clear(Rgb111::WHITE).unwrap();
    display.update().unwrap();
    // loop {}
    // drawing code with embedded-graphics

    let mut x = 0;
    let colors = [
        Rgb111::CYAN,
        Rgb111::RED,
        Rgb111::GREEN,
        Rgb111::MAGENTA,
        Rgb111::BLUE,
        Rgb111::YELLOW,
    ];
    let mut cnt = 0;
    let mut start = Instant::now();
    loop {
        let color = colors[x % colors.len()];
        Text::new(
            "Hello RMK!",
            Point::new(4, 82),
            MonoTextStyle::new(&FONT_7X14_BOLD, Rgb111::RED),
        )
        .draw(&mut *display)
        .unwrap();
        Text::new(
            "Hello RMK!",
            Point::new(4, 102),
            MonoTextStyle::new(&FONT_7X14_BOLD, Rgb111::BLACK),
        )
        .draw(&mut *display)
        .unwrap();
        Text::new(
            "Hello RMK!",
            Point::new(4, 122),
            MonoTextStyle::new(&FONT_7X14_BOLD, Rgb111::BLUE),
        )
        .draw(&mut *display)
        .unwrap();
        Text::new(
            "Hello RMK!",
            Point::new(4, 142),
            MonoTextStyle::new(&FONT_7X14_BOLD, Rgb111::GREEN),
        )
        .draw(&mut *display)
        .unwrap();
        Rectangle::new(Point::new(6, 6), Size::new(60, 60))
            .into_styled(PrimitiveStyle::with_fill(color))
            .draw(&mut *display)
            .unwrap();
        x += 1;
        if x >= colors.len() {
            x = 0;
        }
        // pwm.set_duty_cycle(c.top / 2).unwrap();
        display.update().unwrap();
        cnt += 1;
        if start.elapsed().as_millis() > 1000 {
            info!("{}Hz", cnt);
            cnt = 0;
            start = Instant::now();
        }

        embassy_time::Timer::after_millis(500).await;
        // info!("update {}", x);
    }
}
#[embassy_executor::task(pool_size = 2)]
async fn toggle_led(mut led: Output<'static>, delay: Duration) {
    let mut ticker = Ticker::every(delay);
    loop {
        led.toggle();
        ticker.next().await;
    }
}
