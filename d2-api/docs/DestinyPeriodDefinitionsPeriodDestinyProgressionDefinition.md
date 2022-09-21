# DestinyPeriodDefinitionsPeriodDestinyProgressionDefinition

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**display_properties** | Option<[**crate::models::DestinyPeriodDefinitionsPeriodDestinyProgressionDisplayPropertiesDefinition**](Destiny.Definitions.DestinyProgressionDisplayPropertiesDefinition.md)> |  | [optional]
**scope** | Option<**i32**> | The \"Scope\" of the progression indicates the source of the progression's live data.  See the DestinyProgressionScope enum for more info: but essentially, a Progression can either be backed by a stored value, or it can be a calculated derivative of other values. | [optional]
**repeat_last_step** | Option<**bool**> | If this is True, then the progression doesn't have a maximum level. | [optional]
**source** | Option<**String**> | If there's a description of how to earn this progression in the local config, this will be that localized description. | [optional]
**steps** | Option<[**Vec<crate::models::DestinyPeriodDefinitionsPeriodDestinyProgressionStepDefinition>**](Destiny.Definitions.DestinyProgressionStepDefinition.md)> | Progressions are divided into Steps, which roughly equate to \"Levels\" in the traditional sense of a Progression. Notably, the last step can be repeated indefinitely if repeatLastStep is true, meaning that the calculation for your level is not as simple as comparing your current progress to the max progress of the steps.   These and more calculations are done for you if you grab live character progression data, such as in the DestinyCharacterProgressionComponent. | [optional]
**visible** | Option<**bool**> | If true, the Progression is something worth showing to users.  If false, BNet isn't going to show it. But that doesn't mean you can't. We're all friends here. | [optional]
**faction_hash** | Option<**i32**> | If the value exists, this is the hash identifier for the Faction that owns this Progression.  This is purely for convenience, if you're looking at a progression and want to know if and who it's related to in terms of Faction Reputation. | [optional]
**color** | Option<[**crate::models::DestinyDefinitionsDestinyProgressionDefinitionColor**](Destiny_Definitions_DestinyProgressionDefinition_color.md)> |  | [optional]
**rank_icon** | Option<**String**> | For progressions that have it, this is the rank icon we use in the Companion, displayed above the progressions' rank value. | [optional]
**reward_items** | Option<[**Vec<crate::models::DestinyPeriodDefinitionsPeriodDestinyProgressionRewardItemQuantity>**](Destiny.Definitions.DestinyProgressionRewardItemQuantity.md)> |  | [optional]
**hash** | Option<**i32**> | The unique identifier for this entity. Guaranteed to be unique for the type of entity, but not globally.  When entities refer to each other in Destiny content, it is this hash that they are referring to. | [optional]
**index** | Option<**i32**> | The index of the entity as it was found in the investment tables. | [optional]
**redacted** | Option<**bool**> | If this is true, then there is an entity with this identifier/type combination, but BNet is not yet allowed to show it. Sorry! | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


