# DestinyPeriodResponsesPeriodDestinyProfileResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**vendor_receipts** | Option<[**crate::models::DestinyResponsesDestinyProfileResponseVendorReceipts**](Destiny_Responses_DestinyProfileResponse_vendorReceipts.md)> |  | [optional]
**profile_inventory** | Option<[**crate::models::DestinyResponsesDestinyProfileResponseProfileInventory**](Destiny_Responses_DestinyProfileResponse_profileInventory.md)> |  | [optional]
**profile_currencies** | Option<[**crate::models::DestinyResponsesDestinyProfileResponseProfileCurrencies**](Destiny_Responses_DestinyProfileResponse_profileCurrencies.md)> |  | [optional]
**profile** | Option<[**crate::models::DestinyResponsesDestinyProfileResponseProfile**](Destiny_Responses_DestinyProfileResponse_profile.md)> |  | [optional]
**platform_silver** | Option<[**crate::models::DestinyResponsesDestinyProfileResponsePlatformSilver**](Destiny_Responses_DestinyProfileResponse_platformSilver.md)> |  | [optional]
**profile_kiosks** | Option<[**crate::models::DestinyResponsesDestinyProfileResponseProfileKiosks**](Destiny_Responses_DestinyProfileResponse_profileKiosks.md)> |  | [optional]
**profile_plug_sets** | Option<[**crate::models::DestinyResponsesDestinyProfileResponseProfilePlugSets**](Destiny_Responses_DestinyProfileResponse_profilePlugSets.md)> |  | [optional]
**profile_progression** | Option<[**crate::models::DestinyResponsesDestinyProfileResponseProfileProgression**](Destiny_Responses_DestinyProfileResponse_profileProgression.md)> |  | [optional]
**profile_presentation_nodes** | Option<[**crate::models::DestinyResponsesDestinyProfileResponseProfilePresentationNodes**](Destiny_Responses_DestinyProfileResponse_profilePresentationNodes.md)> |  | [optional]
**profile_records** | Option<[**crate::models::DestinyResponsesDestinyProfileResponseProfileRecords**](Destiny_Responses_DestinyProfileResponse_profileRecords.md)> |  | [optional]
**profile_collectibles** | Option<[**crate::models::DestinyResponsesDestinyProfileResponseProfileCollectibles**](Destiny_Responses_DestinyProfileResponse_profileCollectibles.md)> |  | [optional]
**profile_transitory_data** | Option<[**crate::models::DestinyResponsesDestinyProfileResponseProfileTransitoryData**](Destiny_Responses_DestinyProfileResponse_profileTransitoryData.md)> |  | [optional]
**metrics** | Option<[**crate::models::DestinyResponsesDestinyProfileResponseMetrics**](Destiny_Responses_DestinyProfileResponse_metrics.md)> |  | [optional]
**profile_string_variables** | Option<[**crate::models::DestinyResponsesDestinyProfileResponseProfileStringVariables**](Destiny_Responses_DestinyProfileResponse_profileStringVariables.md)> |  | [optional]
**characters** | Option<[**crate::models::DestinyResponsesDestinyProfileResponseCharacters**](Destiny_Responses_DestinyProfileResponse_characters.md)> |  | [optional]
**character_inventories** | Option<[**crate::models::DestinyResponsesDestinyProfileResponseCharacterInventories**](Destiny_Responses_DestinyProfileResponse_characterInventories.md)> |  | [optional]
**character_progressions** | Option<[**crate::models::DestinyResponsesDestinyProfileResponseCharacterProgressions**](Destiny_Responses_DestinyProfileResponse_characterProgressions.md)> |  | [optional]
**character_render_data** | Option<[**crate::models::DestinyResponsesDestinyProfileResponseCharacterRenderData**](Destiny_Responses_DestinyProfileResponse_characterRenderData.md)> |  | [optional]
**character_activities** | Option<[**crate::models::DestinyResponsesDestinyProfileResponseCharacterActivities**](Destiny_Responses_DestinyProfileResponse_characterActivities.md)> |  | [optional]
**character_equipment** | Option<[**crate::models::DestinyResponsesDestinyProfileResponseCharacterEquipment**](Destiny_Responses_DestinyProfileResponse_characterEquipment.md)> |  | [optional]
**character_kiosks** | Option<[**crate::models::DestinyResponsesDestinyProfileResponseCharacterKiosks**](Destiny_Responses_DestinyProfileResponse_characterKiosks.md)> |  | [optional]
**character_plug_sets** | Option<[**crate::models::DestinyResponsesDestinyProfileResponseCharacterPlugSets**](Destiny_Responses_DestinyProfileResponse_characterPlugSets.md)> |  | [optional]
**character_uninstanced_item_components** | Option<[**::std::collections::HashMap<String, crate::models::DestinyBaseItemComponentSetOfuint32>**](DestinyBaseItemComponentSetOfuint32.md)> | Do you ever get the feeling that a system was designed *too* flexibly? That it can be used in so many different ways that you end up being unable to provide an easy to use abstraction for the mess that's happening under the surface?  Let's talk about character-specific data that might be related to items without instances. These two statements are totally unrelated, I promise.  At some point during D2, it was decided that items - such as Bounties - could be given to characters and *not* have instance data, but that *could* display and even use relevant state information on your account and character.  Up to now, any item that had meaningful dependencies on character or account state had to be instanced, and thus \"itemComponents\" was all that you needed: it was keyed by item's instance IDs and provided the stateful information you needed inside.  Unfortunately, we don't live in such a magical world anymore. This is information held on a per-character basis about non-instanced items that the characters have in their inventory - or that reference character-specific state information even if it's in Account-level inventory - and the values related to that item's state in relation to the given character.  To give a concrete example, look at a Moments of Triumph bounty. They exist in a character's inventory, and show/care about a character's progression toward completing the bounty. But the bounty itself is a non-instanced item, like a mod or a currency. This returns that data for the characters who have the bounty in their inventory.  I'm not crying, you're crying Okay we're both crying but it's going to be okay I promise Actually I shouldn't promise that, I don't know if it's going to be okay | [optional]
**character_presentation_nodes** | Option<[**crate::models::DestinyResponsesDestinyProfileResponseCharacterPresentationNodes**](Destiny_Responses_DestinyProfileResponse_characterPresentationNodes.md)> |  | [optional]
**character_records** | Option<[**crate::models::DestinyResponsesDestinyProfileResponseCharacterRecords**](Destiny_Responses_DestinyProfileResponse_characterRecords.md)> |  | [optional]
**character_collectibles** | Option<[**crate::models::DestinyResponsesDestinyProfileResponseCharacterCollectibles**](Destiny_Responses_DestinyProfileResponse_characterCollectibles.md)> |  | [optional]
**character_string_variables** | Option<[**crate::models::DestinyResponsesDestinyProfileResponseCharacterStringVariables**](Destiny_Responses_DestinyProfileResponse_characterStringVariables.md)> |  | [optional]
**character_craftables** | Option<[**crate::models::DestinyResponsesDestinyProfileResponseCharacterCraftables**](Destiny_Responses_DestinyProfileResponse_characterCraftables.md)> |  | [optional]
**item_components** | Option<[**crate::models::DestinyResponsesDestinyProfileResponseItemComponents**](Destiny_Responses_DestinyProfileResponse_itemComponents.md)> |  | [optional]
**character_currency_lookups** | Option<[**crate::models::DestinyResponsesDestinyProfileResponseCharacterCurrencyLookups**](Destiny_Responses_DestinyProfileResponse_characterCurrencyLookups.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


