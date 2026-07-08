use disruptor::{MultiProducer, SingleConsumerBarrier};

pub trait Executor {
    fn send_order(&self);
    fn cancel_order(&self);
}

pub trait DataProvider<T> {
    async fn listen_trades(&self, disruptor: MultiProducer<T, SingleConsumerBarrier>);
}

pub trait Exchange<T>: DataProvider<T> + Executor {}
