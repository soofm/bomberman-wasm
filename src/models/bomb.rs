pub struct Bomb {
    timer: u32,
    owner_id: u32
}

impl Bomb {
    pub fn new(owner_id: u32) -> Bomb {
        Bomb { timer: 3, owner_id: owner_id }
    }
}