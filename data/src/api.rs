use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use serde_derive::{Deserialize, Serialize};

pub mod manifest;
pub mod model;

#[derive(Debug)]
pub enum D2ApiError {
    Unknown(i64),
    MissingResponse,
    MissingApiKey,
}
impl Error for D2ApiError {}
impl Display for D2ApiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            D2ApiError::Unknown(code) => write!(f, "Unknown Api Error: {}", code),
            D2ApiError::MissingResponse => write!(f, "Missing Response Body"),
            D2ApiError::MissingApiKey => write!(f, "Missing API Key"),
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ApiResponse<T> {
    pub response: Box<Option<T>>,
    pub error_code: i64,
    pub throttle_seconds: i64,
    pub error_status: String,
    pub message: String,
    pub message_data: MessageData,
}

impl <T> ApiResponse<T> {
    pub fn has_error(&self) -> bool {
        self.error_code != 1
    }
}

impl <T> From<ApiResponse<T>> for Result<T, D2ApiError> {
    fn from(response: ApiResponse<T>) -> Self {
        if response.has_error() {
            match response.error_code {
                2102 => Err(D2ApiError::MissingApiKey),
                code => Err(D2ApiError::Unknown(code))
            }
        } else if response.response.is_some() {
            Ok(response.response.unwrap())
        } else {
            Err(D2ApiError::MissingResponse)
        }
    }
}


#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageData {
}

/// Represents the possible components that can be returned from Destiny \"Get\" calls such as GetProfile, GetCharacter, GetVendor etc...  When making one of these requests, you will pass one or more of these components as a comma separated list in the \"?components=\" querystring parameter. For instance, if you want baseline Profile data, Character Data, and character progressions, you would pass \"?components=Profiles,Characters,CharacterProgressions\" You may use either the numerical or string values.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ComponentType {
    None = 0,
    // GetProfile
    Profiles = 100,
    VendorReceipts = 101,
    ProfileInventories = 102,
    ProfileCurrencies = 103,
    ProfileProgression = 104,
    PlatformSilver = 105,

    Characters = 200,
    CharacterInventories = 201,
    CharacterProgressions = 202,
    CharacterRenderData = 203,
    CharacterActivities = 204,
    CharacterEquipment = 205,

    ItemInstances = 300,
    ItemObjectives = 301,
    ItemPerks = 302,
    ItemRenderData = 303,
    ItemStats = 304,
    ItemSockets = 305,
    ItemTalentGrids = 306,
    ItemCommonData = 307,
    ItemPlugStates = 308,
    ItemPlugObjectives = 309,
    ItemReusablePlugs = 310,

    Vendors = 400,
    VendorCategories = 401,
    VendorSales = 402,
    Kiosks = 500,
    CurrencyLookups = 600,
    PresentationNodes = 700,
    Collectibles = 800,
    Records = 900,
    Transitory = 1000,
    Metrics = 1100,
    StringVariables = 1200,
    Craftables = 1300,
}
