#[derive(Debug)]
pub enum CompilationError{
    ParsingError,    
    //LinkerError
}

#[macro_export]
macro_rules! unrecoverable_error {
    ($err_msg:expr, $err_cause: expr) => {
        panic!("\n{} {} => {}{} {} {}\n\n",
            Color::CRed,
            $err_msg,
            Color::BBlack, Color::CYellow,
            $err_cause,
            Color::Reset
        );
    };
}