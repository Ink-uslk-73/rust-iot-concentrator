//! src/core/platform.rs - определение ОС один раз при старте.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Platform { Linux, Windows, Other }

impl Platform {
    pub fn current() -> Self {
        #[cfg(target_os = "linux")]
        { return Platform::Linux; }
        #[cfg(windows)]
        { return Platform::Windows; }
        #[cfg(not(any(target_os = "linux", windows)))]
        { Platform::Other }
    }

    pub fn is_linux(self) -> bool { matches!(self, Platform::Linux) }
    pub fn is_windows(self) -> bool { matches!(self, Platform::Windows) }

    pub fn as_str(self) -> &'static str {
        match self { Platform::Linux => "linux", Platform::Windows => "windows", Platform::Other => "other" }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn current_is_known() {
        let p = Platform::current();
        assert!(matches!(p, Platform::Linux | Platform::Windows | Platform::Other));
    }

    #[test]
    fn is_linux_matches() {
        assert!(Platform::Linux.is_linux());
        assert!(!Platform::Windows.is_linux());
    }
}

