use std::ops::{Deref, DerefMut};

pub struct AsyncDestruction<T: 'static + Send> {
    ptr: *mut T,
}

impl<T: 'static + Send> AsyncDestruction<T> {
    #[inline(always)]
    pub fn new(x: T) -> Self {
        Self {
            ptr: Box::leak(Box::new(x)),
        }
    }
}

impl<T: 'static + Send> AsMut<T> for AsyncDestruction<T> {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut T {
        unsafe { &mut *self.ptr }
    }
}
impl<T: 'static + Send> AsRef<T> for AsyncDestruction<T> {
    #[inline(always)]
    fn as_ref(&self) -> &T {
        unsafe { &*self.ptr }
    }
}

impl<T: 'static + Send> Deref for AsyncDestruction<T> {
    type Target = T;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.ptr }
    }
}

impl<T: 'static + Send> DerefMut for AsyncDestruction<T> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.ptr }
    }
}

impl<T: 'static + Send> Drop for AsyncDestruction<T> {
    fn drop(&mut self) {
        let inner = unsafe { Box::from_raw(self.ptr) };
        tokio::spawn(async move { drop(inner) });
    }
}

#[cfg(test)]
mod tests {
    use super::AsyncDestruction;
    use chrono::Utc;
    use std::{thread::sleep, time::Duration};

    #[derive(Clone)]
    struct S;
    impl Drop for S {
        fn drop(&mut self) {
            sleep(Duration::from_millis(1));
            println!("drop!");
        }
    }
    impl S {
        pub fn do_sth(&self) {
            println!("do_sth");
        }
    }

    #[test]
    fn it_works() {
        let a = vec![S; 10];
        a[0].do_sth();
        let t1 = Utc::now().timestamp_millis();
        drop(a);
        let t2 = Utc::now().timestamp_millis();
        println!("drop cost time: {}ms", t2 - t1);
    }

    #[tokio::test]
    async fn async_works() {
        let a = AsyncDestruction::new(vec![S; 10]);
        a[0].do_sth();
        let t1 = Utc::now().timestamp_millis();
        drop(a);
        let t2 = Utc::now().timestamp_millis();
        println!("drop cost time: {}ms", t2 - t1);
    }
}
