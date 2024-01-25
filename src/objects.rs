use std::collections::{HashMap, HashSet};


pub enum StatusReferenceObjects {
    Angle,
    BedMesh(BedMeshProfiles),
    BedScrews(BedScrews),
    Configfile(Configfile),
    DisplayStatus(DisplayStatus),
    EndstopPhase(EndstopPhase),
    // ExcludeObject(ExcludeObject),
    ExtruderStepper(ExtruderStepper),
    Fan(Fan),
    FilamentSwitchSensor(FilamentSwitchSensor),
    FilamentMotionSensor(FilamentMotionSensor),
    FirmwareRetraction(FirmwareRetraction),
    Gcode(Gcode),
    GcodeButton(GcodeButton),
    GcodeMacro(GcodeMacro),
    GcodeMove(GcodeMove),
    // HallFilamentWidthSensor(HallFilamentWidthSensor),
    Heater(Heater),
    Heaters(Heaters),
    IdleTimeout(IdleTimeout),
    Led(Led),
    ManualProbe(ManualProbe),
    Mcu(Mcu),
    MotionReport(MotionReport),
    OutputPin(OutputPin),
    Palette2(Palette2),
    PauseResume(PauseResume),
    PrintStats(PrintStats),
    Probe(Probe),
    PwmCycleTime(PwmCycleTime),
    QuadGantryLevel(QuadGantryLevel),
    QueryEndstops(QueryEndstops),
    ScrewsTiltAdjust(ScrewsTiltAdjust),
    Servo(Servo),
    StepperEnable(StepperEnable),
    SystemStats(SystemStats),
    TemperatureFan(TemperatureFan),
    TemperatureSensor(TemperatureSensor),
    TmcDrivers(TmcDrivers),
    Toolhead(Toolhead),
    DualCarriage(DualCarriage),
    VirtualSdcard(VirtualSdcard),
    Webhooks(Webhooks),
    ZThermalAdjust(ZThermalAdjust),
    ZTilt(ZTilt),
}

pub struct Angle {
    temp: f32,
}

struct BedScrews {
    is_active: bool,
    state: String,
    current_screw: usize,
    accepted_screws: u32,
}

pub struct BedMeshProfiles{ 
    all: HashSet<String>,
    current: BedMesh,
}
pub struct BedMesh {
    profile_name: String,
    mesh_min: f32,
    mesh_max: f32,
    probed_matrix: (),
    mesh_matrix: (),
}

struct Configfile {
    settings: HashMap<String, String>,
    config: HashMap<String, String>,
    save_config_pending: bool,
    save_config_pending_items: Vec<String>,
    warnings: Vec<ConfigWarning>,
}


struct ConfigWarning {
    warning_type: String,
    message: String,
    // Additional fields if needed based on the types of warnings
}

pub struct DisplayStatus {
    progress: String,
    message: String,
}

pub struct EndstopPhase {
    last_home: HashMap<String, EndstopHomeInfo>,
}

pub struct EndstopHomeInfo {
    phase: i32,
    phases: i32,
    mcu_position: i32,
}

pub struct Polygon {
    coords: [[f32; 2]; 4],
    name: String,
    center: [f32; 2],
}
// impl Hash for Polygon {
// }

// TODO: make Polygon hashable by name
pub struct ExcludeObjects {
    names: HashSet<String>
}

pub struct ExtruderStepper {
    pressure_advance: f32,
    smooth_time: f32,
    motion_queue: Option<String>,
}

pub struct CurrentObject {
    name: String
}

pub struct Fan {
    speed: f32,
    rpm: u32,
}

pub struct FilamentSwitchSensor {
    enabled: bool,
    filament_detected: bool,
}

pub struct FilamentMotionSensor {
    enabled: bool,
    filament_detected: bool,
}

pub struct FirmwareRetraction {
    retract_length: f32,
    retract_speed: f32,
    unretract_extra_length: f32,
    unretract_speed: f32,
}


pub struct Gcode {
    commands: Vec<String>,
}

pub struct GcodeButton {
    state: String,
}

pub struct GcodeMacro {
    // Assuming Gcode macros can have multiple variables with dynamic names and values
    variables: HashMap<String, String>,
}

pub struct GCodePos {
    position: Pos,
    extruder: f32,
}

pub struct Pos {
    x: f32,
    y: f32,
    z: f32,
}

pub struct GcodeMove {
    gcode_position: GCodePos,
    position: GCodePos,
    homing_origin: Pos,
    speed: f32,
    speed_factor: f32,
    extrude_factor: f32,
    absolute_coordinates: bool,
    absolute_extrude: bool,
}

pub struct Heater {
    temperature: f32,
    target: f32,
    power: f32,
    can_extrude: bool,
}

pub struct Heaters {
    available_heaters: Vec<String>,
    available_sensors: Vec<String>,
    available_monitors: Vec<String>,
}

pub struct IdleTimeout {
    state: String,
    printing_time: f64,
}

pub struct Led {
    color_data: Vec<[f32; 4]>, // RGBW values
}

pub struct ManualProbe {
    is_active: bool,
    z_position: f32,
    z_position_lower: f32,
    z_position_upper: f32,
}

pub struct Mcu {
    mcu_version: String,
    mcu_build_versions: String,
    mcu_constants: HashMap<String, String>,
    last_stats: HashMap<String, String>,
}

pub struct MotionReport {
    live_position: [f32; 4],
    live_velocity: f32,
    live_extruder_velocity: f32,
}

pub struct OutputPin {
    value: f32,
}

pub struct Palette2 {
    ping: f32,
    remaining_load_length: f32,
    is_splicing: bool,
}

pub struct PauseResume {
    is_paused: bool,
}

pub struct PrintStats {
    filename: String,
    total_duration: f64,
    print_duration: f64,
    filament_used: f32,
    state: String,
    message: String,
    info: PrintInfo,
}

pub struct PrintInfo {
    total_layer: i32,
    current_layer: i32,
}

pub struct Probe {
    name: String,
    last_query: bool,
    last_z_result: f32,
}

pub struct PwmCycleTime {
    value: f32,
}

pub struct QuadGantryLevel {
    applied: bool,
}

pub struct QueryEndstops {
    last_query: HashMap<String, bool>,
}

pub struct ScrewsTiltAdjust {
    error: bool,
    max_deviation: f32,
    results: HashMap<String, ScrewAdjustInfo>,
}

pub struct ScrewAdjustInfo {
    z: f32,
    sign: String,
    adjust: String,
    is_base: bool,
}

pub struct Servo {
    value: f32,
}

pub struct StepperEnable {
    steppers: HashMap<String, bool>,
}

pub struct SystemStats {
    sysload: f32,
    cputime: f32,
    memavail: u32,
}

pub struct TemperatureFan {
    temperature: f32,
    target: f32,
}

pub struct TemperatureSensor {
    temperature: f32,
    measured_min_temp: f32,
    measured_max_temp: f32,
}

pub struct TmcDrivers {
    mcu_phase_offset: Option<i32>,
    phase_offset_position: Option<f32>,
    drv_status: HashMap<String, String>,
    temperature: Option<f32>,
    run_current: f32,
    hold_current: f32,
}

pub struct Toolhead {
    position: [f32; 4],
    extruder: String,
    homed_axes: String,
    axis_minimum: [f32; 3],
    axis_maximum: [f32; 3],
    max_velocity: f32,
    max_accel: f32,
    max_accel_to_decel: f32,
    square_corner_velocity: f32,
    stalls: u32,
}

pub struct DualCarriage {
    carriage_0: String,
    carriage_1: String,
}

pub struct VirtualSdcard {
    is_active: bool,
    progress: f32,
    file_path: String,
    file_position: u64,
    file_size: u64,
}

pub struct Webhooks {
    state: String,
    state_message: String,
}

pub struct ZThermalAdjust {
    enabled: bool,
    temperature: f32,
    measured_min_temp: f32,
    measured_max_temp: f32,
    current_z_adjust: f32,
    z_adjust_ref_temperature: f32,
}

pub struct ZTilt {
    applied: bool,
}
