fn main() {
    println!("Hello, world!");
    let claim1 = Claim {
        id: 1,
        offset_x: 1,
        offset_y: 3,
        w: 4,
        h: 4,
    };
    let claim2 = Claim {
        id: 2,
        offset_x: 3,
        offset_y: 1,
        w: 4,
        h: 4,
    };
    let claim3 = Claim {
        id: 3,
        offset_x: 5,
        offset_y: 5,
        w: 2,
        h: 2,
    };

    let claims = vec![claim1, claim2, claim3];

    let mut sum: i32 = 0;

    for (i, claim) in claims.iter().enumerate() {
        sum += claims
            .iter()
            .skip(i + 1)
            .filter(|other| claim.overlaps(other))
            .count() as i32;
    }

    println!("sum is {}", sum);
}

struct Claim {
    id: i32,
    offset_x: i32,
    offset_y: i32,
    w: i32,
    h: i32,
}

impl Claim {
    fn overlaps(&self, other: &Claim) -> bool {
        self.contains(&other.top_left())
            || self.contains(&other.top_right())
            || self.contains(&other.bottom_left())
            || self.contains(&other.bottom_right())
    }

    fn contains(&self, p: &P) -> bool {
        self.offset_x <= p.0
            && p.0 < self.offset_x + self.w
            && self.offset_y <= p.1
            && p.1 < self.offset_y + self.h
    }

    fn top_left(&self) -> P {
        P(self.offset_x, self.offset_y)
    }

    fn top_right(&self) -> P {
        P(self.offset_x + self.w, self.offset_y)
    }

    fn bottom_left(&self) -> P {
        P(self.offset_x, self.offset_y + self.h)
    }

    fn bottom_right(&self) -> P {
        P(self.offset_x + self.w, self.offset_y + self.h)
    }
}

struct P(i32, i32);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn contains_point() {
        let claim = Claim {
            id: 1,
            offset_x: 1,
            offset_y: 1,
            w: 2,
            h: 2,
        };

        assert!(claim.contains(&P(1, 1)));
        assert!(claim.contains(&P(2, 2)));
        assert!(claim.contains(&P(1, 2)));
        assert!(!claim.contains(&P(0, 0)));
        assert!(!claim.contains(&P(1, 3)));
        assert!(!claim.contains(&P(3, 1)));
    }

    #[test]
    fn same_boxes_overlap() {
        let claim1 = Claim {
            id: 1,
            offset_x: 1,
            offset_y: 1,
            w: 2,
            h: 2,
        };
        let claim2 = Claim {
            id: 2,
            offset_x: 1,
            offset_y: 1,
            w: 2,
            h: 2,
        };

        assert!(claim1.overlaps(claim2));
    }

    #[test]
    fn adjacent_boxes() {
        let claim1 = Claim {
            id: 1,
            offset_x: 1,
            offset_y: 1,
            w: 2,
            h: 2,
        };
        let claim2 = Claim {
            id: 2,
            offset_x: 3,
            offset_y: 1,
            w: 2,
            h: 2,
        };

        assert!(!claim1.overlaps(claim2));
    }

    #[test]
    fn slightly_overlapping() {
        let claim1 = Claim {
            id: 1,
            offset_x: 1,
            offset_y: 1,
            w: 2,
            h: 2,
        };
        let claim2 = Claim {
            id: 2,
            offset_x: 2,
            offset_y: 2,
            w: 2,
            h: 2,
        };

        assert!(claim1.overlaps(claim2));
    }

    #[test]
    fn stacked_boxes() {
        let claim1 = Claim {
            id: 1,
            offset_x: 1,
            offset_y: 1,
            w: 2,
            h: 2,
        };
        let claim2 = Claim {
            id: 2,
            offset_x: 1,
            offset_y: 3,
            w: 2,
            h: 2,
        };

        assert!(!claim1.overlaps(claim2));
    }
}
