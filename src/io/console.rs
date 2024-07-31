#[macro_export]
macro_rules! _read_line {
    () => {
        match std::io::Write::flush(&mut std::io::stdout()) {
            Ok(_) => {
                let mut input = String::new();
                match std::io::Stdin::read_line(&mut std::io::stdin(), &mut input) {
                    Ok(_) => Ok::<String, std::io::Error>(input),
                    Err(error) => Err(error),
                }
            }
            Err(error) => Err(error),
        }
    };
}

#[macro_export]
macro_rules! prompt_read_line {
    ($($expr:expr),*) => {{
        print!($($expr),*);
        $crate::_read_line!()
    }};
}

#[macro_export]
macro_rules! promptln_read_line {
    ($($expr:expr),*) => {{
        println!($($expr),*);
        $crate::_read_line!()
    }};
}
