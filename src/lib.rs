pub struct AsyncDrop<T: 'static + Send> {
    ptr: *mut T,
}

impl<T: 'static + Send> AsyncDrop<T> {
    #[inline(always)]
    pub fn new(x: T) -> Self {
        Self {
            ptr: Box::leak(Box::new(x)),
        }
    }
}

impl<T: 'static + Send> AsMut<T> for AsyncDrop<T> {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut T {
        unsafe { &mut *self.ptr }
    }
}
impl<T: 'static + Send> AsRef<T> for AsyncDrop<T> {
    #[inline(always)]
    fn as_ref(&self) -> &T {
        unsafe { &*self.ptr }
    }
}
impl<T: 'static + Send> Drop for AsyncDrop<T> {
    fn drop(&mut self) {
        let inner = unsafe { Box::from_raw(self.ptr) };
        tokio::spawn(async move { drop(inner) });
    }
}

#[cfg(test)]
mod tests {
    use super::AsyncDrop;
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

    #[test]
    fn it_works() {
        let a = vec![S; 10];
        let t1 = Utc::now().timestamp_millis();
        drop(a);
        let t2 = Utc::now().timestamp_millis();
        println!("drop cost time: {}ms", t2 - t1);
    }

    #[tokio::test]
    async fn async_works() {
        let a = AsyncDrop::new(vec![S; 10]);
        let t1 = Utc::now().timestamp_millis();
        drop(a);
        let t2 = Utc::now().timestamp_millis();
        println!("drop cost time: {}ms", t2 - t1);
    }
}