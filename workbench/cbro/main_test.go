package main

import (
	jsongoccy "github.com/goccy/go-json"
	"strings"
	"testing"
)

func BenchmarkDeserialiseJson(b *testing.B) {
	jsonString := "{\"_id\":\"8320000216\",\"archived\":\"2021-04-16T22:01:40.946Z\",\"period\":\"2021-04-16T21:49:16.000Z\",\"startingPhaseIndex\":0,\"activityDetails\":{\"referenceId\":788769683,\"directorActivityHash\":2865450620,\"instanceId\":\"8320000216\",\"mode\":37,\"modes\":[5,69,37],\"isPrivate\":false,\"membershipType\":3},\"entries\":[{\"standing\":0,\"score\":0,\"player\":{\"destinyUserInfo\":{\"iconPath\":\"/common/destiny2_content/icons/3f171d0a8a13a0991d6b20e3bb4d2ae1.jpg\",\"crossSaveOverride\":0,\"applicableMembershipTypes\":[3],\"isPublic\":true,\"membershipType\":3,\"membershipId\":\"4611686018470662782\",\"displayName\":\"Inferno_Sword\"},\"characterClass\":\"Titan\",\"classHash\":3655393761,\"raceHash\":2803282938,\"genderHash\":3111576190,\"characterLevel\":50,\"lightLevel\":1318,\"emblemHash\":1983519830},\"characterId\":\"2305843009300199970\",\"values\":{\"assists\":3,\"score\":0,\"kills\":10,\"averageScorePerKill\":0,\"deaths\":12,\"averageScorePerLife\":0,\"completed\":1,\"opponentsDefeated\":13,\"efficiency\":1.0833333333333333,\"killsDeathsRatio\":0.8333333333333334,\"killsDeathsAssists\":0.9583333333333334,\"activityDurationSeconds\":588,\"standing\":0,\"team\":19,\"completionReason\":0,\"fireteamId\":7363038389368486000,\"startSeconds\":2,\"timePlayedSeconds\":586,\"playerCount\":6,\"teamScore\":4},\"extended\":{\"weapons\":[{\"referenceId\":2351180975,\"values\":{\"uniqueWeaponKills\":3,\"uniqueWeaponPrecisionKills\":3,\"uniqueWeaponKillsPrecisionKills\":1}},{\"referenceId\":3413860062,\"values\":{\"uniqueWeaponKills\":1,\"uniqueWeaponPrecisionKills\":0,\"uniqueWeaponKillsPrecisionKills\":0}}],\"values\":{\"medalMulti2x\":2,\"medalMultiEntireTeam\":1,\"medalStreakCombined\":1,\"medalStreak5x\":1,\"medalStreakTeam\":1,\"medalPayback\":3,\"medalAvenger\":1,\"medalSurvivalWinLastStand\":1,\"medalSurvivalKnockout\":1,\"allMedalsEarned\":12,\"precisionKills\":3,\"weaponKillsGrenade\":1,\"weaponKillsMelee\":4,\"weaponKillsSuper\":0,\"weaponKillsAbility\":0}}},{\"standing\":0,\"score\":0,\"player\":{\"destinyUserInfo\":{\"iconPath\":\"/common/destiny2_content/icons/0f8efc77d3a6100c142215ac9d8155cd.jpg\",\"crossSaveOverride\":1,\"applicableMembershipTypes\":[3,1],\"isPublic\":true,\"membershipType\":1,\"membershipId\":\"4611686018476051424\",\"displayName\":\"Ghost FameZ\"},\"characterClass\":\"Hunter\",\"classHash\":671679327,\"raceHash\":3887404748,\"genderHash\":2204441813,\"characterLevel\":50,\"lightLevel\":1320,\"emblemHash\":1230660649},\"characterId\":\"2305843009354144550\",\"values\":{\"assists\":2,\"score\":0,\"kills\":19,\"averageScorePerKill\":0,\"deaths\":7,\"averageScorePerLife\":0,\"completed\":1,\"opponentsDefeated\":21,\"efficiency\":3,\"killsDeathsRatio\":2.7142857142857144,\"killsDeathsAssists\":2.857142857142857,\"activityDurationSeconds\":588,\"standing\":0,\"team\":19,\"completionReason\":0,\"fireteamId\":7363038389368486000,\"startSeconds\":0,\"timePlayedSeconds\":588,\"playerCount\":6,\"teamScore\":4},\"extended\":{\"weapons\":[{\"referenceId\":4184808992,\"values\":{\"uniqueWeaponKills\":1,\"uniqueWeaponPrecisionKills\":0,\"uniqueWeaponKillsPrecisionKills\":0}},{\"referenceId\":2907129556,\"values\":{\"uniqueWeaponKills\":1,\"uniqueWeaponPrecisionKills\":1,\"uniqueWeaponKillsPrecisionKills\":1}},{\"referenceId\":2351180975,\"values\":{\"uniqueWeaponKills\":7,\"uniqueWeaponPrecisionKills\":5,\"uniqueWeaponKillsPrecisionKills\":0.7142857142857143}},{\"referenceId\":3413860062,\"values\":{\"uniqueWeaponKills\":5,\"uniqueWeaponPrecisionKills\":3,\"uniqueWeaponKillsPrecisionKills\":0.6}}],\"values\":{\"medalStreakCombined\":1,\"medalStreak5x\":1,\"medalStreak10x\":1,\"medalStreakTeam\":1,\"medalPayback\":2,\"medalAvenger\":6,\"medalDefense\":1,\"medalWeaponHandCannon\":1,\"medalSurvivalUndefeated\":2,\"allMedalsEarned\":16,\"precisionKills\":9,\"weaponKillsGrenade\":0,\"weaponKillsMelee\":5,\"weaponKillsSuper\":0,\"weaponKillsAbility\":0}}},{\"standing\":1,\"score\":0,\"player\":{\"destinyUserInfo\":{\"iconPath\":\"/common/destiny2_content/icons/7ce96360f9bc7dd28d25df09c76c37c3.jpg\",\"crossSaveOverride\":0,\"applicableMembershipTypes\":[3],\"isPublic\":true,\"membershipType\":3,\"membershipId\":\"4611686018486318816\",\"displayName\":\"CyberTitan\"},\"characterClass\":\"Hunter\",\"classHash\":671679327,\"raceHash\":898834093,\"genderHash\":3111576190,\"characterLevel\":50,\"lightLevel\":1331,\"emblemHash\":1661191192},\"characterId\":\"2305843009418004736\",\"values\":{\"assists\":7,\"score\":0,\"kills\":16,\"averageScorePerKill\":0,\"deaths\":9,\"averageScorePerLife\":0,\"completed\":1,\"opponentsDefeated\":23,\"efficiency\":2.5555555555555554,\"killsDeathsRatio\":1.7777777777777777,\"killsDeathsAssists\":2.1666666666666665,\"activityDurationSeconds\":588,\"standing\":1,\"team\":18,\"completionReason\":0,\"fireteamId\":2075564295108842800,\"startSeconds\":3,\"timePlayedSeconds\":585,\"playerCount\":6,\"teamScore\":2},\"extended\":{\"weapons\":[{\"referenceId\":2220884262,\"values\":{\"uniqueWeaponKills\":9,\"uniqueWeaponPrecisionKills\":7,\"uniqueWeaponKillsPrecisionKills\":0.7777777777777778}},{\"referenceId\":2496242052,\"values\":{\"uniqueWeaponKills\":1,\"uniqueWeaponPrecisionKills\":0,\"uniqueWeaponKillsPrecisionKills\":0}},{\"referenceId\":1179141605,\"values\":{\"uniqueWeaponKills\":4,\"uniqueWeaponPrecisionKills\":0,\"uniqueWeaponKillsPrecisionKills\":0}},{\"referenceId\":2351180975,\"values\":{\"uniqueWeaponKills\":1,\"uniqueWeaponPrecisionKills\":1,\"uniqueWeaponKillsPrecisionKills\":1}}],\"values\":{\"medalMulti2x\":1,\"medalStreakCombined\":2,\"medalStreak5x\":1,\"medalStreak10x\":1,\"medalStreakTeam\":1,\"medalPayback\":2,\"medalAvenger\":2,\"medalDefense\":5,\"medalMatchMostDamage\":1,\"medalWeaponHandCannon\":3,\"medalSurvivalUndefeated\":1,\"allMedalsEarned\":20,\"precisionKills\":8,\"weaponKillsGrenade\":0,\"weaponKillsMelee\":1,\"weaponKillsSuper\":0,\"weaponKillsAbility\":0}}},{\"standing\":1,\"score\":0,\"player\":{\"destinyUserInfo\":{\"iconPath\":\"/common/destiny2_content/icons/34f9de740dca4e9dbd34a2541abdd012.jpg\",\"crossSaveOverride\":0,\"applicableMembershipTypes\":[3],\"isPublic\":true,\"membershipType\":3,\"membershipId\":\"4611686018487125909\",\"displayName\":\"BioFreak580\"},\"characterClass\":\"Hunter\",\"classHash\":671679327,\"raceHash\":2803282938,\"genderHash\":3111576190,\"characterLevel\":50,\"lightLevel\":1309,\"emblemHash\":1661191193},\"characterId\":\"2305843009429174386\",\"values\":{\"assists\":7,\"score\":0,\"kills\":10,\"averageScorePerKill\":0,\"deaths\":12,\"averageScorePerLife\":0,\"completed\":1,\"opponentsDefeated\":17,\"efficiency\":1.4166666666666667,\"killsDeathsRatio\":0.8333333333333334,\"killsDeathsAssists\":1.125,\"activityDurationSeconds\":588,\"standing\":1,\"team\":18,\"completionReason\":0,\"fireteamId\":2075564295108842800,\"startSeconds\":5,\"timePlayedSeconds\":583,\"playerCount\":6,\"teamScore\":2},\"extended\":{\"weapons\":[{\"referenceId\":3628991659,\"values\":{\"uniqueWeaponKills\":5,\"uniqueWeaponPrecisionKills\":2,\"uniqueWeaponKillsPrecisionKills\":0.4}},{\"referenceId\":1179141605,\"values\":{\"uniqueWeaponKills\":4,\"uniqueWeaponPrecisionKills\":0,\"uniqueWeaponKillsPrecisionKills\":0}}],\"values\":{\"medalMulti2x\":2,\"medalStreakCombined\":1,\"medalStreak5x\":1,\"medalStreakTeam\":1,\"medalDefense\":1,\"allMedalsEarned\":6,\"precisionKills\":2,\"weaponKillsGrenade\":0,\"weaponKillsMelee\":0,\"weaponKillsSuper\":0,\"weaponKillsAbility\":0}}},{\"standing\":0,\"score\":0,\"player\":{\"destinyUserInfo\":{\"iconPath\":\"/common/destiny2_content/icons/96aacc85369d268e05498e58d4921b6b.jpg\",\"crossSaveOverride\":0,\"applicableMembershipTypes\":[3],\"isPublic\":true,\"membershipType\":3,\"membershipId\":\"4611686018470672917\",\"displayName\":\"MizeLuna\"},\"characterClass\":\"Warlock\",\"classHash\":2271682572,\"raceHash\":898834093,\"genderHash\":2204441813,\"characterLevel\":50,\"lightLevel\":1319,\"emblemHash\":298334056},\"characterId\":\"2305843009437374480\",\"values\":{\"assists\":3,\"score\":0,\"kills\":3,\"averageScorePerKill\":0,\"deaths\":11,\"averageScorePerLife\":0,\"completed\":1,\"opponentsDefeated\":6,\"efficiency\":0.5454545454545454,\"killsDeathsRatio\":0.2727272727272727,\"killsDeathsAssists\":0.4090909090909091,\"activityDurationSeconds\":588,\"standing\":0,\"team\":19,\"completionReason\":0,\"fireteamId\":7363038389368486000,\"startSeconds\":2,\"timePlayedSeconds\":586,\"playerCount\":6,\"teamScore\":4},\"extended\":{\"weapons\":[{\"referenceId\":1313528549,\"values\":{\"uniqueWeaponKills\":1,\"uniqueWeaponPrecisionKills\":0,\"uniqueWeaponKillsPrecisionKills\":0}}],\"values\":{\"medalMulti2x\":1,\"medalStreakTeam\":1,\"medalPayback\":1,\"allMedalsEarned\":3,\"precisionKills\":0,\"weaponKillsGrenade\":0,\"weaponKillsMelee\":0,\"weaponKillsSuper\":2,\"weaponKillsAbility\":0}}},{\"standing\":1,\"score\":0,\"player\":{\"destinyUserInfo\":{\"iconPath\":\"/common/destiny2_content/icons/d132f979683e86ca72d63b4029c2d8c1.jpg\",\"crossSaveOverride\":2,\"applicableMembershipTypes\":[3,2],\"isPublic\":true,\"membershipType\":2,\"membershipId\":\"4611686018488418979\",\"displayName\":\"DigiAngel13\"},\"characterClass\":\"Titan\",\"classHash\":3655393761,\"raceHash\":898834093,\"genderHash\":3111576190,\"characterLevel\":50,\"lightLevel\":1311,\"emblemHash\":1983519834},\"characterId\":\"2305843009613844819\",\"values\":{\"assists\":2,\"score\":0,\"kills\":4,\"averageScorePerKill\":0,\"deaths\":11,\"averageScorePerLife\":0,\"completed\":1,\"opponentsDefeated\":6,\"efficiency\":0.5454545454545454,\"killsDeathsRatio\":0.36363636363636365,\"killsDeathsAssists\":0.45454545454545453,\"activityDurationSeconds\":588,\"standing\":1,\"team\":18,\"completionReason\":0,\"fireteamId\":2075564295108842800,\"startSeconds\":5,\"timePlayedSeconds\":583,\"playerCount\":6,\"teamScore\":2},\"extended\":{\"weapons\":[{\"referenceId\":1179141605,\"values\":{\"uniqueWeaponKills\":2,\"uniqueWeaponPrecisionKills\":0,\"uniqueWeaponKillsPrecisionKills\":0}},{\"referenceId\":347366834,\"values\":{\"uniqueWeaponKills\":1,\"uniqueWeaponPrecisionKills\":0,\"uniqueWeaponKillsPrecisionKills\":0}}],\"values\":{\"medalStreakTeam\":1,\"medalPayback\":2,\"medalAvenger\":1,\"medalSurvivalUndefeated\":1,\"allMedalsEarned\":5,\"precisionKills\":0,\"weaponKillsGrenade\":0,\"weaponKillsMelee\":1,\"weaponKillsSuper\":0,\"weaponKillsAbility\":0}}}],\"teams\":[{\"teamId\":18,\"standing\":1,\"score\":2,\"teamName\":\"18\"},{\"teamId\":19,\"standing\":0,\"score\":4,\"teamName\":\"19\"}]}"

	for i := 0; i < b.N; i++ {
		reader := strings.NewReader(jsonString)
		decoder := jsongoccy.NewDecoder(reader)
		var p pgcr
		_ = decoder.Decode(&p)
	}
}
