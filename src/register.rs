/**
 * Rust abstraction to represent hardware registers with asynchronous read and write capabilities.
 * Supports reading and writing values of various sizes (`u8`, `u16`, `u32`, `u64`).
 */

 use crate::parser::Parser;
 use crate::socket::Socket;
 use crate::Response;
 use std::marker::PhantomData;
 use std::io;
 
 /**
  * The `Register` struct represents a generic hardware register.
  * 
  * # Type Parameters
  * - `T`: Type of data stored in the register (`u8`, `u16`, `u32`, `u64`).
  * 
  * Each register has a unique address, and an associated type marker `PhantomData<T>`.
  * The `PhantomData` is used here to store the type information without actually holding any data of that type.
  */
 #[derive(Debug)]
 pub struct Register<T> {
     address: usize,           // Memory address of the register.
     _size_marker: PhantomData<T>, // Type marker, ensuring the struct is generic over `T`.
 }
 
 impl<T> Register<T> {
     /**
      * Creates a new `Register` instance.
      * 
      * # Parameters
      * - `address`: Memory-mapped address of the register.
      * 
      * # Returns
      * An instance of `Register` wrapped in a `Result` to handle potential I/O errors.
      */
     pub async fn new(address: usize) -> io::Result<Self> {
         Ok(Register {
             address,
             _size_marker: PhantomData,
         })
     }
 }
 
 // Implementation for `Register<u8>`, allowing asynchronous read/write operations on `u8` data types.
 impl Register<u8> {
     /**
      * Reads an `u8` value from the register asynchronously.
      */
     pub async fn read_register<P>(&self, parser: &mut Parser<P>) -> u8
     where
         P: Socket,
     {
         parser
             .readb(self.address)
             .await
             .expect("Error reading u8 from register")
     }
 
     /**
      * Writes an `u8` value to the register asynchronously.
      */
     pub async fn write_register<P>(&self, value: u8, parser: &mut Parser<P>) -> Response
     where
         P: Socket,
     {
         parser
             .writeb(self.address, value)
             .await
             .expect("Error writing u8 to register")
     }
 }
 
 // Implementation for `Register<u16>`, supporting read and write operations on `u16` data types.
 impl Register<u16> {
     /**
      * Reads a `u16` value from the register asynchronously.
      */
     pub async fn read_register<P>(&self, parser: &mut Parser<P>) -> u16
     where
         P: Socket,
     {
         parser
             .readw(self.address)
             .await
             .expect("Error reading u16 from register")
     }
 
     /**
      * Writes a `u16` value to the register asynchronously.
      */
     pub async fn write_register<P>(&self, value: u16, parser: &mut Parser<P>) -> Response
     where
         P: Socket,
     {
         parser
             .writew(self.address, value)
             .await
             .expect("Error writing u16 to register")
     }
 }
 
 // Implementation for `Register<u32>`, enabling read and write operations for `u32` data types.
 impl Register<u32> {
     /**
      * Reads a `u32` value from the register asynchronously.
      */
     pub async fn read_register<P>(&self, parser: &mut Parser<P>) -> u32
     where
         P: Socket,
     {
         parser
             .readl(self.address)
             .await
             .expect("Error reading u32 from register")
     }
 
     /**
      * Writes a `u32` value to the register asynchronously.
      */
     pub async fn write_register<P>(&self, value: u32, parser: &mut Parser<P>) -> Response
     where
         P: Socket,
     {
         parser
             .writel(self.address, value)
             .await
             .expect("Error writing u32 to register")
     }
 }
 
 // Implementation for `Register<u64>`, with read and write capabilities for `u64` data types.
 impl Register<u64> {
     /**
      * Reads a `u64` value from the register asynchronously.
      */
     pub async fn read_register<P>(&self, parser: &mut Parser<P>) -> u64
     where
         P: Socket,
     {
         parser
             .readq(self.address)
             .await
             .expect("Error reading u64 from register")
     }
 
     /**
      * Writes a `u64` value to the register asynchronously.
      */
     pub async fn write_register<P>(&self, value: u64, parser: &mut Parser<P>) -> Response
     where
         P: Socket,
     {
         parser
             .writeq(self.address, value)
             .await
             .expect("Error writing u64 to register")
     }
 }
 