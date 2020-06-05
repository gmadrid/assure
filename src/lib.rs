#[macro_export]
macro_rules! assure {
    ( $c:expr, $e:expr ) => {
        if !$c {
            return Err($e);
        }
    };
}

