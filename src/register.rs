/**
Abstracción en Rust para representar registros de hardware con la capacidad de leer
y escribir valores de distintos tamaños (u8, u16, u32, u64) de manera asíncrona.
*/
use crate::parser::Parser;
use crate::socket::Socket;
use crate::Response;
use std::io;

//sería necesario meter un parser aquí??
//Debería también meter en la implementación in&out?
#[derive(Debug)]
pub struct Register<T, P>
where
    P: Socket, // Asegura que P implemente el rasgo Socket
{
    name: String,
    address: usize,
    size: T,
    parser: Parser<P>, // El parser como campo de la estructura
}

impl<T, P> Register<T, P>
where
    P: Socket, // Asegura que P implemente el rasgo Socket
{
    pub async fn new(name: &str, address: usize, size: T, url: &str) -> io::Result<Self> {
        // Crear el parser
        let (parser, _irq_receiver) = Parser::<P>::new(url).await?;

        Ok(Register {
            name: name.to_string(),
            address,
            size,
            parser, // Pasar el parser a la estructura
        })
    }
}

//Para crear un registro habría que poner:
//let register = Register::<u8, TcpSocket>::new("MyRegister", 0x1000, 8, "localhost:3000").await?;

impl<T, P> Register<T, P>
where
    P: Socket,
{
    pub fn get_parser(&mut self) -> &mut Parser<P> {
        &mut self.parser
    }
}

// Implementación para `Register<u8, P>`
impl<P> Register<u8, P>
where
    P: Socket,
{
    // pub async fn in_register(&mut self) -> u8 {
    //     self.parser.inb(self.address).await.expect("Error reading u8 from register")
    // }
    // pub async fn out_register(&mut self, value: u8) -> Response{
    //     self.parser.outb(self.address, value).await.expect("Error writing u8 to register")
    // }
    pub async fn read_register(&mut self) -> u8 {
        self.parser
            .readb(self.address)
            .await
            .expect("Error reading u8 from register")
    }

    pub async fn write_register(&mut self, value: u8) -> Response {
        self.parser
            .writeb(self.address, value)
            .await
            .expect("Error writing u8 to register")
    }
}

// Implementación para `Register<u16, P>`
impl<P> Register<u16, P>
where
    P: Socket,
{
    // pub async fn in_register(&mut self) -> u16 {
    //     self.parser.inw(self.address).await.expect("Error reading u16 from register")
    // }
    // pub async fn out_register(&mut self, value: u16) -> Response{
    //     self.parser.outw(self.address, value).await.expect("Error writing u16 to register")
    // }
    pub async fn read_register(&mut self) -> u16 {
        self.parser
            .readw(self.address)
            .await
            .expect("Error reading u16 from register")
    }

    pub async fn write_register(&mut self, value: u16) -> Response {
        self.parser
            .writew(self.address, value)
            .await
            .expect("Error writing u16 to register")
    }
}

// Implementación para `Register<u32, P>`
impl<P> Register<u32, P>
where
    P: Socket,
{
    // pub async fn in_register(&mut self) -> u32 {
    //     self.parser.inl(self.address).await.expect("Error reading u32 from register")
    // }
    // pub async fn out_register(&mut self, value: u32) -> Response{
    //     self.parser.outl(self.address, value).await.expect("Error writing u32 to register")
    // }
    pub async fn read_register(&mut self) -> u32 {
        self.parser
            .readl(self.address)
            .await
            .expect("Error reading u32 from register")
    }

    pub async fn write_register(&mut self, value: u32) -> Response {
        self.parser
            .writel(self.address, value)
            .await
            .expect("Error writing u32 to register")
    }
}

// Implementación para `Register<u64, P>`
impl<P> Register<u64, P>
where
    P: Socket,
{
    pub async fn read_register(&mut self) -> u64 {
        self.parser
            .readq(self.address)
            .await
            .expect("Error reading u64 from register")
    }

    pub async fn write_register(&mut self, value: u64) -> Response {
        self.parser
            .writeq(self.address, value)
            .await
            .expect("Error writing u64 to register")
    }
}

// src/register.rs

// #[macro_use]

// use parser::{writeb, readb, writew, readw, writel, readl, writeq, readq};
// use std::io;

// pub struct Register<T> {
//     name: String,
//     address: usize,
//     size: T,
// }

// impl<T> Register<T> {
//     pub fn new(name: &str, address: usize, size: T) -> Self {
//         Register {
//             name: name.to_string(),
//             address,
//             size: T,
//         }
//     }

//     // Método de alto nivel para leer
//     pub async fn read(&self) -> Result<T, io::Error>
//     where
//         T: Default + std::str::FromStr,
//     {
//         match std::any::type_name::<T>() {
//             "u8" => Ok(readb(self.address).await?),
//             "u16" => Ok(readw(self.address).await?),
//             "u32" => Ok(writel(self.address).await?),
//             "u64" => Ok(readq(self.address).await?),
//             _ => Err(io::Error::new(io::ErrorKind::InvalidInput, "Unsupported type")),
//         }
//     }

//     // Método de alto nivel para escribir
//     pub async fn write(&self, value: T) -> Result<(), io::Error> {
//         match std::any::type_name::<T>() {
//             "u8" => writeb(self.address, value).await,
//             "u16" => writew(self.address, value).await,
//             "u32" => writel(self.address, value).await,
//             "u64" => writeq(self.address, value).await,
//             _ => Err(io::Error::new(io::ErrorKind::InvalidInput, "Unsupported type")),
//         }
//     }
// }

//-----------------------------------------------------

// // Implementación para u16
// impl Register<u16> {
//     pub async fn read_register(&self) -> u16 {
//         readw!(self.address).await.expect("Error reading u16 from register")
//     }

//     pub async fn write_register(&self, value: u16) {
//         writew!(self.address, value).await.expect("Error writing u16 to register")
//     }
// }

// // Implementación para u32
// impl Register<u32> {
//     pub async fn read_register(&self) -> u32 {
//         readl!(self.address).await.expect("Error reading u32 from register")
//     }

//     pub async fn write_register(&self, value: u32) {
//         writel!(self.address, value).await.expect("Error writing u32 to register")
//     }
// }

// // Implementación para u64
// impl Register<u64> {
//     pub async fn read_register(&self) -> u64 {
//         readq!(self.address).await.expect("Error reading u64 from register")
//     }

//     pub async fn write_register(&self, value: u64) {
//         writeq!(self.address, value).await.expect("Error writing u64 to register")
//     }
// }
