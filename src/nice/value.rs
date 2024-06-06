use truckers_scssdk_sys::*;

#[derive(Debug, Clone)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

#[derive(Debug, Clone)]
pub struct Euler {
    pub heading: f32,
    pub pitch: f32,
    pub roll: f32,
}

#[derive(Debug, Clone)]
pub enum Value {
    Bool(bool),
    I32(i32),
    I64(i64),
    U32(u32),
    U64(u64),
    F32(f32),
    F64(f64),
    FVector(Vec3<f32>),
    DVector(Vec3<f64>),
    Euler(Euler),
    FPlacement(Vec3<f32>, Euler),
    DPlacement(Vec3<f64>, Euler),
    String(String),
}

#[derive(Debug, Clone)]
pub struct NamedValue {
    pub name: String,
    pub value: Value,
}

impl From<scs_value_t> for Value {
    fn from(v: scs_value_t) -> Self {
        unsafe {
            match v.type_ {
                SCS_VALUE_TYPE_bool => {
                    let v: bool = v.__bindgen_anon_1.value_bool.value != 0;
                    Self::Bool(v)
                }
                SCS_VALUE_TYPE_s32 => {
                    let v: i32 = v.__bindgen_anon_1.value_s32.value;
                    Self::I32(v)
                }
                SCS_VALUE_TYPE_s64 => {
                    let v: i64 = v.__bindgen_anon_1.value_s64.value;
                    Self::I64(v)
                }
                SCS_VALUE_TYPE_u32 => {
                    let v: u32 = v.__bindgen_anon_1.value_u32.value;
                    Self::U32(v)
                }
                SCS_VALUE_TYPE_u64 => {
                    let v: u64 = v.__bindgen_anon_1.value_u64.value;
                    Self::U64(v)
                }
                SCS_VALUE_TYPE_float => {
                    let v: f32 = v.__bindgen_anon_1.value_float.value;
                    Self::F32(v)
                }
                SCS_VALUE_TYPE_double => {
                    let v: f64 = v.__bindgen_anon_1.value_double.value;
                    Self::F64(v)
                }
                SCS_VALUE_TYPE_fvector => {
                    let v = v.__bindgen_anon_1.value_fvector;
                    Self::FVector(Vec3 {
                        x: v.x,
                        y: v.y,
                        z: v.z,
                    })
                }
                SCS_VALUE_TYPE_dvector => {
                    let v = v.__bindgen_anon_1.value_dvector;
                    Self::DVector(Vec3 {
                        x: v.x,
                        y: v.y,
                        z: v.z,
                    })
                }
                SCS_VALUE_TYPE_euler => {
                    let v = v.__bindgen_anon_1.value_euler;
                    Self::Euler(Euler {
                        heading: v.heading,
                        pitch: v.pitch,
                        roll: v.roll,
                    })
                }
                SCS_VALUE_TYPE_fplacement => {
                    let v = v.__bindgen_anon_1.value_fplacement;
                    Self::FPlacement(
                        Vec3 {
                            x: v.position.x,
                            y: v.position.y,
                            z: v.position.z,
                        },
                        Euler {
                            heading: v.orientation.heading,
                            pitch: v.orientation.pitch,
                            roll: v.orientation.roll,
                        },
                    )
                }
                SCS_VALUE_TYPE_dplacement => {
                    let v = v.__bindgen_anon_1.value_dplacement;
                    Self::DPlacement(
                        Vec3 {
                            x: v.position.x,
                            y: v.position.y,
                            z: v.position.z,
                        },
                        Euler {
                            heading: v.orientation.heading,
                            pitch: v.orientation.pitch,
                            roll: v.orientation.roll,
                        },
                    )
                }
                SCS_VALUE_TYPE_string => {
                    let v = std::ffi::CStr::from_ptr(v.__bindgen_anon_1.value_string.value);
                    Self::String(v.to_string_lossy().into_owned())
                }
                _ => unreachable!(),
            }
        }
    }
}

impl From<scs_named_value_t> for NamedValue {
    fn from(value: scs_named_value_t) -> Self {
        let name = unsafe {
            std::ffi::CStr::from_ptr(value.name)
                .to_string_lossy()
                .into_owned()
        };
        let value = Value::from(value.value);
        Self { name, value }
    }
}
