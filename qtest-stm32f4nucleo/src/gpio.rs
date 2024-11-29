pub mod registers;
use registers::{Afrh, Afrl, Bsrr, Idr, Lckr, Moder, Odr, Ospeedr, Otyper, Pupdr, RegisterOps};

/// GPIO (General Purpose Input/Output) structure represents a GPIO peripheral.
#[derive(Debug, Clone)]
pub struct Gpio {
    moder: Moder,
    otyper: Otyper,
    ospeedr: Ospeedr,
    pupdr: Pupdr,
    idr: Idr,
    odr: Odr,
    bsrr: Bsrr,
    lckr: Lckr,
    afrl: Afrl,
    afrh: Afrh,
}

macro_rules! create_register_accessors {
    ($($name:ident, $reg:ident, $type:ty),*) => {
        $(
            pub fn $reg(&self) -> &$type {
                &self.$reg
            }
            pub fn $name(&mut self) -> &mut $type {
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
            moder: Moder::new(address),
            otyper: Otyper::new(address + 0x04),
            ospeedr: Ospeedr::new(address + 0x08),
            pupdr: Pupdr::new(address + 0x0C),
            idr: Idr::new(address + 0x10),
            odr: Odr::new(address + 0x14),
            bsrr: Bsrr::new(address + 0x18),
            lckr: Lckr::new(address + 0x1C),
            afrl: Afrl::new(address + 0x20),
            afrh: Afrh::new(address + 0x24),
        }
    }

    // get_from_name que devuelve el tipo específico
    pub fn get(&self, name: &str) -> Option<&dyn RegisterOps> {
        match name {
            "MODER" => Some(&self.moder),
            "OTYPER" => Some(&self.otyper),
            "OSPEEDR" => Some(&self.ospeedr),
            "PUPDR" => Some(&self.pupdr),
            "IDR" => Some(&self.idr),
            "ODR" => Some(&self.odr),
            "BSRR" => Some(&self.bsrr),
            "LCKR" => Some(&self.lckr),
            "AFRL" => Some(&self.afrl),
            "AFRH" => Some(&self.afrh),
            _ => None,
        }
    }

    create_register_accessors!(
        moder_mut,
        moder,
        Moder,
        otyper_mut,
        otyper,
        Otyper,
        ospeedr_mut,
        ospeedr,
        Ospeedr,
        pupdr_mut,
        pupdr,
        Pupdr,
        idr_mut,
        idr,
        Idr,
        odr_mut,
        odr,
        Odr,
        bsrr_mut,
        bsrr,
        Bsrr,
        lckr_mut,
        lckr,
        Lckr,
        afrl_mut,
        afrl,
        Afrl,
        afrh_mut,
        afrh,
        Afrh
    );
}
