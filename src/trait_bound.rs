use std::fmt::{ Display};

pub trait Loggable: std::fmt::Debug {   // this is a trait that defines a contract for logging functionality. It requires any struct that implements this trait to also implement the Debug trait, which allows for formatted output of the struct's data.
    fn log(&self); // this is a contract that every struct has to implement this method

    // default implementation
    fn default_log(&self) {
        println!("This is a default log message {:?}", self); // this is a default implementation of the log method and it will be used if the struct does not implement the log method
    }
}

#[derive(Debug)]  // or implement the display trait for the struct
struct DbLogger {
    connection_string: String,
}


// lets implement display trait for dblogger struct
impl Display for DbLogger{
     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.connection_string, "DbLogger")
    }
}

// Implementing the Loggable trait for DbLogger struct 
impl Loggable for DbLogger {
    fn log(&self) {
        println!(
            "Logging to database with connection string: {}",
            self.connection_string
        );
    }
}

// trait bound is a way to specify that a generic type must implement a certain trait. It is used to ensure that the generic type has the necessary functionality to be used in a certain context.
// best way to implement a trait using where
fn log_message<T,U>(str_example: &T, logger: &U) where T: Clone + Display, U: Loggable {
    println!("Logging message: {}", str_example);
    logger.log();
}