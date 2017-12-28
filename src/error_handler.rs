pub fn error_handler<F>(function: F, message: &str ) where F: Fn() -> ::std::result::Result<result, error> {
    let lit = message;
    let result = match function {
        Ok(result) => result,
        Err(error) => {
            let x: i32;
            x = error;
            panic!("{}, {}", message, error);
        },
    };
}
