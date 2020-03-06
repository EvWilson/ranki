use std::time::Duration;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(super) enum SchedStage {
    New,
    Learning(Duration),
    Reviewing(Duration),
    Relearning(Duration),
}

impl SchedStage {
    pub(super) fn duration(&self) -> Duration {
        use SchedStage::*;
        match self {
            New => Duration::from_secs(0),
            Learning(duration) | Reviewing(duration) | Relearning(duration) => *duration,
        }
    }
}

const DAY_IN_SECS: u64 = 86400;
const DAY: Duration = Duration::from_secs(DAY_IN_SECS);
const WEEK: Duration = Duration::from_secs(DAY_IN_SECS * 7);
const MONTH: Duration = Duration::from_secs(DAY_IN_SECS * 30);

/// This function contains the brains behind the scheduling algorithm. It is
/// heavily influenced by the work done on Anki with its usage of the
/// SuperMemo 2 algorithm. This version contains a simplified rendition for the
/// moment, to ease development.
pub(super) fn schedule(stage: &SchedStage, success: bool) -> SchedStage {
    use SchedStage::*;

    return match (stage, success) {
        (New, true) => Learning(DAY),
        (New, false) => New,
        (Learning(duration), true) => {
            if duration >= &WEEK {
                Learning(WEEK)
            } else {
                Learning(*duration + DAY)
            }
        }
        (Learning(duration), false) => Learning(*duration),
        (Reviewing(duration), true) => Reviewing(std::cmp::min(*duration * 2, MONTH)),
        (Reviewing(_), false) => Relearning(DAY),
        (Relearning(duration), true) => {
            if duration >= &(DAY * 4) {
                Reviewing(WEEK)
            } else {
                Relearning(*duration + DAY)
            }
        }
        (Relearning(_), false) => Relearning(DAY),
    };
}
