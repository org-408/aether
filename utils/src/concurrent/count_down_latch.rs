use tokio::sync::Mutex;
use tokio_condvar::Condvar;

pub struct CountDownLatch {
    count: Mutex<usize>,
    condvar: Condvar,
}
impl CountDownLatch {
    pub fn new(count: usize) -> Self {
        Self {
            count: Mutex::new(count),
            condvar: Condvar::new(),
        }
    }

    pub async fn count_down(&self) {
        let mut count = self.count.lock().await;
        *count -= 1;
        if *count == 0 {
            self.condvar.notify_all();
        }
    }

    pub async fn wait(&self) {
        let mut count = self.count.lock().await;
        while *count > 0 {
            count = self.condvar.wait(count).await;
        }
    }
}
