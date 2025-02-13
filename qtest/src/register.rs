use crate::{parser::Parser, socket::Socket, Response};
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
    pub fn new(name: &str, address: usize) -> Self {
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
    pub async fn read_register<P>(&self, parser: &mut Parser<P>) -> io::Result<u8>
    where
        P: Socket,
    {
        parser.readb(self.address).await
    }

    /// Writes a `u8` value to the register asynchronously.
    ///
    /// # Safety
    ///
    /// This function is `unsafe` because it directly accesses and modifies hardware registers,
    /// which can have side effects on the system if used improperly.
    pub async unsafe fn write_register<P>(
        &mut self,
        value: u8,
        parser: &mut Parser<P>,
    ) -> io::Result<Response>
    where
        P: Socket,
    {
        parser.writeb(self.address, value).await
    }
}

impl Register<u16> {
    /// Reads a `u16` value from the register asynchronously.
    pub async fn read_register<P>(&self, parser: &mut Parser<P>) -> io::Result<u16>
    where
        P: Socket,
    {
        parser.readw(self.address).await
    }

    /// Writes a `u16` value to the register asynchronously.
    ///
    /// # Safety
    ///
    /// This function is `unsafe` because it directly accesses and modifies hardware registers,
    /// which can have side effects on the system if used improperly.
    pub async unsafe fn write_register<P>(
        &mut self,
        value: u16,
        parser: &mut Parser<P>,
    ) -> io::Result<Response>
    where
        P: Socket,
    {
        parser.writew(self.address, value).await
    }
}

impl Register<u32> {
    /// Reads a `u32` value from the register asynchronously.
    pub async fn read_register<P>(&self, parser: &mut Parser<P>) -> io::Result<u32>
    where
        P: Socket,
    {
        parser.readl(self.address).await
    }

    /// Writes a `u32` value to the register asynchronously.
    ///
    /// # Safety
    ///
    /// This function is `unsafe` because it directly accesses and modifies hardware registers,
    /// which can have side effects on the system if used improperly.
    pub async unsafe fn write_register<P>(
        &mut self,
        value: u32,
        parser: &mut Parser<P>,
    ) -> io::Result<Response>
    where
        P: Socket,
    {
        parser.writel(self.address, value).await
    }
}

/// Implementation for `Register<u64>`, with read and write capabilities for `u64` data types.
impl Register<u64> {
    /// Reads a `u64` value from the register asynchronously.
    pub async fn read_register<P>(&self, parser: &mut Parser<P>) -> io::Result<u64>
    where
        P: Socket,
    {
        parser.readq(self.address).await
    }

    /// Writes a `u64` value to the register asynchronously.
    ///
    /// # Safety
    ///
    /// This function is `unsafe` because it directly accesses and modifies hardware registers,
    /// which can have side effects on the system if used improperly.
    pub async unsafe fn write_register<P>(
        &mut self,
        value: u64,
        parser: &mut Parser<P>,
    ) -> io::Result<Response>
    where
        P: Socket,
    {
        parser.writeq(self.address, value).await
    }
}
