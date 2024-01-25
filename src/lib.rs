use std::collections::HashMap;

use serde_json::{Value, json};
pub mod objects;

#[derive(Debug, serde::Deserialize)]
pub struct ApiMethod {
    pub id: Option<u32>,
    #[serde(flatten)]
    pub method: Method,
}

#[derive(Debug, serde::Serialize)]
pub struct JsonApiStruct {
    id: Option<u32>,
    method: &'static str,
    params: Option<Value>,
}

impl From<ApiMethod> for JsonApiStruct {
    fn from(value: ApiMethod) -> Self {
        let id = value.id;
        let params = value.method.get_params();
        Self {id, method: value.method.get_method_str(), params}
    }
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
impl Method {
    fn get_method_str(&self) -> &'static str {
        match self {
            Method::GCode(_) => "script/gcode",
            _ => unimplemented!()
        }
    }

    fn get_params(&self) -> Option<Value> {
        match self {
            Method::GCode(GCodeMethod::Script{ script: code }) => Some(json!({ "script": code })),
            _ => unimplemented!()
        }
    }
}


#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub enum GCodeMethod {
    Help,

    Script{ script: String },
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


#[cfg(test)]
mod test {
    use serde_json::json;

    use super::*;
    #[test]
    fn gcode_to_json() {
        let gmeth = ApiMethod{ id: Some(1), method: Method::GCode(GCodeMethod::Script{script: "G28".to_string()}) };

        let as_j = JsonApiStruct::from(gmeth);
        assert_eq!(json!({"id": 1, "method": "script/gcode", "params": {"script": "G28"}}), json!(as_j));
    }
}
