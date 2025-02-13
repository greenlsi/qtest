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

impl Register<u8> {
    /// Reads an `u8` value from the register asynchronously.
    pub async fn read(&self, session: &mut Session) -> io::Result<u8> {
        session.readb(self.address).await
    }

    /// Writes a `u8` value to the register asynchronously.
    ///
    /// # Safety
    ///
    /// This function is `unsafe` because it directly accesses and modifies hardware registers,
    /// which can have side effects on the system if used improperly.
    pub async unsafe fn write(&mut self, value: u8, session: &mut Session) -> io::Result<Response> {
        session.writeb(self.address, value).await
    }
}

impl Register<u16> {
    /// Reads a `u16` value from the register asynchronously.
    pub async fn read(&self, session: &mut Session) -> io::Result<u16> {
        session.readw(self.address).await
    }

    /// Writes a `u16` value to the register asynchronously.
    ///
    /// # Safety
    ///
    /// This function is `unsafe` because it directly accesses and modifies hardware registers,
    /// which can have side effects on the system if used improperly.
    pub async unsafe fn write(
        &mut self,
        value: u16,
        session: &mut Session,
    ) -> io::Result<Response> {
        session.writew(self.address, value).await
    }
}

impl Register<u32> {
    /// Reads a `u32` value from the register asynchronously.
    pub async fn read(&self, session: &mut Session) -> io::Result<u32> {
        session.readl(self.address).await
    }

    /// Writes a `u32` value to the register asynchronously.
    ///
    /// # Safety
    ///
    /// This function is `unsafe` because it directly accesses and modifies hardware registers,
    /// which can have side effects on the system if used improperly.
    pub async unsafe fn write(
        &mut self,
        value: u32,
        session: &mut Session,
    ) -> io::Result<Response> {
        session.writel(self.address, value).await
    }
}

/// Implementation for `Register<u64>`, with read and write capabilities for `u64` data types.
impl Register<u64> {
    /// Reads a `u64` value from the register asynchronously.
    pub async fn read(&self, session: &mut Session) -> io::Result<u64> {
        session.readq(self.address).await
    }

    /// Writes a `u64` value to the register asynchronously.
    ///
    /// # Safety
    ///
    /// This function is `unsafe` because it directly accesses and modifies hardware registers,
    /// which can have side effects on the system if used improperly.
    pub async unsafe fn write(
        &mut self,
        value: u64,
        session: &mut Session,
    ) -> io::Result<Response> {
        session.writeq(self.address, value).await
    }
}

#[macro_export]
macro_rules! register {
    ($($name:ident, $type:ty),*) => {
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
