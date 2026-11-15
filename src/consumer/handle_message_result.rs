use lapin::Channel;
use lapin::Error;
use lapin::options::BasicAckOptions;
use lapin::options::BasicNackOptions;
use lapin::types::LongLongUInt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HandleMessageResult {
    Ack,
    NackNoRequeue,
    NackWithRequeue,
}

/// Consumer Acknowledgements
/// Sends Positive or Negative Acknowledgement
/// * Requeues if needed
pub async fn action_result(
    result: HandleMessageResult,
    channel: &Channel,
    tag: LongLongUInt,
) -> Result<(), Error> {
    match result {
        HandleMessageResult::Ack => {
            channel
                .basic_ack(tag, BasicAckOptions { multiple: false })
                .await
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
        }
    }
}
