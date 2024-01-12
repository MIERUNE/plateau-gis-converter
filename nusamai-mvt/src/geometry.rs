const GEOM_COMMAND_MOVE_TO: u32 = 1;
const GEOM_COMMAND_LINE_TO: u32 = 2;
const GEOM_COMMAND_CLOSE_PATH: u32 = 7;

const GEOM_COMMAND_MOVE_TO_WITH_COUNT1: u32 = 1 << 3 | GEOM_COMMAND_MOVE_TO;
const GEOM_COMMAND_CLOSE_PATH_WITH_COUNT1: u32 = 1 << 3 | GEOM_COMMAND_CLOSE_PATH;

pub struct GeometryEncoder {
    buf: Vec<u32>,
    prev_x: i16,
    prev_y: i16,
}

impl GeometryEncoder {
    // TODO: with_capacity
    pub fn new() -> Self {
        Self {
            buf: Vec::new(),
            prev_x: 0,
            prev_y: 0,
        }
    }

    pub fn into_vec(self) -> Vec<u32> {
        self.buf
    }

    pub fn add_ring(&mut self, mut iter: impl Iterator<Item = [i16; 2]>) {
        let Some([first_x, first_y]) = iter.next() else {
            return;
        };
        let dx = (first_x - self.prev_x) as i32;
        let dy = (first_y - self.prev_y) as i32;
        (self.prev_x, self.prev_y) = (first_x, first_y);

        // move to
        self.buf.push(GEOM_COMMAND_MOVE_TO_WITH_COUNT1);
        self.buf.push(((dx << 1) ^ (dx >> 31)) as u32);
        self.buf.push(((dy << 1) ^ (dy >> 31)) as u32);

        // line to
        let lineto_cmd_pos = self.buf.len();
        self.buf.push(GEOM_COMMAND_LINE_TO); // length will be updated later
        let mut count = 0;
        for [x, y] in iter {
            let dx = (x - self.prev_x) as i32;
            let dy = (y - self.prev_y) as i32;
            (self.prev_x, self.prev_y) = (x, y);

            if dx != 0 || dy != 0 {
                self.buf.push(((dx << 1) ^ (dx >> 31)) as u32);
                self.buf.push(((dy << 1) ^ (dy >> 31)) as u32);
                count += 1;
            }
        }
        assert!(count >= 2);
        self.buf[lineto_cmd_pos] = GEOM_COMMAND_LINE_TO | count << 3;

        // close path
        self.buf.push(GEOM_COMMAND_CLOSE_PATH_WITH_COUNT1);
    }
}

impl Default for GeometryEncoder {
    fn default() -> Self {
        Self::new()
    }
}
