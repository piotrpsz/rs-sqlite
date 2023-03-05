#[macro_use]

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

macro_rules! sql_error(
    ($db: expr) => (
        let (file, func, line) = fpos!();
        eprintln!("ERROR [{}/{}:{}] {} ({})", file, func, line, $db.err_string(), $db.err_code());
    );
);

macro_rules! check(
    ($db: expr) => (
        if $db.fpath.is_empty() {
            panic!("unkown database path");
        }
    );
);

macro_rules! str2ptr(
    ($str:expr) => (
        CString::new($str).unwrap().into_raw()
    );
);

macro_rules! ptr2str(
    ($ptr:expr) => (
        String::from_utf8_lossy(CString::from_raw($ptr).to_bytes()).into_owned().clone()
    );
);



