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

### Design Decisions

To help an implementing developer, a couple of design decisions have been made that stray from the official API design spec.
- Any integer that is represented by an i64 in the official API spec gets sent over as a string. This is because JavaScript does not support i64.
  We don't want this kind of indirection to hinder a developer, so we parse any such string as an i64.
- We assume that any property defined by the API specification is required. This is to help a developer avoid having to check for null values.
  The API does not always mark these clearly, and as such some manual work has gone into marking values as Option<T> where needed.
- In the case of optional arrays, we default to an empty array instead.
- The value 0 is coerced to None for certain values where it makes sense. An implementing developer should not have to worry about having to check both for 
  an optional value and for a value of 0.
- An empty string is coerced to None for certain values where it makes sense. An implementing developer should not have to worry about having to check both for 
  an optional value and for a value of "".
- Any hashes are their own types and strictly typed. This helps to avoid using the wrong hash for the wrong purpose. It also allows a developer to add a better facade for loading hashes in a client implementation (see d2-client).
- Bitflags, where present, are converted to an internal Bitflags struct to ease use for a developer. See the bitflags crate for more information.

### Roadmap
- wasm support with typescript exports
- feature flags for untested/internal/unstable apis

### fuck
Non-exhaustive list of random fields that don't seem to be documented but returned by manifest or similar endpoints 
* DestinyColor.colorHash
* DestinyItemActionBlockDefinition.rewardSheetHash
* DestinyItemActionBlockDefinition.rewardItemHash
* DestinyItemActionBlockDefinition.rewardSiteHash
* DestinyEquippingBlockDefinition.equippingSoundHash
* DestinyInventoryItemDefinition.acquireRewardSiteHash
* DestinyItemInventoryBlockDefinition.nonTransferrableOriginal
* DestinyInventoryItemDefinition.acquireRewardSiteHash
* DestinyInventoryItemDefinition.acquireUnlockHash
* DestinyInventoryItemDefinition.blacklisted

