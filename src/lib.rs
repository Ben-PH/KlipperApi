use std::collections::{HashMap, HashSet};
pub mod objects;

pub struct ApiMethod {
    id: Option<u32>,
    method: Method,
}

pub enum Method {
    GCode(GCodeMethod),
    Report(ReportMethod),
    Obects(ObjectsMethod),
    Info(Option<HashMap<String, String>>),
    EStop,
    // RegisterRemoteMethod(),
    QueryEndstopStatus,
}


enum GCodeMethod {
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


