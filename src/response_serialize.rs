use masterpower_api::commands::qpigs::DeviceChargingStatus::{
    ChargingFromAC, ChargingFromSCC, ChargingFromSCCAndAC, NotCharging,
};
use masterpower_api::commands::qpigs::QPIGSResponse;
use serde::ser::{Serialize, SerializeStruct, Serializer};

pub struct JSONQPIGSResponse(QPIGSResponse);

impl Serialize for JSONQPIGSResponse {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("QPIGSResponse", 17)?;
        state.serialize_field("grid_voltage", &self.0.grid_voltage)?;
        state.serialize_field("grid_frequency", &self.0.grid_frequency)?;
        state.serialize_field("ac_out_voltage", &self.0.ac_out_voltage)?;
        state.serialize_field("ac_out_frequency", &self.0.ac_out_frequency)?;
        state.serialize_field("ac_out_apparent_power", &self.0.ac_out_apparent_power)?;
        state.serialize_field("ac_out_active_power", &self.0.ac_out_active_power)?;
        state.serialize_field("out_load_percent", &self.0.out_load_percent)?;
        state.serialize_field("bus_voltage", &self.0.bus_voltage)?;
        state.serialize_field("battery_voltage", &self.0.battery_voltage)?;
        state.serialize_field("battery_charge_current", &self.0.battery_charge_current)?;
        state.serialize_field("battery_capacity", &self.0.battery_capacity)?;
        state.serialize_field("inverter_heat_sink_temp", &self.0.inverter_heat_sink_temp)?;
        state.serialize_field("pv_input_current", &self.0.pv_input_current)?;
        state.serialize_field("pv_input_voltage", &self.0.pv_input_voltage)?;
        state.serialize_field("battery_scc_voltage", &self.0.battery_scc_voltage)?;
        state.serialize_field(
            "battery_discharge_current",
            &self.0.battery_discharge_current,
        )?;
        state.serialize_field(
            "device_status_active_load",
            &self.0.device_status.active_load,
        )?;
        state.serialize_field(
            "device_status_charge_status",
            match &self.0.device_status.charge_status {
                NotCharging => "not_charging",
                ChargingFromSCC => "charging_from_scc",
                ChargingFromAC => "charging_from_ac",
                ChargingFromSCCAndAC => "charging_from_scc_and_ac",
            },
        )?;
        state.end()
    }
}

impl From<QPIGSResponse> for JSONQPIGSResponse {
    fn from(value: QPIGSResponse) -> Self {
        Self(value)
    }
}
