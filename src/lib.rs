use std::collections::HashMap;

use serde_json::{json, Value};
pub mod objects;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ApiMethod {
    pub id: Option<u32>,
    #[serde(flatten)]
    pub method: Method,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct JsonApiStruct {
    id: Option<u32>,
    method: String,
    params: Option<Value>,
}

impl From<ApiMethod> for JsonApiStruct {
    fn from(value: ApiMethod) -> Self {
        let id = value.id;
        let params = value.method.get_params();
        Self {
            id,
            method: value.method.get_method_val().to_string(),
            params,
        }
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub enum Method {
    GCode(GCodeMethod),
    Report(ReportMethod),
    Objects(ObjectsMethod),
    /// Clients are encouraged to provide the name of the client and its software version when first connecting to the Klipper API server. 
    Info(Option<HashMap<String, String>>),
    EStop,
    Cancel,
    Pause,
    Resume,
    // RegisterRemoteMethod(),
    QueryEndstopStatus,
}
impl Method {
    fn get_method_val(&self) -> &'static str {
        match self {
            Method::Info(_) => "info",
            Method::EStop => "emergency_stop",
            // Method::RegisterRemoteMethod => "register_remote_method",
            Method::Objects(obj) => match obj {
                ObjectsMethod::List => "objects/list",
                // ObjectsMethod::Query(_) => "objects/query",
                // ObjectsMethod::Subscribe(_) => "objects/subscribe",
            },
            Method::GCode(gcode) => match gcode {
                GCodeMethod::Help => "gcode/help",
                GCodeMethod::Script { .. } => "gcode/script",
                GCodeMethod::Restart => "gcode/restart",
                GCodeMethod::FirmwareRestart => "gcode/firmware_restart",
                GCodeMethod::SubscribeOutput => "gcode/subscribe_output",
            },
            Method::Report(rep) => match rep {
                ReportMethod::DumpStepper => "motion_report/dump_stepper",
                ReportMethod::DumpTrapq => "motion_report/dump_trapq",
                ReportMethod::DumpAdxl345 => "adxl345/dump_adxl345",
                ReportMethod::DumpAngle => "angle/dump_angle",
            },
            Method::Pause => "pause_resume/pause",
            Method::Resume => "pause_resume/resume",
            Method::Cancel => "pause_resume/cancel",
            Method::QueryEndstopStatus => "query_endstop_staus",
        }
    }

    fn get_params(&self) -> Option<Value> {
        match self {
            // Method::RegisterRemoteMethod => todo!(),
            Method::Objects(obj) => match obj {
                ObjectsMethod::List => todo!(),
                // ObjectsMethod::Query(_) => todo!(),
                // ObjectsMethod::Subscribe(_) => todo!(),
            }
            Method::GCode(inner) => match inner {
                GCodeMethod::Script { script: code } => Some(json!({ "script": code })),
                GCodeMethod::SubscribeOutput => todo!(),
                GCodeMethod::Help | GCodeMethod::Restart | GCodeMethod::FirmwareRestart => None,
            }
            Method::Report(rep) => match rep {
                ReportMethod::DumpStepper => todo!(),
                ReportMethod::DumpTrapq => todo!(),
                ReportMethod::DumpAdxl345 => todo!(),
                ReportMethod::DumpAngle => todo!(),
            }
            Method::Info(Some(map)) => {
                if map.is_empty() {
                    return None;
                }
                todo!("info with client details not yet implemented")
                // Some(json!({"client_info": {map}}))
            }
            Method::Info(None) | Method::EStop | Method::Cancel | Method::Pause | Method::Resume | Method::QueryEndstopStatus  => None,

        }
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub enum GCodeMethod {
    Help,

    Script { script: String },
    Restart,
    FirmwareRestart,
    SubscribeOutput,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub enum ReportMethod {
    DumpStepper,
    DumpTrapq,
    DumpAdxl345,
    DumpAngle,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub enum ObjectsMethod {
    List,
    // Query(HashMap<objects::StatusReferenceObjects, Vec<String>>),
    // Subscribe(HashMap<objects::StatusReferenceObjects, Vec<String>>),
}

#[cfg(test)]
mod test {
    use serde_json::json;

    use super::*;
    #[test]
    fn gcode_to_json() {
        let gmeth = ApiMethod {
            id: Some(1),
            method: Method::GCode(GCodeMethod::Script {
                script: "G28".to_string(),
            }),
        };

        let as_j = JsonApiStruct::from(gmeth);
        assert_eq!(
            json!({"id": 1, "method": "script/gcode", "params": {"script": "G28"}}),
            json!(as_j)
        );
    }
}
