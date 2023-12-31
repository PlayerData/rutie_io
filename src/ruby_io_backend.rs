use rutie::{class, AnyObject, Encoding, Integer, NilClass, Object, RString, VerifiedObject};

class!(RubyIOBackend);

impl VerifiedObject for RubyIOBackend {
    fn is_correct_type<T: Object>(object: &T) -> bool {
        let supported_parents = [
            rutie::Class::from_existing("IO"),
            rutie::Class::from_existing("StringIO"),
            rutie::Class::from_existing("Tempfile"),
        ];

        object
            .class()
            .ancestors()
            .iter()
            .any(|parent| supported_parents.contains(parent))
    }

    fn error_message() -> &'static str {
        "Error converting to IO"
    }
}

fn get_class_name(object: AnyObject) -> Result<String, String> {
    let class_name_object = object
        .class()
        .protect_public_send("name", &[])
        .map_err(|e| format!("Couldnt request class name {:?}", e))?;

    class_name_object
        .try_convert_to::<RString>()
        .map(|rstring| rstring.to_string())
        .map_err(|e| format!("Couldnt convert class name to string {:?}", e))
}

impl std::io::Read for RubyIOBackend {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let read_size = Integer::from(buf.len() as u32).to_any_object();
        let result = match self.protect_public_send("read", &[read_size]) {
            Ok(result) => result,

            Err(e) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Failed to read from IO, ruby error {:?}", e),
                ));
            }
        };

        if result.try_convert_to::<NilClass>().is_ok() {
            return Ok(0);
        }

        let string = match result.try_convert_to::<RString>() {
            Ok(result) => result,

            Err(e) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "Failed to convert ruby result to RString {:?} class was {:?}",
                        e,
                        get_class_name(result)
                    ),
                ));
            }
        };

        let bytes = string.to_bytes_unchecked();
        let bytes_len = bytes.len();

        if bytes_len > buf.len() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to read from IO, buffer too small",
            ));
        }

        buf[..bytes_len].copy_from_slice(&bytes[..bytes_len]);

        Ok(bytes_len)
    }
}

impl std::io::Write for RubyIOBackend {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let result = match self.protect_public_send(
            "write",
            &[RString::from_bytes(buf, &Encoding::utf8()).to_any_object()],
        ) {
            Ok(result) => result,

            Err(e) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Failed to write to IO, ruby error {:?}", e),
                ));
            }
        };

        match result.try_convert_to::<rutie::Integer>() {
            Ok(result) => Ok(result.to_i64() as usize),
            Err(_) => Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to convert result to integer",
            )),
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        match self.protect_public_send("flush", &[]) {
            Ok(_) => Ok(()),
            Err(e) => Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to flush, ruby error {:?}", e),
            )),
        }
    }
}
