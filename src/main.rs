use rppal::gpio::{Gpio, OutputPin};
use rppal::pwm::{self, Pwm};
use std::thread::sleep;
use std::time::Duration;

struct Motor {
    gpio_pin: Pwm,
}

impl Motor {
    pub fn new(pin: rppal::pwm::Channel) -> Motor {
        let gpio_pin = Pwm::with_period(
            pin,
            Duration::from_millis(20),
            Self::pulse_with_from_angle(0.0),
            rppal::pwm::Polarity::Normal,
            true,
        )
        .unwrap();
        Motor { gpio_pin: gpio_pin }
    }

    fn pulse_with_from_angle(angle: f64) -> Duration {
        let duty = angle * 2.0 / 180.0 + 0.5;

        Duration::from_nanos((duty * 1000000.0).round() as u64)
    }

    pub fn set_angle(&mut self, angle: f64) {
        self.gpio_pin
            .set_pulse_width(Self::pulse_with_from_angle(angle))
            .unwrap();
    }

    pub fn turn_off(self) {
        self.gpio_pin.disable().unwrap();
    }
}

struct Led {
    gpio_pin: OutputPin,
}

impl Led {
    pub fn new(gpio: &Gpio, gpio_pin: u8) -> Led {
        Led {
            gpio_pin: gpio.get(gpio_pin).unwrap().into_output(),
        }
    }

    pub fn turn_on(&mut self) {
        self.gpio_pin.set_high();
    }

    pub fn turn_off(&mut self) {
        self.gpio_pin.set_low();
    }
}

fn round_around(
    motor_v: &mut Motor,
    motor_h: &mut Motor,
    center_setpoint_v: f64,
    center_setpoint_h: f64,
    diameter: f64,
    nb_step: u32,
    pause: f64,
) {
    let step_angle = 2.0 * std::f64::consts::PI / (nb_step as f64);
    for t in 0..nb_step {
        let setpoint_v = center_setpoint_v + diameter * ((t as f64) * step_angle).cos();
        let setpoint_h = center_setpoint_h + diameter * ((t as f64) * step_angle).sin();
        motor_v.set_angle(setpoint_v);
        motor_h.set_angle(setpoint_h);
        sleep(Duration::from_secs_f64(pause));
    }
}

fn main() {
    let gpio = Gpio::new().unwrap();
    let mut led = Led::new(&gpio, 23);
    let mut motor_v = Motor::new(pwm::Channel::Pwm0);
    let mut motor_h = Motor::new(pwm::Channel::Pwm1);
    sleep(Duration::from_secs(1));
    led.turn_on();

    loop {
        round_around(&mut motor_v, &mut motor_h, 110.0, 75.0, 3.0, 40, 0.2);
    }

    // let cin = std::io::stdin();
    // loop {
    //     let mut s = String::new();
    //     cin.read_line(&mut s).unwrap();
    //     if s == "q" {
    //         break;
    //     }
    //     let values = s
    //         .split_whitespace()
    //         .map(|x| x.parse::<f64>())
    //         .collect::<Result<Vec<f64>, _>>()
    //         .unwrap();
    //     assert!(values.len() == 2);

    //     let var1 = values[0];
    //     let var2 = values[1];
    //     println!("var1: {}, var2: {}", var1, var2);
    //     motor_v.set_angle(var1);
    //     motor_h.set_angle(var2);
    // }

    motor_v.turn_off();
    motor_h.turn_off();

    led.turn_off();
}
