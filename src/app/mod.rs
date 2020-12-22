
pub enum CerialMode {
    Menu,
    Input,
    HexInput,
}

pub struct CerialState {
    pub input: String,
    pub mode: CerialMode,
    pub exit: bool
}