use super::write_log;

pub fn info(msg: &str) {
    write_log("INFO", msg);
}

pub fn warn(msg: &str) {
    write_log("WARN", msg);
}

pub fn error(msg: &str) {
    write_log("ERROR", msg);
}

pub fn debug(msg: &str) {
    write_log("DEBUG", msg);
}
