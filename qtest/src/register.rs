use crate::{session::Session, Response};
use std::{io, marker::PhantomData};

/// Proxy to access to a generic hardware register with QTest.
///
/// `T` represents the data type of the register (`u8`, `u16`, `u32`, `u64`).
#[derive(Debug, Clone)]
pub struct Register<T> {
    /// The name of the register.
    name: String,
    /// The memory address of the register.
    address: usize,
    /// Type marker, ensuring the struct is generic over `T`.
    _size_marker: PhantomData<T>,
}

impl<T> Register<T> {
    /// Creates a new `Register` instance.
    ///
    /// # Parameters
    ///
    /// - `name`: A string slice representing the name of the register.
    /// - `address`: The memory-mapped address of the register.
    ///
    /// # Returns
    ///
    /// Returns a new instance of `Register`.
    ///
    /// # Example
    ///    
    /// ```
    /// let reg = Register::new("reg1", 0x1000);
    /// ```
    ///
    pub fn new<S: ToString>(name: S, address: usize) -> Self {
        Register {
            name: name.to_string(),
            address,
            _size_marker: PhantomData,
        }
    }

    /// Returns the name of the register.
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// Returns the address of the register.
    pub fn get_address(&self) -> usize {
        self.address
    }
}

macro_rules! impl_register {
    ($($type:ty => $read_fn:ident, $write_fn:ident);* $(;)?) => {
        $(
            impl Register<$type> {
                /// Reads a value from the register asynchronously.
                pub async fn read(&self, session: &mut Session) -> io::Result<$type> {
                    session.$read_fn(self.address).await
                }

                /// Writes a value to the register asynchronously.
                ///
                /// # Safety
                ///
                /// This function is `unsafe` because it directly accesses and modifies hardware registers,
                /// which can have side effects on the system if used improperly.
                pub async unsafe fn write(
                    &mut self,
                    value: $type,
                    session: &mut Session,
                ) -> io::Result<Response> {
                    session.$write_fn(self.address, value).await
                }
            }
        )*
    };
}

impl_register! {
    u8 => readb, writeb;
    u16 => readw, writew;
    u32 => readl, writel;
    u64 => readq, writeq;
}

#[macro_export]
macro_rules! register {
    ($($name:ident, $type:ty);*) => {
        $(
            #[repr(transparent)]
            #[derive(Debug, Clone)]
            pub struct $name {
                register: $crate::register::Register<$type>,
            }
            impl $name {
                pub fn new(address: usize) -> Self {
                    Self {
                        register: $crate::register::Register::new(stringify!($name), address),
                    }
                }
            }
            impl std::ops::Deref for $name {
                type Target = $crate::register::Register<$type>;
                fn deref(&self) -> &Self::Target {
                    &self.register
                }
            }
            impl std::ops::DerefMut for $name {
                fn deref_mut(&mut self) -> &mut Self::Target {
                    &mut self.register
                }
            }
        )*
    };
}
