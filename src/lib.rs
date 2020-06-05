#[macro_export]
macro_rules! ensure {
    ( $c:expr, $e:expr ) => {
        if !$c {
            return Err($e);
        }
    };
}

