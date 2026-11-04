//! src/raw/source.rs - RawSource trait (P3.37).


pub trait RawSource: Send {
    /// Имя источника (интерфейс, файл).
    fn name(&self) -> &str;
    /// Захват следующего пакета (блокирующий, таймаут — по настройке).
    fn next(&mut self) -> Result<Option<Vec<u8>>, String>;
}

pub fn create_all() -> Vec<Box<dyn RawSource>> {
    vec![]
}
