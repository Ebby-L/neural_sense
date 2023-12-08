# neural_sense

Welcome to your new neural_sense project and to the internet computer development community. By default, creating a new project adds this README and some template files to your project directory. You can edit these template files to customize your project and to include your own code to speed up the development cycle.

To get started, you might want to explore the project directory structure and the default configuration file. Working with this project in your development environment will not affect any production deployment or identity tokens.

Neural sense is a  build app that handles information related to neuroprosthetics -- such as user profiles, device configuration, device settings and Research data.

The User Profile is connected to Device Configuration and then connected to Research Data which is then connected to  Device setting 


## Structures Implemented 
### User Profile 

Represents a user's profile with associated devices.

### Device Configuration

Describes the configuration details for various neuroprosthetic devices.

### Device Setting 

Contains settings specific to neuroprosthetic devices.

### Research Data 

Stores information related to research conducted in the field of neuroprosthetics.

## CRUD Functions 
### User Profiles 

  - `add_user_profile(payload: UserProfilePayload) -> Result<UserProfile, Error>`:
      - Description: Adds a new user profile.
      - Parameters:
          - `payload`: UserProfilePayload - Details of the user profile to be added.
      - Returns: Result<UserProfile, Error> - Returns the added user profile or an error.

  - `update_user_profile(user_id: u64, payload: UserProfilePayload) -> Result<UserProfile, Error>`:
      - Description: Updates an existing user profile.
      - Parameters:
          - `user_id`: u64 - ID of the user profile to be updated.
          - `payload`: UserProfilePayload - Updated details of the user profile.
      - Returns: Result<UserProfile, Error> - Returns the updated user profile or an error.

  - `get_user_profile(user_id: u64) -> Result<UserProfile, Error>`:
      - Description: Retrieves a user profile by ID.
      - Parameters:
          - `user_id`: u64 - ID of the user profile to be retrieved.
      - Returns: Result<UserProfile, Error> - Returns the requested user profile or an error.

  - `delete_user_profile(user_id: u64) -> Result<(), Error>`:
      - Description: Deletes a user profile by ID.
      - Parameters:
          - `user_id`: u64 - ID of the user profile to be deleted.
      - Returns: Result<(), Error> - Returns success or an error.

  - `get_all_user_profiles() -> Vec<UserProfile>`:
      - Description: Retrieves all user profiles.
      - Returns: Vec<UserProfile> - Returns a vector containing all user profiles.
### Device Configuration 

  - `add_device_configuration(payload: DeviceConfigurationPayload) -> Result<DeviceConfiguration, Error>`:
      - Description: Adds a new device configuration.
      - Parameters:
          - `payload`: DeviceConfigurationPayload - Details of the device configuration to be added.
      - Returns: Result<DeviceConfiguration, Error> - Returns the added device configuration or an error.

  - `update_device_configuration(device_id: u64, payload: DeviceConfigurationPayload) -> Result<DeviceConfiguration, Error>`:
      - Description: Updates an existing device configuration.
      - Parameters:
          - `device_id`: u64 - ID of the device configuration to be updated.
          - `payload`: DeviceConfigurationPayload - Updated details of the device configuration.
      - Returns: Result<DeviceConfiguration, Error> - Returns the updated device configuration or an error.

  - `get_device_configuration(device_id: u64) -> Result<DeviceConfiguration, Error>`:
      - Description: Retrieves a device configuration by ID.
      - Parameters:
          - `device_id`: u64 - ID of the device configuration to be retrieved.
      - Returns: Result<DeviceConfiguration, Error> - Returns the requested device configuration or an error.

  - `delete_device_configuration(device_id: u64) -> Result<(), Error>`:
      - Description: Deletes a device configuration by ID.
      - Parameters:
          - `device_id`: u64 - ID of the device configuration to be deleted.
      - Returns: Result<(), Error> - Returns success or an error.

  - `get_all_device_configurations() -> Vec<DeviceConfiguration>`:
      - Description: Retrieves all device configurations.
      - Returns: Vec<DeviceConfiguration> - Returns a vector containing all device configurations.
### Research Data 
- `add_research_data(payload: ResearchDataPayload) -> Result<ResearchData, Error>`:
  - Description: Adds new research data.
    - Parameters:
        - `payload`: ResearchDataPayload - Details of the research data to be added.
    -Returns: Result<ResearchData, Error> - Returns the added research data or an error.

- `update_research_data(research_id: u64, payload: ResearchDataPayload) -> Result<ResearchData, Error>`:
  - Description: Updates existing research data.
    - Parameters:
        - `research_id`: u64 - ID of the research data to be updated.
        - `payload`: ResearchDataPayload - Updated details of the research data.
    -Returns: Result<ResearchData, Error> - Returns the updated research data or an error.

- `get_research_data(research_id: u64) -> Result<ResearchData, Error>`:
  - Description: Retrieves research data by ID.
    - Parameters:
        - `research_id`: u64 - ID of the research data to be retrieved.
    -Returns: Result<ResearchData, Error> - Returns the requested research data or an error.

- `delete_research_data(research_id: u64) -> Result<(), Error>`:
  - Description: Deletes research data by ID.
    - Parameters:
        - `research_id`: u64 - ID of the research data to be deleted.
    -Returns: Result<(), Error> - Returns success or an error.

-` get_all_research_data() -> Vec<ResearchData>`:
  - Description: Retrieves all research data.
  -Returns: Vec<ResearchData> - Returns a vector containing all research data.- 

### Device Settings 

- `add_device_settings(payload: DeviceSettingsPayload) -> Result<DeviceSettings, Error>:`
    - Description: Adds new device settings.
    - Parameters:
        - `payload`: DeviceSettingsPayload - Details of the device settings to be added.
    - Returns: Result<DeviceSettings, Error> - Returns the added device settings or an error.

- `update_device_settings(device_settings_id: u64, payload: DeviceSettingsPayload) -> Result<DeviceSettings, Error>:`
    - Description: Updates existing device settings.
    - Parameters:
        - `device_settings_id`: u64 - ID of the device settings to be updated.
        - `payload`: DeviceSettingsPayload - Updated details of the device settings.
    - Returns: Result<DeviceSettings, Error> - Returns the updated device settings or an error.

- `get_device_settings(device_settings_id: u64) -> Result<DeviceSettings, Error>:`
    - Description: Retrieves device settings by ID.
    - Parameters:
        - `device_settings_id`: u64 - ID of the device settings to be retrieved.
    - Returns: Result<DeviceSettings, Error> - Returns the requested device settings or an error.

- `delete_device_settings(device_settings_id: u64) -> Result<(), Error>:`
    - Description: Deletes device settings by ID.
    - Parameters:
        - `device_settings_id`: u64 - ID of the device settings to be deleted.
    - Returns: Result<(), Error> - Returns success or an error.

- `get_all_device_settings() -> Vec<DeviceSettings>:`
    - Description: Retrieves all device settings.
    - Returns: Vec<DeviceSettings> - Returns a vector containing all device settings.


## Payload Structures 

### User profile payload 
Represents the data structure for adding/updating user profiles.
### Device Configuration Payload

Describes the data structure for adding/updating device configurations.
### Research Data Payload

Specifies the payload structure for adding/updating research data.
### Device Settings Payload

Represents the payload structure for adding/updating device settings.
To learn more before you start working with neural_sense, see the following documentation available online:

- [Quick Start](https://internetcomputer.org/docs/current/developer-docs/setup/deploy-locally)
- [SDK Developer Tools](https://internetcomputer.org/docs/current/developer-docs/setup/install)
- [Rust Canister Development Guide](https://internetcomputer.org/docs/current/developer-docs/backend/rust/)
- [ic-cdk](https://docs.rs/ic-cdk)
- [ic-cdk-macros](https://docs.rs/ic-cdk-macros)
- [Candid Introduction](https://internetcomputer.org/docs/current/developer-docs/backend/candid/)

If you want to start working on your project right away, you might want to try the following commands:

```bash
cd neural_sense/
dfx help
dfx canister --help
```

## Running the project locally

If you want to test your project locally, you can use the following commands:

```bash
# Starts the replica, running in the background
dfx start --background

# Deploys your canisters to the replica and generates your candid interface
dfx deploy
```

Once the job completes, your application will be available at `http://localhost:4943?canisterId={asset_canister_id}`.

If you have made changes to your backend canister, you can generate a new candid interface with

```bash
npm run generate
```

at any time. This is recommended before starting the frontend development server, and will be run automatically any time you run `dfx deploy`.

If you are making frontend changes, you can start a development server with

```bash
npm start
```

Which will start a server at `http://localhost:8080`, proxying API requests to the replica at port 4943.

### Note on frontend environment variables

If you are hosting frontend code somewhere without using DFX, you may need to make one of the following adjustments to ensure your project does not fetch the root key in production:

- set`DFX_NETWORK` to `ic` if you are using Webpack
- use your own preferred method to replace `process.env.DFX_NETWORK` in the autogenerated declarations
  - Setting `canisters -> {asset_canister_id} -> declarations -> env_override to a string` in `dfx.json` will replace `process.env.DFX_NETWORK` with the string in the autogenerated declarations
- Write your own `createActor` constructor
