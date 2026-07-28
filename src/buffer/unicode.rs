use unicode_segmentation::GraphemeCursor;

pub struct GraphemeBounds<'a> {
    buf: &'a str,
    cursor1: GraphemeCursor,
    cursor2: GraphemeCursor,
    curr: (usize, usize),
}

impl<'a> GraphemeBounds<'a> {
    pub fn new(buf: &'a str) -> Self {
        let cursor1 = GraphemeCursor::new(0, buf.len(), true);
        let mut cursor2 = GraphemeCursor::new(0, buf.len(), true);
        // Advance cursor2 by one grapheme
        let g_end = cursor2.next_boundary(buf, 0).unwrap().unwrap();
        Self {
            buf,
            cursor1,
            cursor2,
            curr: (0, g_end),
        }
    }

    pub fn move_forward(&mut self) -> Option<(usize, usize)> {
        if let Ok(Some(boundary2)) = self.cursor2.next_boundary(self.buf, 0)
            && let Ok(Some(boundary1)) = self.cursor1.next_boundary(self.buf, 0)
        {
            let ret = self.curr;
            self.curr = (boundary1, boundary2);
            Some(ret)
        } else {
            None
        }
    }

    pub fn move_backward(&mut self) -> Option<(usize, usize)> {
        if let Ok(Some(boundary1)) = self.cursor1.prev_boundary(self.buf, 0)
            && let Ok(Some(boundary2)) = self.cursor2.prev_boundary(self.buf, 0)
        {
            self.curr = (boundary1, boundary2);
            Some(self.curr)
        } else {
            None
        }
    }
}
