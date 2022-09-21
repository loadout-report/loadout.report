# DestinyPeriodConfigPeriodDestinyManifest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**version** | Option<**String**> |  | [optional]
**mobile_asset_content_path** | Option<**String**> |  | [optional]
**mobile_gear_asset_data_bases** | Option<[**Vec<crate::models::DestinyPeriodConfigPeriodGearAssetDataBaseDefinition>**](Destiny.Config.GearAssetDataBaseDefinition.md)> |  | [optional]
**mobile_world_content_paths** | Option<**::std::collections::HashMap<String, String>**> |  | [optional]
**json_world_content_paths** | Option<**::std::collections::HashMap<String, String>**> | This points to the generated JSON that contains all the Definitions. Each key is a locale. The value is a path to the aggregated world definitions (warning: large file!) | [optional]
**json_world_component_content_paths** | Option<[**::std::collections::HashMap<String, ::std::collections::HashMap<String, String>>**](map.md)> | This points to the generated JSON that contains all the Definitions. Each key is a locale. The value is a dictionary, where the key is a definition type by name, and the value is the path to the file for that definition. WARNING: This is unsafe and subject to change - do not depend on data in these files staying around long-term. | [optional]
**mobile_clan_banner_database_path** | Option<**String**> |  | [optional]
**mobile_gear_cdn** | Option<**::std::collections::HashMap<String, String>**> |  | [optional]
**icon_image_pyramid_info** | Option<[**Vec<crate::models::DestinyPeriodConfigPeriodImagePyramidEntry>**](Destiny.Config.ImagePyramidEntry.md)> | Information about the \"Image Pyramid\" for Destiny icons. Where possible, we create smaller versions of Destiny icons. These are found as subfolders under the location of the \"original/full size\" Destiny images, with the same file name and extension as the original image itself. (this lets us avoid sending largely redundant path info with every entity, at the expense of the smaller versions of the image being less discoverable) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


