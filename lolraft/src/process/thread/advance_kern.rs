use super::*;

#[derive(Clone)]
pub struct Thread {
    command_log: CommandLog,
    voter: Voter,
    consumer: EventConsumer<CommitEvent>,
    producer: EventProducer<KernEvent>,
}
impl Thread {
    async fn advance_once(&self) -> Result<()> {
        self.command_log
            .advance_kern_process(self.voter.clone())
            .await
    }

    fn do_loop(self) -> ThreadHandle {
        let fut = async move {
            loop {
                self.consumer.consume_events(Duration::from_secs(1)).await;
                while self.advance_once().await.is_ok() {
                    self.producer.push_event(KernEvent);
                    tokio::task::yield_now().await;
                }
            }
        };
        let fut = tokio::task::unconstrained(fut);
        let hdl = tokio::spawn(fut).abort_handle();
        ThreadHandle(hdl)
    }
}

pub fn new(
    command_log: CommandLog,
    voter: Voter,
    consumer: EventConsumer<CommitEvent>,
    producer: EventProducer<KernEvent>,
) -> ThreadHandle {
    Thread {
        command_log,
        voter,
        consumer,
        producer,
    }
    .do_loop()
}
