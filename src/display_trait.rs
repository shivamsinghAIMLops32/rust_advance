use core::fmt;
pub trait CarProperties<'a> {
    fn new(color: &'a str, model: &'a str, year: u32, price: u32) -> Self;
}

// also wwe can use #[derive(Debug)] to automatically implement the Debug trait for the Car struct, which allows us to print the struct using the {:?} format specifier.
pub struct Car<'a> {
    color: &'a str,
    model: &'a str,
    year: u32,
    price: u32,
}

// contructor for car struct
impl<'a> CarProperties<'a> for Car<'a> {
    fn new(color: &'a str, model: &'a str, year: u32, price: u32) -> Self {
        Self {
            color,
            model,
            year,
            price,
        }
    }
}

impl fmt::Display for Car<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Car {{ color: {}, model: {}, year: {}, price: {} }}",
            self.color, self.model, self.year, self.price
        )
    }
}
