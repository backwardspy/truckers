pub enum Channel {
    WorldPlacement,
    LocalLinearVelocity,
    LocalAngularVelocity,
    LocalLinearAcceleration,
    LocalAngularAcceleration,
    CabinOffset,
    CabinAngularVelocity,
    CabinAngularAcceleration,
    HeadOffset,
    Speed,
    EngineRpm,
    EngineGear,
    DisplayedGear,
    InputSteering,
    InputThrottle,
    InputBrake,
    InputClutch,
    EffectiveSteering,
    EffectiveThrottle,
    EffectiveBrake,
    EffectiveClutch,
    CruiseControl,
    HshifterSlot,
    HshifterSelector,
    ParkingBrake,
    MotorBrake,
    RetarderLevel,
    BrakeAirPressure,
    BrakeAirPressureWarning,
    BrakeAirPressureEmergency,
    BrakeTemperature,
    Fuel,
    FuelWarning,
    FuelAverageConsumption,
    FuelRange,
    Adblue,
    AdblueWarning,
    AdblueAverageConsumption,
    OilPressure,
    OilPressureWarning,
    OilTemperature,
    WaterTemperature,
    WaterTemperatureWarning,
    BatteryVoltage,
    BatteryVoltageWarning,
    ElectricEnabled,
    EngineEnabled,
    Lblinker,
    Rblinker,
    HazardWarning,
    LightLblinker,
    LightRblinker,
    LightParking,
    LightLowBeam,
    LightHighBeam,
    LightAuxFront,
    LightAuxRoof,
    LightBeacon,
    LightBrake,
    LightReverse,
    Wipers,
    DashboardBacklight,
    DifferentialLock,
    LiftAxle,
    LiftAxleIndicator,
    TrailerLiftAxle,
    TrailerLiftAxleIndicator,
    WearEngine,
    WearTransmission,
    WearCabin,
    WearChassis,
    WearWheels,
    Odometer,
    NavigationDistance,
    NavigationTime,
    NavigationSpeedLimit,
    WheelSuspDeflection,
    WheelOnGround,
    WheelSubstance,
    WheelVelocity,
    WheelSteering,
    WheelRotation,
    WheelLift,
    WheelLiftOffset,
}

impl std::fmt::Display for Channel {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::WorldPlacement => write!(f, "truck.world.placement"),
            Self::LocalLinearVelocity => write!(f, "truck.local.velocity.linear"),
            Self::LocalAngularVelocity => write!(f, "truck.local.velocity.angular"),
            Self::LocalLinearAcceleration => write!(f, "truck.local.acceleration.linear"),
            Self::LocalAngularAcceleration => write!(f, "truck.local.acceleration.angular"),
            Self::CabinOffset => write!(f, "truck.cabin.offset"),
            Self::CabinAngularVelocity => write!(f, "truck.cabin.velocity.angular"),
            Self::CabinAngularAcceleration => write!(f, "truck.cabin.acceleration.angular"),
            Self::HeadOffset => write!(f, "truck.head.offset"),
            Self::Speed => write!(f, "truck.speed"),
            Self::EngineRpm => write!(f, "truck.engine.rpm"),
            Self::EngineGear => write!(f, "truck.engine.gear"),
            Self::DisplayedGear => write!(f, "truck.displayed.gear"),
            Self::InputSteering => write!(f, "truck.input.steering"),
            Self::InputThrottle => write!(f, "truck.input.throttle"),
            Self::InputBrake => write!(f, "truck.input.brake"),
            Self::InputClutch => write!(f, "truck.input.clutch"),
            Self::EffectiveSteering => write!(f, "truck.effective.steering"),
            Self::EffectiveThrottle => write!(f, "truck.effective.throttle"),
            Self::EffectiveBrake => write!(f, "truck.effective.brake"),
            Self::EffectiveClutch => write!(f, "truck.effective.clutch"),
            Self::CruiseControl => write!(f, "truck.cruise_control"),
            Self::HshifterSlot => write!(f, "truck.hshifter.slot"),
            Self::HshifterSelector => write!(f, "truck.hshifter.select"),
            Self::ParkingBrake => write!(f, "truck.brake.parking"),
            Self::MotorBrake => write!(f, "truck.brake.motor"),
            Self::RetarderLevel => write!(f, "truck.brake.retarder"),
            Self::BrakeAirPressure => write!(f, "truck.brake.air.pressure"),
            Self::BrakeAirPressureWarning => write!(f, "truck.brake.air.pressure.warning"),
            Self::BrakeAirPressureEmergency => write!(f, "truck.brake.air.pressure.emergency"),
            Self::BrakeTemperature => write!(f, "truck.brake.temperature"),
            Self::Fuel => write!(f, "truck.fuel.amount"),
            Self::FuelWarning => write!(f, "truck.fuel.warning"),
            Self::FuelAverageConsumption => write!(f, "truck.fuel.consumption.average"),
            Self::FuelRange => write!(f, "truck.fuel.range"),
            Self::Adblue => write!(f, "truck.adblue"),
            Self::AdblueWarning => write!(f, "truck.adblue.warning"),
            Self::AdblueAverageConsumption => write!(f, "truck.adblue.consumption.average"),
            Self::OilPressure => write!(f, "truck.oil.pressure"),
            Self::OilPressureWarning => write!(f, "truck.oil.pressure.warning"),
            Self::OilTemperature => write!(f, "truck.oil.temperature"),
            Self::WaterTemperature => write!(f, "truck.water.temperature"),
            Self::WaterTemperatureWarning => write!(f, "truck.water.temperature.warning"),
            Self::BatteryVoltage => write!(f, "truck.battery.voltage"),
            Self::BatteryVoltageWarning => write!(f, "truck.battery.voltage.warning"),
            Self::ElectricEnabled => write!(f, "truck.electric.enabled"),
            Self::EngineEnabled => write!(f, "truck.engine.enabled"),
            Self::Lblinker => write!(f, "truck.lblinker"),
            Self::Rblinker => write!(f, "truck.rblinker"),
            Self::HazardWarning => write!(f, "truck.hazard.warning"),
            Self::LightLblinker => write!(f, "truck.light.lblinker"),
            Self::LightRblinker => write!(f, "truck.light.rblinker"),
            Self::LightParking => write!(f, "truck.light.parking"),
            Self::LightLowBeam => write!(f, "truck.light.beam.low"),
            Self::LightHighBeam => write!(f, "truck.light.beam.high"),
            Self::LightAuxFront => write!(f, "truck.light.aux.front"),
            Self::LightAuxRoof => write!(f, "truck.light.aux.roof"),
            Self::LightBeacon => write!(f, "truck.light.beacon"),
            Self::LightBrake => write!(f, "truck.light.brake"),
            Self::LightReverse => write!(f, "truck.light.reverse"),
            Self::Wipers => write!(f, "truck.wipers"),
            Self::DashboardBacklight => write!(f, "truck.dashboard.backlight"),
            Self::DifferentialLock => write!(f, "truck.differential_lock"),
            Self::LiftAxle => write!(f, "truck.lift_axle"),
            Self::LiftAxleIndicator => write!(f, "truck.lift_axle.indicator"),
            Self::TrailerLiftAxle => write!(f, "truck.trailer.lift_axle"),
            Self::TrailerLiftAxleIndicator => write!(f, "truck.trailer.lift_axle.indicator"),
            Self::WearEngine => write!(f, "truck.wear.engine"),
            Self::WearTransmission => write!(f, "truck.wear.transmission"),
            Self::WearCabin => write!(f, "truck.wear.cabin"),
            Self::WearChassis => write!(f, "truck.wear.chassis"),
            Self::WearWheels => write!(f, "truck.wear.wheels"),
            Self::Odometer => write!(f, "truck.odometer"),
            Self::NavigationDistance => write!(f, "truck.navigation.distance"),
            Self::NavigationTime => write!(f, "truck.navigation.time"),
            Self::NavigationSpeedLimit => write!(f, "truck.navigation.speed.limit"),
            Self::WheelSuspDeflection => write!(f, "truck.wheel.suspension.deflection"),
            Self::WheelOnGround => write!(f, "truck.wheel.on_ground"),
            Self::WheelSubstance => write!(f, "truck.wheel.substance"),
            Self::WheelVelocity => write!(f, "truck.wheel.angular_velocity"),
            Self::WheelSteering => write!(f, "truck.wheel.steering"),
            Self::WheelRotation => write!(f, "truck.wheel.rotation"),
            Self::WheelLift => write!(f, "truck.wheel.lift"),
            Self::WheelLiftOffset => write!(f, "truck.wheel.lift.offset"),
        }
    }
}