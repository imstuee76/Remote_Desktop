#[allow(dead_code)]
pub enum LinuxDisplayStack {
    X11,
    Wayland,
}

#[allow(dead_code)]
pub struct LinuxCaptureStub {
    pub stack: LinuxDisplayStack,
}

#[allow(dead_code)]
impl LinuxCaptureStub {
    pub fn new(stack: LinuxDisplayStack) -> Self {
        Self { stack }
    }
}

#[allow(dead_code)]
pub struct LinuxInputStub {
    pub stack: LinuxDisplayStack,
}

#[allow(dead_code)]
impl LinuxInputStub {
    pub fn new(stack: LinuxDisplayStack) -> Self {
        Self { stack }
    }
}
