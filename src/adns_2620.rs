use core::ptr::{read_volatile, write_volatile};

/*
#define Config				0x40
#define Status				0x41
#define Delta_Y				0x42
#define Delta_X				0x43
#define SQual				0x44
#define MaxPix				0x45
#define MinPix				0x46
#define PixSum				0x47
#define PixData				0x48
#define ShutterUp			0x49
#define ShutterDown			0x4A
#define FramePeriod			0x4B
*/

pub struct Adns2620 {
    config: *const usize,
    status: *const usize,
    delta_x: *const usize,
    delta_y: *const usize,
    squal: *const usize,
    max_pix: *const usize,
    min_pix: *const usize,
    pix_sum: *const usize,
    pix_data: *const usize,
    shutter_up: *const usize,
    shutter_down: *const usize,
    frame_period: *const usize,
}

impl Adns2620 {
    pub fn new() -> Self {
        Adns2620 {
            config: 0x40 as *const usize,
            status: 0x41 as *const usize,
            delta_y: 0x42 as *const usize,
            delta_x: 0x43 as *const usize,
            squal: 0x44 as *const usize,
            max_pix: 0x45 as *const usize,
            min_pix: 0x46 as *const usize,
            pix_sum: 0x47 as *const usize,
            pix_data: 0x48 as *const usize,
            shutter_up: 0x49 as *const usize,
            shutter_down: 0x4A as *const usize,
            frame_period: 0x4B as *const usize,
        }
    }
    pub fn config(&self) -> usize {
        unsafe { read_volatile(self.config) }
    }

    pub fn status(&self) -> usize {
        unsafe { read_volatile(self.status) }
    }

    pub fn delta_x(&self) -> usize {
        unsafe { read_volatile(self.delta_x) }
    }

    pub fn delta_y(&self) -> usize {
        unsafe { read_volatile(self.delta_y) }
    }

    pub fn squal(&self) -> usize {
        unsafe { read_volatile(self.squal) }
    }

    pub fn max_pix(&self) -> usize {
        unsafe { read_volatile(self.max_pix) }
    }

    pub fn min_pix(&self) -> usize {
        unsafe { read_volatile(self.min_pix) }
    }

    pub fn pix_sum(&self) -> usize {
        unsafe { read_volatile(self.pix_sum) }
    }

    pub fn pix_data(&self) -> usize {
        unsafe { read_volatile(self.pix_data) }
    }

    pub fn start_pix_read(&self) {
        unsafe { write_volatile(self.pix_data as *mut usize, 0) }
    }

    pub fn shutter_up(&self) -> usize {
        unsafe { read_volatile(self.shutter_up) }
    }

    pub fn shutter_down(&self) -> usize {
        unsafe { read_volatile(self.shutter_down) }
    }
    pub fn frame_period(&self) -> usize {
        unsafe { read_volatile(self.frame_period) }
    }
}
