# d2-api

d2-api intends to be a low-level Bungie.net api wrapper.

A higher level abstraction is provided by [d2-client]. 
This library is intended to be used by d2-client and other libraries.

| Api                | Models | Wrapper | Client | Cache |
|--------------------|--------|---------|--------|-------|
| Apps               | ✅      |         |        |       |
| User               |        |         |        |       |
| Content            |        |         |        |       |
| Forum              |        |         |        |       |
| GroupV2            |        |         |        |       |
| Tokens             |        |         |        |       |
| Trending           |        |         |        |       |
| Fireteam           |        |         |        |       |
| Social             |        |         |        |       |
| CommunityContent   |        |         |        |       |
| Destiny Manifest*  |        |         |        |       |
| Destiny2           |        |         |        |       |

* The Destiny Manifest is a special case. It is not a Bungie.net api, but a
  separate api provided by Bungie. It is versioned differently and data retrieved from it can be cached very well.
  The d2-client library provides a higher level abstraction over the manifest API 
  as well as a good default cache implementation.


### fuck
Non-exhaustive list of random fields that don't seem to be documented but returned by manifest or similar endpoints
DestinyColor.colorHash
DestinyItemActionBlockDefinition.rewardSheetHash
DestinyItemActionBlockDefinition.rewardItemHash
DestinyItemActionBlockDefinition.rewardSiteHash
DestinyItemInventoryBlockDefinition.nonTransferrableOriginal
DestinyEquippingBlockDefinition.equippingSoundHash


