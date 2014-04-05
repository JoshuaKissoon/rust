use std::io;
use std::io::IoResult;
use std::os;

fn run() -> IoResult<()> {
    let mut out = io::stdio::stdout_raw();
    for _ in range(0u, 32) {
        let mut buf = ['x' as u8, ..1024];
        buf[1023] = '\n' as u8;
        try!(out.write(buf));
    }
    Ok(())
}

fn main() {
    match run() {
        Err(e) => {
            (writeln!(&mut io::stderr(), "Error: {}", e)).unwrap();
            os::set_exit_status(1);
        }
        Ok(()) => ()
    }
}
