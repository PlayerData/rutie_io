use rutie::{
    class, methods, AnyException, AnyObject, Class, Exception, NilClass, Object, RString, VM,
};
use rutie_io::RubyIOBackend;

use std::io::{Read, Write};

class!(RutieIOExample);

pub fn raise_error<T, I>(error_type: &'static str) -> Box<dyn Fn(I) -> T>
where
    I: std::fmt::Debug,
{
    Box::new(move |e| {
        VM::raise_ex(AnyException::new(
            error_type,
            Some(format!("{e:#?}").as_str()),
        ));
        unreachable!();
    })
}

methods!(
    RutieIOExample,
    _rtself,
    fn echo(input_io: AnyObject, output_io: AnyObject) -> NilClass {
        let Ok(mut input_io) = input_io
            .unwrap_or_else(raise_error("ArgumentError"))
            .try_convert_to::<RubyIOBackend>()
        else {
            VM::raise_ex(AnyException::new(
                "ArgumentError",
                Some("First argument was not StringIO or a subclass of IO"),
            ));
            unreachable!();
        };

        let Ok(mut output_io) = output_io
            .unwrap_or_else(raise_error("ArgumentError"))
            .try_convert_to::<RubyIOBackend>()
        else {
            VM::raise_ex(AnyException::new(
                "ArgumentError",
                Some("First argument was not StringIO or a subclass of IO"),
            ));
            unreachable!();
        };

        let mut input = [0; 1024];

        let len = input_io.read(&mut input).unwrap();
        let _len = output_io.write(&input[0..len]).unwrap();

        NilClass::new()
    }
);

methods!(
    RutieIOExample,
    _rtself,
    fn read_char(input_io: AnyObject) -> RString {
        let Ok(mut input_io) = input_io
            .unwrap_or_else(raise_error("ArgumentError"))
            .try_convert_to::<RubyIOBackend>()
        else {
            VM::raise_ex(AnyException::new(
                "ArgumentError",
                Some("First argument was not StringIO or a subclass of IO"),
            ));
            unreachable!();
        };

        let mut input = [0; 1];

        let len = input_io.read(&mut input).unwrap_or(0);
        RString::new_utf8(std::str::from_utf8(&input[0..len]).unwrap())
    }
);

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn Init_rutie_io_example() {
    Class::new("RutieIOExample", None).define(|klass| {
        klass.def_self("echo", echo);
        klass.def_self("read_char", read_char);
    });
}
