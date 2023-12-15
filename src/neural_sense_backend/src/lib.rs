#[macro_use]
extern crate serde;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager};
use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap};
use std::cell::RefCell;

mod types;
use types::*;

// thread local storage for the memory manager
thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static USER_PROFILE_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))), 0)
            .expect("Cannot create a counter")
    );

    static USER_PROFILE_STORAGE: RefCell<StableBTreeMap<u64, UserProfile, Memory>> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1))))
    );

    static DEVICE_CONFIGURATION_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2))), 0)
            .expect("Cannot create a counter")
    );

    static DEVICE_CONFIGURATION_STORAGE: RefCell<StableBTreeMap<u64, DeviceConfiguration, Memory>> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(3))))
    );

    static RESEARCH_DATA_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(4))), 0)
            .expect("Cannot create a counter")
    );

    static RESEARCH_DATA_STORAGE: RefCell<StableBTreeMap<u64, ResearchData, Memory>> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(5))))
    );

    static DEVICE_SETTINGS_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(6))), 0)
            .expect("Cannot create a counter")
    );

    static DEVICE_SETTINGS_STORAGE: RefCell<StableBTreeMap<u64, DeviceSettings, Memory>> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(7))))
    );




}

fn is_invalid_string(str: &String) -> bool{
    return str.trim().is_empty()
}
// function to create a user profile
#[ic_cdk::update]
fn add_user_profile(payload: UserProfilePayload) -> Result<UserProfile, Error> {
    //input validation check
    if is_invalid_string(&payload.user_name) || is_invalid_string(&payload.user_email) || is_invalid_string(&payload.contact_number){
        return Err(Error::NotFound {
            msg: "Invalid input Fill all Fields".to_string(),
        });
      
    }
    let id = USER_PROFILE_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("cannot increment id counter");
    let user_profile = UserProfile {
        user_id: id,
        user_name: payload.user_name,
        user_email: payload.user_email,
        contact_number: payload.contact_number,
        userdevices: Vec::new(),
    };
    USER_PROFILE_STORAGE.with(|s| {
        s.borrow_mut()
            .insert(id, user_profile.clone())
    });
    Ok(user_profile)
}

// function to update a user profile
#[ic_cdk::update]
fn update_user_profile(user_id: u64, payload: UserProfilePayload) -> Result<UserProfile, Error> {
    //input validation check
    if is_invalid_string(&payload.user_name) || is_invalid_string(&payload.user_email) || is_invalid_string(&payload.contact_number){
        return Err(Error::NotFound {
            msg: "Invalid input Fill all Fields".to_string(),
        });
      
    }
    let user_profile = UserProfile {
        user_id,
        user_name: payload.user_name,
        user_email: payload.user_email,
        contact_number: payload.contact_number,
        userdevices: Vec::new(),
    };
    USER_PROFILE_STORAGE.with(|storage| {
        storage
            .borrow_mut()
            .insert(user_id, user_profile.clone())
            .expect("cannot insert user profile")
    });
    Ok(user_profile)

}

// function to get a user profile
#[ic_cdk::query]
fn get_user_profile(user_id: u64) -> Result<UserProfile, Error> {
    USER_PROFILE_STORAGE.with(|storage| {
        storage.borrow_mut().get(&user_id).ok_or(Error::NotFound {
            msg: format!("user with id={} not found", user_id),
        })
    })
}

// function to delete a user profile
#[ic_cdk::update]
fn delete_user_profile(user_id: u64) -> Result<(), Error> {
    USER_PROFILE_STORAGE.with(|storage| {
        storage
            .borrow_mut()
            .remove(&user_id)
            .ok_or(Error::NotFound {
                msg: "User profile not found".to_string(),
            })
    })?;
    Ok(())
}

// function to get all user profiles
#[ic_cdk::query]
fn get_all_user_profiles() -> Vec<UserProfile> {
    USER_PROFILE_STORAGE.with(|storage| {
        storage
            .borrow()
            .iter()
            .map(|(_, user_profile)| user_profile.clone())
            .collect()
    })
}


// Device Configuration

// function to create a device configuration
#[ic_cdk::update]
fn add_device_configuration(payload: DeviceConfigurationPayload) -> Result<DeviceConfiguration, Error> {
    //input validation check
    if is_invalid_string(&payload.device_name)
        || is_invalid_string(&payload.device_type)
        || is_invalid_string(&payload.device_description)
        || is_invalid_string(&payload.device_status) 
        || is_invalid_string(&payload.device_config)
    {
            return Err(Error::NotFound {
                msg: "Invalid input Fill all Fields".to_string(),
            });
      
    }
    // ensures research data exists
    get_research_data(payload.research_data_id)?;
    let id = DEVICE_CONFIGURATION_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("cannot increment id counter");
    let device_configuration = DeviceConfiguration {
        device_id: id,
        device_name: payload.device_name,
        device_type: payload.device_type,
        device_description: payload.device_description,
        device_status: payload.device_status,
        device_config: payload.device_config,
        research_data_id: payload.research_data_id,
    };
    DEVICE_CONFIGURATION_STORAGE.with(|s| {
        s.borrow_mut()
            .insert(id, device_configuration.clone())
    });
    Ok(device_configuration)
}

// function to update a device configuration
#[ic_cdk::update]
fn update_device_configuration(device_id: u64, payload: DeviceConfigurationPayload) -> Result<DeviceConfiguration, Error> {
    //input validation check
    if is_invalid_string(&payload.device_name)
        || is_invalid_string(&payload.device_type)
        || is_invalid_string(&payload.device_description)
        || is_invalid_string(&payload.device_status) 
        || is_invalid_string(&payload.device_config)
    {
            return Err(Error::NotFound {
                msg: "Invalid input Fill all Fields".to_string(),
            });
      
    }
    // ensures research data exists
    get_research_data(payload.research_data_id)?;
    let device_configuration = DeviceConfiguration {
        device_id,
        device_name: payload.device_name,
        device_type: payload.device_type,
        device_description: payload.device_description,
        device_status: payload.device_status,
        device_config: payload.device_config,
        research_data_id: payload.research_data_id,
    };
    DEVICE_CONFIGURATION_STORAGE.with(|storage| {
        storage
            .borrow_mut()
            .insert(device_id, device_configuration.clone())
            .expect("cannot insert device configuration")
    });
    Ok(device_configuration)

}

// function to get a device configuration
#[ic_cdk::query]
fn get_device_configuration(device_id: u64) -> Result<DeviceConfiguration, Error> {
    DEVICE_CONFIGURATION_STORAGE.with(|storage| {
        storage.borrow_mut().get(&device_id).ok_or(Error::NotFound {
            msg: format!("device with id={} not found", device_id),
        })
    })
}

// function to delete a device configuration
#[ic_cdk::update]
fn delete_device_configuration(device_id: u64) -> Result<(), Error> {
    DEVICE_CONFIGURATION_STORAGE.with(|storage| {
        storage
            .borrow_mut()
            .remove(&device_id)
            .ok_or(Error::NotFound {
                msg: "Device configuration not found".to_string(),
            })
    })?;

    delete_device_configuration_from_user_profile(device_id);
    Ok(())
}

// delete a device configuration from a user profile
fn delete_device_configuration_from_user_profile(device_id: u64){
    let user_map: Vec<(u64, UserProfile)> =
        USER_PROFILE_STORAGE.with(|storage| storage.borrow().iter().collect());
    let mut user_vec: Vec<UserProfile> = user_map
        .into_iter()
        .map(|(_, user)| user)
        .collect();

    for user in user_vec.iter_mut() {
        let mut userdevices = user.userdevices.clone();
        userdevices.retain(|x| x.device_id != device_id);
        user.userdevices = userdevices;

        USER_PROFILE_STORAGE.with(|storage| {
            storage
                .borrow_mut()
                .insert(user.user_id, user.clone())
                .expect("cannot insert user profile")
        });   
    }


}

// function to get all device configurations
#[ic_cdk::query]
fn get_all_device_configurations() -> Result<Vec<DeviceConfiguration>, Error> {
    let device_config_map: Vec<(u64, DeviceConfiguration)> =
    DEVICE_CONFIGURATION_STORAGE.with(|storage| storage.borrow().iter().collect());
    let device_config_vec: Vec<DeviceConfiguration> = device_config_map
        .into_iter()
        .map(|(_, device_config)| device_config)
        .collect();

    if !device_config_vec.is_empty() {
        Ok(device_config_vec)
    } else {
        Err(Error::NotFound {
            msg: "No device configurations found".to_string(),
        })
    }
}

// function to get all device configurations by research data id
#[ic_cdk::query]
fn get_all_device_configurations_by_research_data_id(research_data_id: u64) -> Result<Vec<DeviceConfiguration>, Error> {
    let device_config_map: Vec<(u64, DeviceConfiguration)> =
    DEVICE_CONFIGURATION_STORAGE.with(|storage| storage.borrow().iter().collect());
    let device_config_vec: Vec<DeviceConfiguration> = device_config_map
        .into_iter()
        .map(|(_, device_config)| device_config)
        .collect();

    let mut device_config_vec_by_research_data_id: Vec<DeviceConfiguration> = Vec::new();

    for device_config in device_config_vec {
        if device_config.research_data_id == research_data_id {
            device_config_vec_by_research_data_id.push(device_config);
        }
    }

    if !device_config_vec_by_research_data_id.is_empty() {
        Ok(device_config_vec_by_research_data_id)
    } else {
        Err(Error::NotFound {
            msg: "No device configurations found".to_string(),
        })
    }

}


// Research Data 

// function to create a research data
#[ic_cdk::update]
fn add_research_data(payload: ResearchDataPayload) -> Result<ResearchData, Error> {
    //input validation check
    if is_invalid_string(&payload.research_data_name) 
        || is_invalid_string(&payload.research_data_description)
        || is_invalid_string(&payload.research_data_status)
        
    {
        return Err(Error::NotFound {
            msg: "Invalid input Fill all Fields".to_string(),
        });
      
    }
    let id = RESEARCH_DATA_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("cannot increment id counter");
    let research_data = ResearchData {
        research_data_id: id,
        research_data_name: payload.research_data_name,
        research_data_description: payload.research_data_description,
        research_data_status: payload.research_data_status,
        research_data_config: Vec::new(),
        
        };
    RESEARCH_DATA_STORAGE.with(|s| {
        s.borrow_mut()
            .insert(id, research_data.clone())
    });
    Ok(research_data)
}

// function to update a research data
#[ic_cdk::update]
fn update_research_data(research_data_id: u64, payload: ResearchDataPayload) -> Result<ResearchData, Error> {
    //input validation check
    if is_invalid_string(&payload.research_data_name) 
        || is_invalid_string(&payload.research_data_description)
        || is_invalid_string(&payload.research_data_status)
        
    {
        return Err(Error::NotFound {
            msg: "Invalid input Fill all Fields".to_string(),
        });
      
    }
    let research_data = ResearchData {
        research_data_id,
        research_data_name: payload.research_data_name,
        research_data_description: payload.research_data_description,
        research_data_status: payload.research_data_status,
        research_data_config: Vec::new(),
    };
    RESEARCH_DATA_STORAGE.with(|storage| {
        storage
            .borrow_mut()
            .insert(research_data_id, research_data.clone())
            .expect("cannot insert research data")
    });
    Ok(research_data)

}

// function to get a research data
#[ic_cdk::query]
fn get_research_data(research_data_id: u64) -> Result<ResearchData, Error> {
    RESEARCH_DATA_STORAGE.with(|storage| {
        storage.borrow_mut().get(&research_data_id).ok_or(Error::NotFound {
            msg: format!("research data with id={} not found", research_data_id),
        })
    })
}

// function to delete a research data

#[ic_cdk::update]
fn delete_research_data(research_data_id: u64) -> Result<(), Error> {
    RESEARCH_DATA_STORAGE.with(|storage| {
        storage
            .borrow_mut()
            .remove(&research_data_id)
            .ok_or(Error::NotFound {
                msg: "Research data not found".to_string(),
            })
    })?;
    Ok(())
}

// function to get all research data
#[ic_cdk::query]
fn get_all_research_data() -> Result<Vec<ResearchData>, Error> {
    let research_data_map: Vec<(u64, ResearchData)> =
    RESEARCH_DATA_STORAGE.with(|storage| storage.borrow().iter().collect());
    let research_data_vec: Vec<ResearchData> = research_data_map
        .into_iter()
        .map(|(_, research_data)| research_data)
        .collect();

    if !research_data_vec.is_empty() {
        Ok(research_data_vec)
    } else {
        Err(Error::NotFound {
            msg: "No research data found".to_string(),
        })
    }
}


// Device Settings

// function to create a device settings
#[ic_cdk::update]
fn add_device_settings(payload: DeviceSettingsPayload) -> Result<DeviceSettings, Error> {
    //input validation check
    if is_invalid_string(&payload.signal_type) || payload.compatability.is_empty(){
        return Err(Error::NotFound {
            msg: "Invalid input Fill all Fields".to_string(),
        });
      
    }
    let id = DEVICE_SETTINGS_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("cannot increment id counter");
    let device_settings = DeviceSettings {
        device_settings_id: id,
        power_consumption: payload.power_consumption,
        signal_frequency: payload.signal_frequency,
        signal_type: payload.signal_type,
        compatability: payload.compatability,
    };
    DEVICE_SETTINGS_STORAGE.with(|s| {
        s.borrow_mut()
            .insert(id, device_settings.clone())
    });
    Ok(device_settings)
}

// function to update a device settings
#[ic_cdk::update]
fn update_device_settings(device_settings_id: u64, payload: DeviceSettingsPayload) -> Result<DeviceSettings, Error> {
    //input validation check
    if is_invalid_string(&payload.signal_type) || payload.compatability.is_empty(){
        return Err(Error::NotFound {
            msg: "Invalid input Fill all Fields".to_string(),
        });
      
    }
    let device_settings = DeviceSettings {
        device_settings_id,
        power_consumption: payload.power_consumption,
        signal_frequency: payload.signal_frequency,
        signal_type: payload.signal_type,
        compatability: payload.compatability,
    };
    DEVICE_SETTINGS_STORAGE.with(|storage| {
        storage
            .borrow_mut()
            .insert(device_settings_id, device_settings.clone())
            .expect("cannot insert device settings")
    });
    Ok(device_settings)

}

// function to get a device settings
#[ic_cdk::query]
fn get_device_settings(device_settings_id: u64) -> Result<DeviceSettings, Error> {
    DEVICE_SETTINGS_STORAGE.with(|storage| {
        storage.borrow_mut().get(&device_settings_id).ok_or(Error::NotFound {
            msg: format!("device settings with id={} not found", device_settings_id),
        })
    })
}

// function to delete a device settings
#[ic_cdk::update]
fn delete_device_settings(device_settings_id: u64) -> Result<(), Error> {
    DEVICE_SETTINGS_STORAGE.with(|storage| {
        storage
            .borrow_mut()
            .remove(&device_settings_id)
            .ok_or(Error::NotFound {
                msg: "Device settings not found".to_string(),
            })
    })?;

    delete_device_settings_from_research_data(device_settings_id);
    Ok(())
}

// delete a device settings from a research data
fn delete_device_settings_from_research_data(device_settings_id: u64){
    let research_data_map: Vec<(u64, ResearchData)> =
        RESEARCH_DATA_STORAGE.with(|storage| storage.borrow().iter().collect());
    let mut research_data_vec: Vec<ResearchData> = research_data_map
        .into_iter()
        .map(|(_, research_data)| research_data)
        .collect();

    for research_data in research_data_vec.iter_mut() {
        let mut research_data_config = research_data.research_data_config.clone();
        research_data_config.retain(|x| x.device_settings_id != device_settings_id);
        research_data.research_data_config = research_data_config;

        RESEARCH_DATA_STORAGE.with(|storage| {
            storage
                .borrow_mut()
                .insert(research_data.research_data_id, research_data.clone())
                .expect("cannot insert research data")
        });   
    }

}

// function to get all device settings
#[ic_cdk::query]
fn get_all_device_settings() -> Result<Vec<DeviceSettings>, Error> {
    let device_settings_map: Vec<(u64, DeviceSettings)> =
    DEVICE_SETTINGS_STORAGE.with(|storage| storage.borrow().iter().collect());
    let device_settings_vec: Vec<DeviceSettings> = device_settings_map
        .into_iter()
        .map(|(_, device_settings)| device_settings)
        .collect();

    if !device_settings_vec.is_empty() {
        Ok(device_settings_vec)
    } else {
        Err(Error::NotFound {
            msg: "No device settings found".to_string(),
        })
    }
}

// add a device configuration to a user profile
#[ic_cdk::update]
fn add_device_configuration_to_user_profile(user_id: u64, device_configuration_id: u64) -> Result<(), Error> {
    let user_profile = USER_PROFILE_STORAGE.with(|storage| {
        storage
            .borrow_mut()
            .get(&user_id)
            .ok_or(Error::NotFound {
                msg: format!("user profile  with id={} not found", user_id),
            })
    })?;

    let device_config = DEVICE_CONFIGURATION_STORAGE.with(|storage| {
        storage
            .borrow_mut()
            .get(&device_configuration_id)
            .ok_or(Error::NotFound {
                msg: format!("weapon with id={} not found", device_configuration_id),
            })
    })?;

    let mut user_profile = user_profile.clone();
    user_profile.userdevices.push(device_config.clone());

    USER_PROFILE_STORAGE.with(|storage| {
        storage
            .borrow_mut()
            .insert(user_id, user_profile.clone())
            .expect("cannot insert user profile")
    });

    Ok(())
}

// add a research data to a device configuration
#[ic_cdk::update]
fn add_research_data_to_device_configuration(device_configuration_id: u64, research_data_id: u64) -> Result<(), Error> {
    let device_config = DEVICE_CONFIGURATION_STORAGE.with(|storage| {
        storage
            .borrow_mut()
            .get(&device_configuration_id)
            .ok_or(Error::NotFound {
                msg: format!("device configuration  with id={} not found", device_configuration_id),
            })
    })?;
    // ensures research data exists
    get_research_data(research_data_id)?;
    let mut device_config = device_config.clone();
    device_config.research_data_id = research_data_id;

    DEVICE_CONFIGURATION_STORAGE.with(|storage| {
        storage
            .borrow_mut()
            .insert(device_configuration_id, device_config.clone())
            .expect("cannot insert device configuration")
    });

    Ok(())
}

// add a device settings to a research data
#[ic_cdk::update]
fn add_device_settings_to_research_data(research_data_id: u64, device_settings_id: u64) -> Result<(), Error> {
    let research_data = RESEARCH_DATA_STORAGE.with(|storage| {
        storage
            .borrow_mut()
            .get(&research_data_id)
            .ok_or(Error::NotFound {
                msg: format!("research data  with id={} not found", research_data_id),
            })
    })?;
    let mut research_data = research_data.clone();
    let device_settings = DEVICE_SETTINGS_STORAGE.with(|storage| {
        storage
            .borrow_mut()
            .get(&device_settings_id)
            .ok_or(Error::NotFound {
                msg: format!("device settings  with id={} not found", device_settings_id),
            })
    })?;
    research_data.research_data_config.push(device_settings.clone());

    RESEARCH_DATA_STORAGE.with(|storage| {
        storage
            .borrow_mut()
            .insert(research_data_id, research_data.clone())
            .expect("cannot insert research data")
    });

    Ok(())
}




#[derive(candid::CandidType, Deserialize, Serialize)]
enum Error {
    NotFound { msg: String },
}

// Export the candid interface
ic_cdk::export_candid!();