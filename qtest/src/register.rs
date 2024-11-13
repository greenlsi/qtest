use crate::parser::Parser;
use crate::socket::Socket;
use crate::Response;
use std::marker::PhantomData;
use std::ops::DerefMut;

/// The `Register` struct represents a generic hardware register.
///
/// # Type Parameters
/// - `T`: The type of data stored in the register (`u8`, `u16`, `u32`, `u64`).
///
/// Each register has a name, a unique address, and an associated type marker `PhantomData<T>`.
/// The `PhantomData` is used here to retain type information without actually holding any data of that type.
#[derive(Debug)]
pub struct Register<T> {
    name: String,
    address: usize,               // Memory address of the register.
    _size_marker: PhantomData<T>, // Type marker, ensuring the struct is generic over `T`.
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
    /// ```rust
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

/// Implementation for `Register<u8>`, allowing asynchronous read/write operations on `u8` data types.
impl Register<u8> {
    ///Reads an `u8` value from the register asynchronously.
    pub async fn read_register<F, P>(&self, mut parser: F) -> u8
    where
        F: DerefMut<Target = Parser<P>>,
        P: Socket,
    {
        parser
            .readb(self.address)
            .await
            .expect("Error reading u8 from register")
    }

    /// Reads the value of a pin of the register asynchronously.
    pub async fn read_pin<P>(&self, pin: u8, parser: &mut Parser<P>) -> bool
    where
        P: Socket,
    {
        let lectura = self.read_register(parser).await;
        (lectura & (0x3 << pin)) != 0
    }

    /// Writes a `u8` value to the register asynchronously.
    ///
    /// # Safety
    ///
    /// This function is `unsafe` because it directly accesses and modifies hardware registers,
    /// which can have side effects on the system if used improperly.
    pub async unsafe fn write_register<P>(&mut self, value: u8, parser: &mut Parser<P>) -> Response
    where
        P: Socket,
    {
        parser
            .writeb(self.address, value)
            .await
            .expect("Error writing u8 to register")
    }
}

/// Implementation for `Register<u16>`, supporting read and write operations on `u16` data types.
impl Register<u16> {
    ///Reads a `u16` value from the register asynchronously.
    pub async fn read_register<P>(&self, parser: &mut Parser<P>) -> u16
    where
        P: Socket,
    {
        parser
            .readw(self.address)
            .await
            .expect("Error reading u16 from register")
    }

    /// Reads the value of a pin of the register asynchronously.
    pub async fn read_pin<P>(&self, pin: u8, parser: &mut Parser<P>) -> bool
    where
        P: Socket,
    {
        let lectura = self.read_register(parser).await;
        (lectura & (1 << pin)) != 0
    }

    /// Writes a `u16` value to the register asynchronously.
    ///
    /// # Safety
    ///
    /// This function is `unsafe` because it directly accesses and modifies hardware registers,
    /// which can have side effects on the system if used improperly.
    pub async unsafe fn write_register<P>(&mut self, value: u16, parser: &mut Parser<P>) -> Response
    where
        P: Socket,
    {
        parser
            .writew(self.address, value)
            .await
            .expect("Error writing u16 to register")
    }
}

/// Implementation for `Register<u32>`, enabling read and write operations for `u32` data types.
impl Register<u32> {
    ///Reads a `u32` value from the register asynchronously.
    pub async fn read_register<P>(&self, mut parser: impl DerefMut<Target = Parser<P>>) -> u32
    where
        P: Socket,
    {
        parser
            .readl(self.address)
            .await
            .expect("Error reading u32 from register")
    }

    /// Reads the value of a pin of the register asynchronously.
    pub async fn read_pin<P>(&self, pin: u8, parser: &mut Parser<P>) -> bool
    where
        P: Socket,
    {
        let lectura = self.read_register(parser).await;
        (lectura & (1 << pin)) != 0
    }

    /// Writes a `u32` value to the register asynchronously.
    ///
    /// # Safety
    ///
    /// This function is `unsafe` because it directly accesses and modifies hardware registers,
    /// which can have side effects on the system if used improperly.
    pub async unsafe fn write_register<P>(&mut self, value: u32, parser: &mut Parser<P>) -> Response
    where
        P: Socket,
    {
        parser
            .writel(self.address, value)
            .await
            .expect("Error writing u32 to register")
    }
}

/// Implementation for `Register<u64>`, with read and write capabilities for `u64` data types.
impl Register<u64> {
    ///Reads a `u64` value from the register asynchronously.
    pub async fn read_register<P>(&self, parser: &mut Parser<P>) -> u64
    where
        P: Socket,
    {
        parser
            .readq(self.address)
            .await
            .expect("Error reading u64 from register")
    }

    /// Reads the value of a pin of the register asynchronously.
    pub async fn read_pin<P>(&self, pin: u8, parser: &mut Parser<P>) -> bool
    where
        P: Socket,
    {
        let lectura = self.read_register(parser).await;
        (lectura & (1 << pin)) != 0
    }

    /// Writes a `u64` value to the register asynchronously.
    ///
    /// # Safety
    ///
    /// This function is `unsafe` because it directly accesses and modifies hardware registers,
    /// which can have side effects on the system if used improperly.
    pub async unsafe fn write_register<P>(&mut self, value: u64, parser: &mut Parser<P>) -> Response
    where
        P: Socket,
    {
        parser
            .writeq(self.address, value)
            .await
            .expect("Error writing u64 to register")
    }
}
