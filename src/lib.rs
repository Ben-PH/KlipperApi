use std::{collections::{HashMap, HashSet}, fmt::Display};
pub mod objects;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct ApiMethod {
    id: Option<u32>,
    method: Method,
}


#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub enum Method {
    GCode(GCodeMethod),
    // Report(ReportMethod),
    // Obects(ObjectsMethod),
    // Info(Option<HashMap<String, String>>),
    // EStop,
    // // RegisterRemoteMethod(),
    // QueryEndstopStatus,
}


#[derive(Debug, serde::Deserialize, serde::Serialize)]
enum GCodeMethod {
    Help,
    Script(String),
    Restart,
    FirmwareRestart,
    SubstribeOutput,
}

#[derive(Debug)]
enum ReportMethod {
    DumpStepper,
    DumpTrapq,
    DumpAdxl345,
    DumpAngle,
}

#[derive(Debug)]
enum ObjectsMethod {
    List,
    Query(HashMap<objects::StatusReferenceObjects, Vec<String>>),
    Subscribe(HashMap<objects::StatusReferenceObjects, Vec<String>>),
}


