pub struct SystemInfo {
    pub cpu: String,
    pub gpu: String,
    pub ram: String,
    pub storage: String,
}

pub system {
    pub fn get_system_info() -> SystemInfo {
        get_info()
    }
}
