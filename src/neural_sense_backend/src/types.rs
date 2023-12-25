use candid::{Decode, Encode};
use ic_stable_structures::memory_manager::VirtualMemory;
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, Storable};
use std::borrow::Cow;

pub type Memory = VirtualMemory<DefaultMemoryImpl>;
pub type IdCell = Cell<u64, Memory>;

// struct representing a user profile
#[derive(candid::CandidType, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    pub user_id: u64,
    pub user_principal: String,
    pub user_name: String,
    pub user_email: String,
    pub contact_number: String,
    pub userdevices: Vec<DeviceConfiguration>,
}

// struct representing a prosthetic configuration
#[derive(candid::CandidType, Clone, Serialize, Deserialize)]
pub struct DeviceConfiguration {
    pub device_id: u64,
    pub device_name: String,
    pub device_type: String,
    pub device_description: String,
    pub device_status: String,
    pub device_config: String,
    pub research_data_id: u64,
}

// struct representing a research data
#[derive(candid::CandidType, Clone, Serialize, Deserialize)]
pub struct ResearchData {
    pub research_data_id: u64,
    pub researcher_principal: String,
    pub research_data_name: String,
    pub research_data_description: String,
    pub research_data_status: String,
    pub research_data_config: Vec<DeviceSettings>,  // Willhold the record of Device setting and later updates
}

// struct representing a device settings
#[derive(candid::CandidType, Clone, Serialize, Deserialize)]
pub struct DeviceSettings {
    pub device_settings_id: u64,
    pub power_consumption: u64,
    pub signal_frequency: u64,
    pub signal_type: String,
    pub compatability: Vec<String>, // List of Compatible Interfaces or systems
}



// Implementing the Storable and BoundedStorable trait for the UserProfile struct
impl Storable for UserProfile {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for UserProfile {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}


// Implementing the Storable and BoundedStorable trait for the DeviceConfiguration struct
impl Storable for DeviceConfiguration {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for DeviceConfiguration {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}


// Implementing the Storable and BoundedStorable trait for the ResearchData struct
impl Storable for ResearchData {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for ResearchData {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}


// Implementing the Storable and BoundedStorable trait for the DeviceSettings struct
impl Storable for DeviceSettings {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for DeviceSettings {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}


// payload for the  user profile
#[derive(candid::CandidType, Clone, Serialize, Deserialize)]
pub struct UserProfilePayload {
    pub user_name: String,
    pub user_email: String,
    pub contact_number: String,
}

// payload for the  device configuration
#[derive(candid::CandidType, Clone, Serialize, Deserialize)]
pub struct DeviceConfigurationPayload {
    pub device_name: String,
    pub device_type: String,
    pub device_description: String,
    pub device_status: String,
    pub device_config: String,
    pub research_data_id: u64,
}


// payload for the  research data
#[derive(candid::CandidType, Clone, Serialize, Deserialize)]
pub struct ResearchDataPayload {
    pub research_data_name: String,
    pub research_data_description: String,
    pub research_data_status: String,
}

// payload for the  device settings

#[derive(candid::CandidType, Clone, Serialize, Deserialize)]
pub struct DeviceSettingsPayload {
    pub power_consumption: u64,
    pub signal_frequency: u64,
    pub signal_type: String,
    pub compatability: Vec<String>, // List of Compatible Interfaces or systems
}