use std::cmp;

use crate::Coords;

#[derive(Debug)]
pub struct Pack {
    pub children: Option<Box<(Pack, Pack)>>,
    pub top: u16,
    pub left: u16,
    pub bottom: u16,
    pub right: u16,
    pub elem: Option<Coords>,
}

impl Pack {
    // for best results, sort elements by height before insertion
    pub fn insert(&mut self, dim: Coords) -> Option<Coords> {
        if !self.is_leaf() {
            let (mut right, mut below) = *self.children.take().unwrap();
            // must start with right, because we have "infinite" height
            // due to paging
            let res = right.insert(dim).or_else(|| below.insert(dim));
            self.children = Some(Box::new((right, below)));
            res
        } else {
            if self.elem.is_some() {
                return None;
            }
            if !self.can_accomodate(dim) {
                return None;
            }

            // we will accomodate
            self.elem = Some(dim);

            let cap = self.dim();
            if cap == dim {
                // no need for splitting remainder
                return Some((self.left, self.top));
            }

            // we need to split so that future inserts can
            // use the slack left from this insert
            let dx = cap.0 - dim.0;
            let dy = cap.1 - dim.1;

            // resize self
            self.right -= dx;
            self.bottom -= dy;

            let right = Pack {
                children: None,
                top: self.top,
                left: self.right,
                bottom: self.bottom,
                right: self.right + dx,
                elem: None,
            };

            let below = Pack {
                children: None,
                top: self.bottom,
                left: self.left,
                bottom: self.bottom + dy,
                right: self.right + dx,
                elem: None,
            };

            self.children = Some(Box::new((right, below)));

            Some((self.left, self.top))
        }
    }

    fn dim(&self) -> Coords {
        trace!("dim({:?})", self);
        (cmp::max(self.right, self.left) - self.left, cmp::max(self.bottom, self.top) - self.top)
    }

    fn is_leaf(&self) -> bool {
        self.children.is_none()
    }

    fn can_accomodate(&self, dim: Coords) -> bool {
        let capacity = self.dim();
        capacity.0 >= dim.0 && capacity.1 >= dim.1
    }
}
