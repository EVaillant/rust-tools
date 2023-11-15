pub trait Observer {
    fn notif(&self);
}

pub struct Observable {}
