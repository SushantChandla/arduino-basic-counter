#![no_std]
#![no_main]

use arduino_hal::{
    hal::port::{PB1, PB2, PD2, PD3, PD4, PD6, PD7},
    port::{mode::Output, Pin},
    Pins,
};
use panic_halt as _;

use avr_device::interrupt;
use core::cell::RefCell;

type Console = arduino_hal::hal::usart::Usart0<arduino_hal::DefaultClock>;
static CONSOLE: interrupt::Mutex<RefCell<Option<Console>>> =
    interrupt::Mutex::new(RefCell::new(None));

macro_rules! println {
    ($($t:tt)*) => {
        interrupt::free(
            |cs| {
                if let Some(console) = CONSOLE.borrow(cs).borrow_mut().as_mut() {
                    let _ = ufmt::uwriteln!(console, $($t)*);
                }
            },
        )
    };
}

fn put_console(console: Console) {
    interrupt::free(|cs| {
        *CONSOLE.borrow(cs).borrow_mut() = Some(console);
    })
}

struct NumberDisplay {
    a: Pin<Output, PD7>,
    b: Pin<Output, PD6>,
    c: Pin<Output, PD4>,
    d: Pin<Output, PD2>,
    e: Pin<Output, PD3>,
    f: Pin<Output, PB1>,
    g: Pin<Output, PB2>,
}

impl NumberDisplay {
    fn draw_zero(&mut self) {
        self.a.set_high();
        self.b.set_high();
        self.c.set_high();
        self.d.set_high();
        self.e.set_high();
        self.f.set_high();

        self.g.set_low();
    }

    fn draw_one(&mut self) {
        self.b.set_high();
        self.c.set_high();

        self.a.set_low();
        self.d.set_low();
        self.e.set_low();
        self.f.set_low();
        self.g.set_low();
    }

    fn draw_two(&mut self) {
        self.b.set_high();
        self.a.set_high();
        self.g.set_high();
        self.e.set_high();
        self.d.set_high();

        self.c.set_low();
        self.f.set_low();
    }

    fn draw_three(&mut self) {
        self.b.set_high();
        self.a.set_high();
        self.g.set_high();
        self.c.set_high();
        self.d.set_high();

        self.e.set_low();
        self.f.set_low();
    }

    fn draw_four(&mut self) {
        self.b.set_high();
        self.f.set_high();
        self.g.set_high();
        self.c.set_high();

        self.a.set_low();
        self.d.set_low();
        self.e.set_low();
    }

    fn draw_five(&mut self) {
        self.a.set_high();
        self.f.set_high();
        self.g.set_high();
        self.c.set_high();
        self.d.set_high();

        self.b.set_low();
        self.e.set_low();
    }

    fn draw_six(&mut self) {
        self.g.set_high();
        self.c.set_high();
        self.f.set_high();
        self.e.set_high();
        self.d.set_high();

        self.a.set_low();
        self.b.set_low();
    }

    fn draw_seven(&mut self) {
        self.b.set_high();
        self.a.set_high();
        self.c.set_high();

        self.d.set_low();
        self.e.set_low();
        self.f.set_low();
        self.g.set_low();
    }

    fn draw_eight(&mut self) {
        self.b.set_high();
        self.a.set_high();
        self.g.set_high();
        self.c.set_high();
        self.f.set_high();
        self.e.set_high();
        self.d.set_high();
    }

    fn draw_nine(&mut self) {
        self.b.set_high();
        self.a.set_high();
        self.g.set_high();
        self.c.set_high();
        self.f.set_high();

        self.d.set_low();
        self.e.set_low();
    }

    fn draw_number(&mut self, num: i8) {
        match num {
            0 => self.draw_zero(),
            1 => self.draw_one(),
            2 => self.draw_two(),
            3 => self.draw_three(),
            4 => self.draw_four(),
            5 => self.draw_five(),
            6 => self.draw_six(),
            7 => self.draw_seven(),
            8 => self.draw_eight(),
            9 => self.draw_nine(),
            _ => println!("Unimplemented"),
        }
    }
}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins: Pins = arduino_hal::pins!(dp);
    let serial = arduino_hal::default_serial!(dp, pins, 57600);
    put_console(serial);

    let mut number_display = NumberDisplay {
        a: pins.d7.into_output(),
        b: pins.d6.into_output(),
        c: pins.d4.into_output(),
        d: pins.d2.into_output(),
        e: pins.d3.into_output(),
        f: pins.d9.into_output(),
        g: pins.d10.into_output(),
    };

    let mut i = 0;
    loop {
        println!("Displaying number: {}", i);
        number_display.draw_number(i);
        arduino_hal::delay_ms(1000);
        i += 1;
        if i == 10 {
            i = 0;
        }
    }
}
