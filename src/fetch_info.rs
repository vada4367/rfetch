use libc::size_t;

#[derive(Clone, Copy)]
pub struct FetchInfo {
    pub logo: bool,
    pub user_host: bool,
    pub os: bool,
    pub device: bool,
    pub kernel: bool,
    pub uptime: bool,
    pub pkgs: bool,
    pub memory: bool,
}

impl FetchInfo {
    pub fn max_length(self) -> size_t {
        if self.kernel || self.uptime || self.memory {
            return 6;
        }
        if self.pkgs || self.device {
            return 4;
        }
        return 2;
    }
}
