use crate::register::Register;

/// GPIO (General Purpose Input/Output) structure represents a GPIO peripheral.
#[derive(Debug)]
pub struct Gpio {
    moder: Register<u32>,
    otyper: Register<u32>,
    ospeedr: Register<u32>,
    pupdr: Register<u32>,
    idr: Register<u32>,
    odr: Register<u32>,
    bsrr: Register<u32>,
    lckr: Register<u32>,
    afrl: Register<u32>,
    afrh: Register<u32>,
}

macro_rules! create_register_accessors {
    ($($name:ident,$reg:ident),*) => {
        $(
            pub fn $reg(&self) -> &Register<u32> {
                &self.$reg
            }
            pub fn $name(&mut self) -> &mut Register<u32> {
                &mut self.$reg
            }
        )*
    };
}

/// GPIO (General Purpose Input/Output) structure represents a GPIO peripheral.
///
/// This structure provides access to GPIO registers:  `MODER`, `OTYPER`, `OSPEEDR`, `PUPDR`, `IDR`, `ODR`, `BSRR`, `LCKR`, `AFRH`, and `AFRL`.
/// Each register is represented by a `Register<u32>` type.
///
/// # Example
///
/// ```rust
/// let gpio = Gpio::new(0x40020000);
/// let moder = gpio.moder();
/// let otyper = gpio.otyper();
/// ```
///
/// # Registers
///
/// - `MODER`: GPIO port mode register
/// - `OTYPER`: GPIO port output type register
/// - `OSPEEDR`: GPIO port output speed register
/// - `PUPDR`: GPIO port pull-up/pull-down register
/// - `IDR`: GPIO port input data register
/// - `ODR`: GPIO port output data register
/// - `BSRR`: GPIO port bit set/reset register
/// - `LCKR`: GPIO port configuration lock register
/// - `AFRH`: GPIO alternate function high register
/// - `AFRL`: GPIO alternate function low register
///
/// # Methods
///
/// - `new(address: usize) -> Self`: Creates a new `Gpio` instance with the specified base address.
/// - Getter methods to access each register (e.g., `moder()`, `otyper()`, etc.).
/// - Mutable getter methods to access each register mutably (e.g., `moder_mut()`, `otyper_mut()`, etc.).
///
/// # Note
///
/// The `create_register_accessors!` and `create_register_accessors_mut!` macros are used to generate the getter and mutable getter methods for the registers.
impl Gpio {
    pub fn new(address: usize) -> Self {
        Gpio {
            moder: Register::<u32>::new("MODER", address),
            otyper: Register::<u32>::new("OTYPER", address + 0x04),
            ospeedr: Register::<u32>::new("OSPEEDR", address + 0x08),
            pupdr: Register::<u32>::new("OTYPER", address + 0x0C),
            idr: Register::<u32>::new("IDR", address + 0x10),
            odr: Register::<u32>::new("ODR", address + 0x14),
            bsrr: Register::<u32>::new("BSRR", address + 0x18),
            lckr: Register::<u32>::new("LCKR", address + 0x1C),
            afrh: Register::<u32>::new("AFRH", address + 0x20),
            afrl: Register::<u32>::new("AFRL", address + 0x24),
        }
    }

    create_register_accessors!(
        moder_mut,
        moder,
        otyper_mut,
        otyper,
        ospeedr_mut,
        ospeedr,
        pupdr_mut,
        pupdr,
        idr_mut,
        idr,
        odr_mut,
        odr,
        bsrr_mut,
        bsrr,
        lckr_mut,
        lckr,
        afrl_mut,
        afrl,
        afrh_mut,
        afrh
    );
}
