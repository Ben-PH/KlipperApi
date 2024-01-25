use std::collections::{HashMap, HashSet};
pub mod objects;


enum GcodeMethod {
    Help,
    Script(String),
    Restart,
    FirmwareRestart,
    SubstribeOutput,
}

enum ReportMethod {
    DumpStepper,
    DumpTrapq,
    DumpAdxl345,
    DumpAngle,
}

enum ObjectsMethod {
    List,
    Query(HashMap<objects::StatusReferenceObjects, Vec<String>>),
    Subscribe(HashMap<objects::StatusReferenceObjects, Vec<String>>),
}

enum GenericMethod {
    Info(Option<HashMap<String, String>>),
    EStop,
    // RegisterRemoteMethod(),
    QueryEndstopStatus,
}


