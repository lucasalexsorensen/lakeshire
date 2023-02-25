
#[derive(Debug, Clone, Copy)]
pub struct ControlPixel {
    pub control_first: u8,
    pub control_second: u8,
    pub cell_size: usize,
}

impl ControlPixel {
    pub fn from_offset(buffer: &[u8]) -> Self {
        ControlPixel {
            control_first: buffer[2],
            control_second: buffer[1],
            cell_size: buffer[0] as usize,
        }
    }

    pub fn is_valid(&self) -> bool {
        self.control_first == 199 && self.control_second == 99
    }
}

#[derive(Debug, Clone, Copy)]
pub struct MetaPixel {
    pub remainder: u8,
    pub least_significant: u8,
    pub most_significant: u8
}

impl MetaPixel {
    pub fn from_offset(buffer: &[u8]) -> Self {
        MetaPixel {
            remainder: buffer[2],
            least_significant: buffer[1],
            most_significant: buffer[0]
        }
    }

    pub fn compute_msg_length(&self) -> u16 {
        (self.most_significant as u16) << 8 | self.least_significant as u16
    }
}
