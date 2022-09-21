# DestinyPeriodDefinitionsPeriodDestinyNodeStepDefinition

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**display_properties** | Option<[**crate::models::DestinyDefinitionsDestinyNodeStepDefinitionDisplayProperties**](Destiny_Definitions_DestinyNodeStepDefinition_displayProperties.md)> |  | [optional]
**step_index** | Option<**i32**> | The index of this step in the list of Steps on the Talent Node.  Unfortunately, this is the closest thing we have to an identifier for the Step: steps are not provided a content version agnostic identifier. This means that, when you are dealing with talent nodes, you will need to first ensure that you have the latest version of content. | [optional]
**node_step_hash** | Option<**i32**> | The hash of this node step. Unfortunately, while it can be used to uniquely identify the step within a node, it is also content version dependent and should not be relied on without ensuring you have the latest vesion of content. | [optional]
**interaction_description** | Option<**String**> | If you can interact with this node in some way, this is the localized description of that interaction. | [optional]
**damage_type** | Option<**i32**> | An enum representing a damage type granted by activating this step, if any. | [optional]
**damage_type_hash** | Option<**i32**> | If the step provides a damage type, this will be the hash identifier used to look up the damage type's DestinyDamageTypeDefinition. | [optional]
**activation_requirement** | Option<[**crate::models::DestinyDefinitionsDestinyNodeStepDefinitionActivationRequirement**](Destiny_Definitions_DestinyNodeStepDefinition_activationRequirement.md)> |  | [optional]
**can_activate_next_step** | Option<**bool**> | There was a time when talent nodes could be activated multiple times, and the effects of subsequent Steps would be compounded on each other, essentially \"upgrading\" the node. We have moved away from this, but theoretically the capability still exists.  I continue to return this in case it is used in the future: if true and this step is the current step in the node, you are allowed to activate the node a second time to receive the benefits of the next step in the node, which will then become the active step. | [optional]
**next_step_index** | Option<**i32**> | The stepIndex of the next step in the talent node, or -1 if this is the last step or if the next step to be chosen is random.  This doesn't really matter anymore unless canActivateNextStep begins to be used again. | [optional]
**is_next_step_random** | Option<**bool**> | If true, the next step to be chosen is random, and if you're allowed to activate the next step. (if canActivateNextStep = true) | [optional]
**perk_hashes** | Option<**Vec<i32>**> | The list of hash identifiers for Perks (DestinySandboxPerkDefinition) that are applied when this step is active. Perks provide a variety of benefits and modifications - examine DestinySandboxPerkDefinition to learn more. | [optional]
**start_progression_bar_at_progress** | Option<**i32**> | When the Talent Grid's progression reaches this value, the circular \"progress bar\" that surrounds the talent node should be shown.  This also indicates the lower bound of said progress bar, with the upper bound being the progress required to reach activationRequirement.gridLevel. (at some point I should precalculate the upper bound and put it in the definition to save people time) | [optional]
**stat_hashes** | Option<**Vec<i32>**> | When the step provides stat benefits on the item or character, this is the list of hash identifiers for stats (DestinyStatDefinition) that are provided. | [optional]
**affects_quality** | Option<**bool**> | If this is true, the step affects the item's Quality in some way. See DestinyInventoryItemDefinition for more information about the meaning of Quality. I already made a joke about Zen and the Art of Motorcycle Maintenance elsewhere in the documentation, so I will avoid doing it again. Oops too late | [optional]
**step_groups** | Option<[**crate::models::DestinyDefinitionsDestinyNodeStepDefinitionStepGroups**](Destiny_Definitions_DestinyNodeStepDefinition_stepGroups.md)> |  | [optional]
**affects_level** | Option<**bool**> | If true, this step can affect the level of the item. See DestinyInventoryItemDefintion for more information about item levels and their effect on stats. | [optional]
**socket_replacements** | Option<[**Vec<crate::models::DestinyPeriodDefinitionsPeriodDestinyNodeSocketReplaceResponse>**](Destiny.Definitions.DestinyNodeSocketReplaceResponse.md)> | If this step is activated, this will be a list of information used to replace socket items with new Plugs. See DestinyInventoryItemDefinition for more information about sockets and plugs. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


