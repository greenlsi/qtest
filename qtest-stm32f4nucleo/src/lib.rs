// stm32f4.rs

// This module provides support for the STM32F4 series of microcontrollers.

pub mod gpio;

use gpio::Gpio;

#[derive(Clone)]
// Crear una nueva estructura para encapsular las instancias específicas
pub struct Peripheral {
    gpio_a: Gpio,
    gpio_b: Gpio,
    gpio_c: Gpio,
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

impl Peripheral {
    // Ensure the new function is public
    pub fn new() -> Self {
        Peripheral {
            gpio_a: Gpio::new(0x40020000),
            gpio_b: Gpio::new(0x40020400),
            gpio_c: Gpio::new(0x40020800),
        }
    }
    // Use the macro to create the accessor functions
    create_gpio_accessors!(gpio_a, gpio_b, gpio_c);
}

// Instancias estáticas de GPIOs específicos usando lazy_static:
//lazy_static es útil para crear instancias globales de estructuras que necesitas
//compartir en varias partes de tu código, sin necesidad de inicializarlas de manera
//explícita en cada lugar donde las vayas a utilizar
//TO DO: Revisar si es necesario usar lazy_static
// lazy_static! {
//     pub static ref GPIO_EXT: GpioExt = GpioExt::new();
// }
