pub struct UnalignedBuffer<T> {
    ptr: *mut T,
    length: usize,
}

impl<T> UnalignedBuffer<T> {
    pub unsafe fn from_parts(ptr: *mut T, length: usize) -> Self {
        Self { ptr, length }
    }
    pub fn get(&self, idx: usize) -> T {
        assert!(idx < self.length);
        unsafe { self.ptr.offset(idx as isize).read_unaligned() }
    }
    pub fn put(&self, idx: usize, item: T) {
        assert!(idx < self.length);
        unsafe { self.ptr.offset(idx as isize).write_unaligned(item) }
    }
}
impl<T: Copy> UnalignedBuffer<T> {
    pub fn put_buffer(&mut self, buf: &[T]) {
        let len = buf.len();
        assert!(len < self.length);
        let src = &buf[0] as *const _ as *const u8;
        let dst = self.ptr as *mut u8;
        unsafe {
            std::ptr::copy_nonoverlapping(src, dst, len * std::mem::size_of::<T>());
        }
    }
    pub fn get_to_buffer(&self, buf: &mut [T]) {
        let len = buf.len();
        assert!(len < self.length);
        let src = &mut buf[0] as *mut _ as *mut u8;
        let dst = self.ptr as *const u8;
        unsafe {
            std::ptr::copy_nonoverlapping(dst, src, len * std::mem::size_of::<T>());
        }
    }
}
