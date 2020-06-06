use lapin::Channel;
use lapin::options::BasicNackOptions;
use lapin::options::BasicAckOptions;
use lapin::types::LongLongUInt;

pub enum HandleMessageResult {
    Ack,
    NackNoRequeue,
    NackWithRequeue,
}

/// Consumer Acknowledgements
/// Sends Positive or Negative Acknowledgement
/// * Requeues if needed
pub async fn action_result(result: HandleMessageResult, channel: &Channel, tag: LongLongUInt) {
    match result {
        HandleMessageResult::Ack => {
            channel
                .basic_ack(tag, BasicAckOptions { multiple: false })
                .await
                .expect("basic_ack");
        }
        HandleMessageResult::NackNoRequeue => {
            channel
                .basic_nack(
                    tag,
                    BasicNackOptions {
                        multiple: false,
                        requeue: false,
                    },
                )
                .await
                .expect("basic_ack");
        }
        HandleMessageResult::NackWithRequeue => {
            channel
                .basic_nack(
                    tag,
                    BasicNackOptions {
                        multiple: false,
                        requeue: true,
                    },
                )
                .await
                .expect("basic_ack");
        }
    }
}
