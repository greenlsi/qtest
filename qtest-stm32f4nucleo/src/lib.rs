// stm32f4.rs

// This module provides support for the STM32F4 series of microcontrollers.

pub mod gpio;

pub mod timer;

use gpio::Gpio;
use timer::Timer;

// Crear una nueva estructura para encapsular las instancias específicas
#[derive(Debug, Clone)]
pub struct Peripheral {
    gpio_a: Gpio,
    gpio_b: Gpio,
    gpio_c: Gpio,
    gpio_d: Gpio,
    gpio_e: Gpio,
    gpio_f: Gpio,
    gpio_g: Gpio,
    gpio_h: Gpio,
    timer2: Timer,
    timer5: Timer,
}

//macro para acceder a los registros de los GPIOs
macro_rules! create_gpio_accessors {
    ($($name:ident),*) => {
        $(
            pub fn $name(&self) -> &Gpio {
                &self.$name
            }
        )*
    };
}
macro_rules! create_timer_accessors {
    ($($name:ident),*) => {
        $(
            pub fn $name(&self) -> &Timer {
                &self.$name
            }
        )*
    };
}

//NECESARIO SI QUIERO IMPLEMENTAR LIFETIMES
// pub enum PeripheralType<'a> {
//     Gpio(&'a Gpio),
//     Timer(&'a Timer),
// }

impl Default for Peripheral {
    fn default() -> Self {
        Peripheral::new()
    }
}

impl Peripheral {
    // Ensure the new function is public
    pub fn new() -> Self {
        Peripheral {
            gpio_a: Gpio::new(0x40020000),
            gpio_b: Gpio::new(0x40020400),
            gpio_c: Gpio::new(0x40020800),
            gpio_d: Gpio::new(0x40020C00),
            gpio_e: Gpio::new(0x40021000),
            gpio_f: Gpio::new(0x40021400),
            gpio_g: Gpio::new(0x40021800),
            gpio_h: Gpio::new(0x40021C00),
            timer2: Timer::new(0x40000000), //asegurarse de que estas direcciones estén bien
            timer5: Timer::new(0x40000C00), //
        }
    }

    pub fn get_gpio(&self, name: &str) -> Option<&Gpio> {
        let normalized = name
            .to_lowercase()
            .chars()
            .filter(|c| c.is_alphanumeric()) // elimina _ - espacios, etc.
            .collect::<String>();

        match normalized.as_str() {
            "gpioa" => Some(&self.gpio_a),
            "gpiob" => Some(&self.gpio_b),
            "gpioc" => Some(&self.gpio_c),
            "gpiod" => Some(&self.gpio_d),
            "gpioe" => Some(&self.gpio_e),
            "gpiof" => Some(&self.gpio_f),
            "gpiog" => Some(&self.gpio_g),
            "gpioh" => Some(&self.gpio_h),
            _ => None,
        }
    }

    pub fn get_timer(&self, name: &str) -> Option<&Timer> {
        let normalized = name
            .to_lowercase()
            .chars()
            .filter(|c| c.is_alphanumeric())
            .collect::<String>();

        match normalized.as_str() {
            "timer2" => Some(&self.timer2),
            "timer5" => Some(&self.timer5),
            _ => None,
        }
    }

    //Opción de get con LIFETIMES
    // pub fn get(&self, name: &str) -> Option<PeripheralType> {
    //     match name {
    //         "gpio_a" => Some(PeripheralType::Gpio(&self.gpio_a)),
    //         "gpio_b" => Some(PeripheralType::Gpio(&self.gpio_b)),
    //         "gpio_c" => Some(PeripheralType::Gpio(&self.gpio_c)),
    //         "gpio_d" => Some(PeripheralType::Gpio(&self.gpio_d)),
    //         "gpio_e" => Some(PeripheralType::Gpio(&self.gpio_e)),
    //         "gpio_f" => Some(PeripheralType::Gpio(&self.gpio_f)),
    //         "gpio_g" => Some(PeripheralType::Gpio(&self.gpio_g)),
    //         "gpio_h" => Some(PeripheralType::Gpio(&self.gpio_h)),
    //         "timer2" => Some(PeripheralType::Timer(&self.timer2)),
    //         "timer5" => Some(PeripheralType::Timer(&self.timer5)),
    //         _ => None,
    //     }
    // }

    // Use the macro to create the accessor functions
    create_gpio_accessors!(gpio_a, gpio_b, gpio_c);
    create_timer_accessors!(timer2, timer5);
}

impl Default for Peripheral {
    fn default() -> Self {
        Self::new()
    }
}

// Instancias estáticas de GPIOs específicos usando lazy_static:
//lazy_static es útil para crear instancias globales de estructuras que necesitas
//compartir en varias partes de tu código, sin necesidad de inicializarlas de manera
//explícita en cada lugar donde las vayas a utilizar
//TO DO: Revisar si es necesario usar lazy_static
// lazy_static! {
//     pub static ref GPIO_EXT: GpioExt = GpioExt::new();
// }
