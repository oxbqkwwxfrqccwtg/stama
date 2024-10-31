use serde::{Serialize, Deserialize};
#[macro_export]
macro_rules! build_event {
    ($($name:ident, $salt:expr, $namespace:ident, $category:ident),* $(,)?) => {
        #[repr(u32)]
        #[derive(Debug)]
        pub enum Event {
            $(
                $name = build_event_id($salt as i8, EventObject::$namespace as i8, EventStatus::$category as i8),
            )*
        }

        impl Event {
            pub fn lookup(object: &EventObject, status: &EventStatus) -> Option<Self> {
                match (&object, &status) {
                    $(
                        (EventObject::$namespace, EventStatus::$category) => Some(Event::$name),
                    )*
                    _ => None
                }
            }
        }
    };
}


const fn build_event_id(id: i8, namespace: i8, category: i8) -> u32 {
    ((id as u32) << 24) | ((namespace as u32) << 16) | ((category as u32) << 8)
}


build_event! {
    ActivityScheduled,      1,  Activity,      Scheduled,
    ActivityScheduleFailed, 2,  Activity,      Failed,
    ActivityStarted,        3,  Activity,      Started,
    ActivitySucceeded,      4,  Activity,      Succeeded,
    ActivityTimedOut,       5,  Activity,      TimedOut,
    ChoiceStateEntered,     6,  Choice,        StateEntered,
    ChoiceStateExited,      7,  Choice,        StateExited,
    ExecutionAborted,       8,  Execution,     Aborted,
    ExecutionFailed,        9,  Execution,     Failed,
    ExecutionRedriven,      10, Execution,     Redriven,
    ExecutionStarted,       11, Execution,     Started,
    ExecutionSucceeded,     12, Execution,     Succeeded,
    ExecutionTimedOut,      13, Execution,     TimedOut,
    FailStateEntered,       14, Fail,          StateEntered,
    MapIterationAborted,    15, MapIteration,  Aborted,
    MapIterationFailed,     16, MapIteration,  Failed,
    MapIterationStarted,    17, MapIteration,  Started,
    MapIterationSucceeded,  18, MapIteration,  Succeeded,
    MapRunAborted,          19, MapRun,        Aborted,
    MapRunFailed,           20, MapRun,        Failed,
    MapRunRedriven,         21, MapRun,        Redriven,
    MapRunStarted,          22, MapRun,        Started,
    MapRunSucceeded,        23, MapRun,        Succeeded,
    MapStateAborted,        24, Map,           StateAborted,
    MapStateEntered,        25, Map,           StateEntered,
    MapStateExited,         26, Map,           StateExited,
    MapStateFailed,         27, Map,           StateFailed,
    MapStateStarted,        28, Map,           StateStarted,
    MapStateSucceeded,      29, Map,           StateSucceeded,
    ParallelStateAborted,   30, Parallel,      StateAborted,
    ParallelStateEntered,   31, Parallel,      StateEntered,
    ParallelStateExited,    32, Parallel,      StateExited,
    ParallelStateFailed,    33, Parallel,      StateFailed,
    ParallelStateStarted,   34, Parallel,      StateStarted,
    ParallelStateSucceeded, 35, Parallel,      StateSucceeded,
    PassStateEntered,       36, Pass,          StateEntered,
    PassStateExited,        37, Pass,          StateExited,
    SucceedStateEntered,    38, Succeed,       StateEntered,
    SucceedStateExited,     39, Succeed,       StateExited,
    TaskFailed,             40, Task,          Failed,
    TaskScheduled,          41, Task,          Scheduled,
    TaskStarted,            42, Task,          Started,
    TaskStartFailed,        43, Task,          StartFailed,
    TaskStateAborted,       44, Task,          StateAborted,
    TaskStateEntered,       45, Task,          StateEntered,
    TaskStateExited,        46, Task,          StateExited,
    TaskSubmitFailed,       47, TaskSubmit,    Failed,
    TaskSubmitted,          48, Task,          Submitted,
    TaskSucceeded,          49, Task,          Succeeded,
    TaskTimedOut,           50, Task,          TimedOut,
    WaitStateAborted,       51, Wait,          StateAborted,
    WaitStateEntered,       52, Wait,          StateEntered,
    WaitStateExited,        53, Wait,          StateExited,
}


#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum EventObject {
    Activity = 1,
    Choice = 2,
    Execution = 3,
    Fail = 4,
    MapIteration = 6,
    Map = 7,
    Parallel = 8,
    Pass = 9,
    Succeed = 10,
    Task = 11,
    TaskStart = 12,
    TaskSubmit = 13,
    Wait = 14,
    MapRun = 15,
}


pub enum EventStatus {
    Aborted = 1,
    Entered = 2,
    Exited = 3,
    Failed = 4,
    Redriven = 5,
    Scheduled = 6,
    Started = 7,
    Submitted = 8,
    Succeeded = 9,
    TimedOut = 10,
    StateAborted = 13,
    StateEntered = 14,
    StateExited = 15,
    StateFailed = 16,
    StateStarted = 17,
    StateSucceeded = 18,
    StartFailed = 19,
}


pub enum Record<'a> {
    Orig {
        r#type: Event,
        payload: Option<serde_json::Value>
    },
    Ref  {
        r#type: Event,
        payload: Option<&'a serde_json::Value>
    }
}

