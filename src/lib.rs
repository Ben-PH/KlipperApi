use serde_json::{json, Value};
pub mod method_groups;
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

/// Top level method menu.
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub enum Method {
    GCode(method_groups::GCodeMethod),
    Report(method_groups::ReportMethod),
    Objects(method_groups::ObjectsMethod),
    SetStatus(method_groups::StatusMethod),
    // RegisterRemoteMethod(),
}

impl Method {
    fn get_method_val(&self) -> &'static str {
        match self {
            // Method::RegisterRemoteMethod => "register_remote_method",
            Method::Objects(obj) => match obj {
                method_groups::ObjectsMethod::List => "objects/list",
                // method_groups::ObjectsMethod::Query(_) => "objects/query",
                // method_groups::ObjectsMethod::Subscribe(_) => "objects/subscribe",
            },
            Method::GCode(gcode) => match gcode {
                method_groups::GCodeMethod::Help => "gcode/help",
                method_groups::GCodeMethod::Script { .. } => "gcode/script",
                method_groups::GCodeMethod::Restart => "gcode/restart",
                method_groups::GCodeMethod::FirmwareRestart => "gcode/firmware_restart",
                method_groups::GCodeMethod::SubscribeOutput => "gcode/subscribe_output",
            },
            Method::Report(rep) => match rep {
                method_groups::ReportMethod::DumpStepper => "motion_report/dump_stepper",
                method_groups::ReportMethod::DumpTrapq => "motion_report/dump_trapq",
                method_groups::ReportMethod::DumpAdxl345 => "adxl345/dump_adxl345",
                method_groups::ReportMethod::DumpAngle => "angle/dump_angle",
                method_groups::ReportMethod::QueryEndstopStatus => "query_endstop_staus",
            },
            Method::SetStatus(stat) => match stat {
                method_groups::StatusMethod::Info(_) => "info",
                method_groups::StatusMethod::EStop => "emergency_stop",
                method_groups::StatusMethod::Cancel => "pause_resume/cancel",
                method_groups::StatusMethod::Pause => "pause_resume/pause",
                method_groups::StatusMethod::Resume => "pause_resume/resume",
            },
        }
    }

    fn get_params(&self) -> Option<Value> {
        match self {
            // Method::RegisterRemoteMethod => todo!(),
            Method::Objects(obj) => match obj {
                method_groups::ObjectsMethod::List => todo!(),
                // method_groups::ObjectsMethod::Query(_) => todo!(),
                // method_groups::ObjectsMethod::Subscribe(_) => todo!(),
            },
            Method::GCode(inner) => match inner {
                method_groups::GCodeMethod::Script { script: code } => {
                    Some(json!({ "script": code }))
                }
                method_groups::GCodeMethod::SubscribeOutput => todo!(),
                method_groups::GCodeMethod::Help
                | method_groups::GCodeMethod::Restart
                | method_groups::GCodeMethod::FirmwareRestart => None,
            },
            Method::Report(rep) => match rep {
                method_groups::ReportMethod::DumpStepper => todo!(),
                method_groups::ReportMethod::DumpTrapq => todo!(),
                method_groups::ReportMethod::DumpAdxl345 => todo!(),
                method_groups::ReportMethod::DumpAngle => todo!(),
                method_groups::ReportMethod::QueryEndstopStatus => None,
            },
            Method::SetStatus(method_groups::StatusMethod::Info(Some(map))) => {
                if map.is_empty() {
                    return None;
                }
                todo!("info with client details not yet implemented")
                // Some(json!({"client_info": {map}}))
            }
            Method::SetStatus(_) => None,
        }
    }
}

#[cfg(test)]
mod test {
    use serde_json::json;

    use crate::method_groups::GCodeMethod;

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
