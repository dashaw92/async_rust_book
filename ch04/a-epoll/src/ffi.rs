//https://man7.org/linux/man-pages/man2/epoll_ctl.2.html

//1 is the operation for adding a registration to epoll
pub(crate) const EPOLL_CTL_ADD: i32 = 1;
//We're interested in read operations
pub(crate) const EPOLLIN: i32 = 0x1;
//Edge triggering mode for epoll events
pub(crate) const EPOLLET: i32 = 1 << 31;

#[link(name = "c")]
extern "C" {
    pub(crate) fn epoll_create(size: i32) -> i32;
    pub(crate) fn close(fd: i32) -> i32;
    pub(crate) fn epoll_ctl(epfd: i32, op: i32, fd: i32, event: *mut Event) -> i32;
    pub(crate) fn epoll_wait(epfd: i32, events: *mut Event, max_events: i32, timeout: i32) -> i32;
}

#[derive(Debug)]
#[repr(C, packed)]
pub struct Event {
    pub(crate) events: u32,
    pub(crate) epoll_data: usize,
}

impl Event {
    pub fn token(&self) -> usize {
        self.epoll_data
    }
}