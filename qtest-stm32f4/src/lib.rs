pub mod gpio;
pub mod timer;

use gpio::Gpio;
use qtest::{
    utils::{Peripheral, SoC},
    Irq, IrqState,
};
use timer::Timer;

#[cfg(feature = "qtest-web")]
mod web;

// Crear una nueva estructura para encapsular las instancias específicas
#[derive(Debug, Clone)]
pub struct Peripherals {}

impl Peripherals {
    // Ensure the new function is public
    pub fn new() -> Self {
        Peripherals {}
    }

    pub fn gpio(&self, gpio_id: &str) -> Option<Gpio> {
        let normalized = gpio_id
            .to_lowercase()
            .chars()
            .filter(|c| c.is_alphanumeric()) // removes _ - spaces, etc.
            .collect::<String>();

        match normalized.as_str() {
            "gpioa" => Some(Gpio::A),
            "gpiob" => Some(Gpio::B),
            "gpioc" => Some(Gpio::C),
            "gpiod" => Some(Gpio::D),
            "gpioe" => Some(Gpio::E),
            "gpiof" => Some(Gpio::F),
            "gpiog" => Some(Gpio::G),
            "gpioh" => Some(Gpio::H),
            _ => None,
        }
    }

    pub fn get_timer(&self, name: &str) -> Option<Timer> {
        let normalized = name
            .to_lowercase()
            .chars()
            .filter(|c| c.is_alphanumeric())
            .collect::<String>();

        match normalized.as_str() {
            "tim2" => Some(Timer::Tim2),
            "tim5" => Some(Timer::Tim5),
            _ => None,
        }
    }
}

impl Default for Peripherals {
    fn default() -> Self {
        Self::new()
    }
}

impl SoC for Peripherals {
    fn name(&self) -> &str {
        "stm32f4"
    }

    fn get_peripheral(&self, name: &str) -> Option<Box<dyn Peripheral>> {
        if let Some(gpio) = self.gpio(name) {
            Some(Box::new(gpio))
        } else if let Some(timer) = self.get_timer(name) {
            Some(Box::new(timer))
        } else {
            None
        }
    }

    fn input_irq_to_peripheral(&self, irq: Irq) -> Option<Box<dyn Peripheral>> {
        match irq.state {
            IrqState::Raise => None, // We ignore rising edge IRQs, as peripherals provoke pulses Raise-Lower
            IrqState::Lower => match irq.line {
                // GPIOs
                0..=7 => {
                    let gpio = match irq.line {
                        0 => Gpio::A,
                        1 => Gpio::B,
                        2 => Gpio::C,
                        3 => Gpio::D,
                        4 => Gpio::E,
                        5 => Gpio::F,
                        6 => Gpio::G,
                        7 => Gpio::H,
                        _ => unreachable!(),
                    };
                    Some(Box::new(gpio))
                }
                // Currently, we only map GPIOs; other peripherals can be added here
                _ => None,
            },
        }
    }
}
