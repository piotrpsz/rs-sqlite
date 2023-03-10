#[allow(unused_macros)]
/*
 * Copyright (C) 2023 Piotr Pszczółkowski
 * Licence: GNU v2
 *
 * E-mail: piotr@beesoft.pl
 *
 * Project: rs-sqlite
 * File: macros.inc
 */

macro_rules! fpos {
        () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
           std::any::type_name::<T>()
        }

        let func = type_name_of(f).split(':')
            .rev()
            .filter(|s| !s.is_empty())
            .skip(1)
            .take(1)
            .map(|s| s.to_string())
            .next()
            .unwrap();

        let file =  file!().split('/')
            .rev()
            .map(|s| s.to_string())
            .take(1)
            .next()
            .unwrap();

        (file, func, format!("{}", line!()))
    }}
}

#[allow(unused_macros)]
macro_rules! sql_error(
    ($db: expr) => (
//        unsafe {
            let (file, func, line) = fpos!();
            eprintln!("ERROR [{}/{}:{}] {} ({})", file, func, line, SQLite::err_string($db), SQLite::err_code($db));
//        }
    );
);


macro_rules! str2ptr(
    ($str:expr) => (
        CString::new($str).unwrap().into_raw()
    );
);




