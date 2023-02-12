


use serde_repr::{Serialize_repr, Deserialize_repr};

/// No documentation provided.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum PlatformErrorCodes {
    /// No documentation provided.
    None = 0,
    /// No documentation provided.
    Success = 1,
    /// No documentation provided.
    TransportException = 2,
    /// No documentation provided.
    UnhandledException = 3,
    /// No documentation provided.
    NotImplemented = 4,
    /// No documentation provided.
    SystemDisabled = 5,
    /// No documentation provided.
    FailedToLoadAvailableLocalesConfiguration = 6,
    /// No documentation provided.
    ParameterParseFailure = 7,
    /// No documentation provided.
    ParameterInvalidRange = 8,
    /// No documentation provided.
    BadRequest = 9,
    /// No documentation provided.
    AuthenticationInvalid = 10,
    /// No documentation provided.
    DataNotFound = 11,
    /// No documentation provided.
    InsufficientPrivileges = 12,
    /// No documentation provided.
    Duplicate = 13,
    /// No documentation provided.
    UnknownSqlResult = 14,
    /// No documentation provided.
    ValidationError = 15,
    /// No documentation provided.
    ValidationMissingFieldError = 16,
    /// No documentation provided.
    ValidationInvalidInputError = 17,
    /// No documentation provided.
    InvalidParameters = 18,
    /// No documentation provided.
    ParameterNotFound = 19,
    /// No documentation provided.
    UnhandledHttpException = 20,
    /// No documentation provided.
    NotFound = 21,
    /// No documentation provided.
    WebAuthModuleAsyncFailed = 22,
    /// No documentation provided.
    InvalidReturnValue = 23,
    /// No documentation provided.
    UserBanned = 24,
    /// No documentation provided.
    InvalidPostBody = 25,
    /// No documentation provided.
    MissingPostBody = 26,
    /// No documentation provided.
    ExternalServiceTimeout = 27,
    /// No documentation provided.
    ValidationLengthError = 28,
    /// No documentation provided.
    ValidationRangeError = 29,
    /// No documentation provided.
    JsonDeserializationError = 30,
    /// No documentation provided.
    ThrottleLimitExceeded = 31,
    /// No documentation provided.
    ValidationTagError = 32,
    /// No documentation provided.
    ValidationProfanityError = 33,
    /// No documentation provided.
    ValidationUrlFormatError = 34,
    /// No documentation provided.
    ThrottleLimitExceededMinutes = 35,
    /// No documentation provided.
    ThrottleLimitExceededMomentarily = 36,
    /// No documentation provided.
    ThrottleLimitExceededSeconds = 37,
    /// No documentation provided.
    ExternalServiceUnknown = 38,
    /// No documentation provided.
    ValidationWordLengthError = 39,
    /// No documentation provided.
    ValidationInvisibleUnicode = 40,
    /// No documentation provided.
    ValidationBadNames = 41,
    /// No documentation provided.
    ExternalServiceFailed = 42,
    /// No documentation provided.
    ServiceRetired = 43,
    /// No documentation provided.
    UnknownSqlException = 44,
    /// No documentation provided.
    UnsupportedLocale = 45,
    /// No documentation provided.
    InvalidPageNumber = 46,
    /// No documentation provided.
    MaximumPageSizeExceeded = 47,
    /// No documentation provided.
    ServiceUnsupported = 48,
    /// No documentation provided.
    ValidationMaximumUnicodeCombiningCharacters = 49,
    /// No documentation provided.
    ValidationMaximumSequentialCarriageReturns = 50,
    /// No documentation provided.
    PerEndpointRequestThrottleExceeded = 51,
    /// No documentation provided.
    AuthContextCacheAssertion = 52,
    /// No documentation provided.
    ExPlatformStringValidationError = 53,
    /// No documentation provided.
    PerApplicationThrottleExceeded = 54,
    /// No documentation provided.
    PerApplicationAnonymousThrottleExceeded = 55,
    /// No documentation provided.
    PerApplicationAuthenticatedThrottleExceeded = 56,
    /// No documentation provided.
    PerUserThrottleExceeded = 57,
    /// No documentation provided.
    PayloadSignatureVerificationFailure = 58,
    /// No documentation provided.
    InvalidServiceAuthContext = 59,
    /// No documentation provided.
    ObsoleteCredentialType = 89,
    /// No documentation provided.
    UnableToUnPairMobileApp = 90,
    /// No documentation provided.
    UnableToPairMobileApp = 91,
    /// No documentation provided.
    CannotUseMobileAuthWithNonMobileProvider = 92,
    /// No documentation provided.
    MissingDeviceCookie = 93,
    /// No documentation provided.
    FacebookTokenExpired = 94,
    /// No documentation provided.
    AuthTicketRequired = 95,
    /// No documentation provided.
    CookieContextRequired = 96,
    /// No documentation provided.
    UnknownAuthenticationError = 97,
    /// No documentation provided.
    BungieNetAccountCreationRequired = 98,
    /// No documentation provided.
    WebAuthRequired = 99,
    /// No documentation provided.
    ContentUnknownSqlResult = 100,
    /// No documentation provided.
    ContentNeedUniquePath = 101,
    /// No documentation provided.
    ContentSqlException = 102,
    /// No documentation provided.
    ContentNotFound = 103,
    /// No documentation provided.
    ContentSuccessWithTagAddFail = 104,
    /// No documentation provided.
    ContentSearchMissingParameters = 105,
    /// No documentation provided.
    ContentInvalidId = 106,
    /// No documentation provided.
    ContentPhysicalFileDeletionError = 107,
    /// No documentation provided.
    ContentPhysicalFileCreationError = 108,
    /// No documentation provided.
    ContentPerforceSubmissionError = 109,
    /// No documentation provided.
    ContentPerforceInitializationError = 110,
    /// No documentation provided.
    ContentDeploymentPackageNotReadyError = 111,
    /// No documentation provided.
    ContentUploadFailed = 112,
    /// No documentation provided.
    ContentTooManyResults = 113,
    /// No documentation provided.
    ContentInvalidState = 115,
    /// No documentation provided.
    ContentNavigationParentNotFound = 116,
    /// No documentation provided.
    ContentNavigationParentUpdateError = 117,
    /// No documentation provided.
    DeploymentPackageNotEditable = 118,
    /// No documentation provided.
    ContentValidationError = 119,
    /// No documentation provided.
    ContentPropertiesValidationError = 120,
    /// No documentation provided.
    ContentTypeNotFound = 121,
    /// No documentation provided.
    DeploymentPackageNotFound = 122,
    /// No documentation provided.
    ContentSearchInvalidParameters = 123,
    /// No documentation provided.
    ContentItemPropertyAggregationError = 124,
    /// No documentation provided.
    DeploymentPackageFileNotFound = 125,
    /// No documentation provided.
    ContentPerforceFileHistoryNotFound = 126,
    /// No documentation provided.
    ContentAssetZipCreationFailure = 127,
    /// No documentation provided.
    ContentAssetZipCreationBusy = 128,
    /// No documentation provided.
    ContentProjectNotFound = 129,
    /// No documentation provided.
    ContentFolderNotFound = 130,
    /// No documentation provided.
    ContentPackagesInconsistent = 131,
    /// No documentation provided.
    ContentPackagesInvalidState = 132,
    /// No documentation provided.
    ContentPackagesInconsistentType = 133,
    /// No documentation provided.
    ContentCannotDeletePackage = 134,
    /// No documentation provided.
    ContentLockedForChanges = 135,
    /// No documentation provided.
    ContentFileUploadFailed = 136,
    /// No documentation provided.
    ContentNotReviewed = 137,
    /// No documentation provided.
    ContentPermissionDenied = 138,
    /// No documentation provided.
    ContentInvalidExternalUrl = 139,
    /// No documentation provided.
    ContentExternalFileCannotBeImportedLocally = 140,
    /// No documentation provided.
    ContentTagSaveFailure = 141,
    /// No documentation provided.
    ContentPerforceUnmatchedFileError = 142,
    /// No documentation provided.
    ContentPerforceChangelistResultNotFound = 143,
    /// No documentation provided.
    ContentPerforceChangelistFileItemsNotFound = 144,
    /// No documentation provided.
    ContentPerforceInvalidRevisionError = 145,
    /// No documentation provided.
    ContentUnloadedSaveResult = 146,
    /// No documentation provided.
    ContentPropertyInvalidNumber = 147,
    /// No documentation provided.
    ContentPropertyInvalidUrl = 148,
    /// No documentation provided.
    ContentPropertyInvalidDate = 149,
    /// No documentation provided.
    ContentPropertyInvalidSet = 150,
    /// No documentation provided.
    ContentPropertyCannotDeserialize = 151,
    /// No documentation provided.
    ContentRegexValidationFailOnProperty = 152,
    /// No documentation provided.
    ContentMaxLengthFailOnProperty = 153,
    /// No documentation provided.
    ContentPropertyUnexpectedDeserializationError = 154,
    /// No documentation provided.
    ContentPropertyRequired = 155,
    /// No documentation provided.
    ContentCannotCreateFile = 156,
    /// No documentation provided.
    ContentInvalidMigrationFile = 157,
    /// No documentation provided.
    ContentMigrationAlteringProcessedItem = 158,
    /// No documentation provided.
    ContentPropertyDefinitionNotFound = 159,
    /// No documentation provided.
    ContentReviewDataChanged = 160,
    /// No documentation provided.
    ContentRollbackRevisionNotInPackage = 161,
    /// No documentation provided.
    ContentItemNotBasedOnLatestRevision = 162,
    /// No documentation provided.
    ContentUnauthorized = 163,
    /// No documentation provided.
    ContentCannotCreateDeploymentPackage = 164,
    /// No documentation provided.
    ContentUserNotFound = 165,
    /// No documentation provided.
    ContentLocalePermissionDenied = 166,
    /// No documentation provided.
    ContentInvalidLinkToInternalEnvironment = 167,
    /// No documentation provided.
    ContentInvalidBlacklistedContent = 168,
    /// No documentation provided.
    ContentMacroMalformedNoContentId = 169,
    /// No documentation provided.
    ContentMacroMalformedNoTemplateType = 170,
    /// No documentation provided.
    ContentIllegalBNetMembershipId = 171,
    /// No documentation provided.
    ContentLocaleDidNotMatchExpected = 172,
    /// No documentation provided.
    ContentBabelCallFailed = 173,
    /// No documentation provided.
    ContentEnglishPostLiveForbidden = 174,
    /// No documentation provided.
    ContentLocaleEditPermissionDenied = 175,
    /// No documentation provided.
    ContentStackUnknownError = 176,
    /// No documentation provided.
    ContentStackNotFound = 177,
    /// No documentation provided.
    ContentStackRateLimited = 178,
    /// No documentation provided.
    ContentStackTimeout = 179,
    /// No documentation provided.
    ContentStackServiceError = 180,
    /// No documentation provided.
    ContentStackDeserializationFailure = 181,
    /// No documentation provided.
    UserNonUniqueName = 200,
    /// No documentation provided.
    UserManualLinkingStepRequired = 201,
    /// No documentation provided.
    UserCreateUnknownSqlResult = 202,
    /// No documentation provided.
    UserCreateUnknownSqlException = 203,
    /// No documentation provided.
    UserMalformedMembershipId = 204,
    /// No documentation provided.
    UserCannotFindRequestedUser = 205,
    /// No documentation provided.
    UserCannotLoadAccountCredentialLinkInfo = 206,
    /// No documentation provided.
    UserInvalidMobileAppType = 207,
    /// No documentation provided.
    UserMissingMobilePairingInfo = 208,
    /// No documentation provided.
    UserCannotGenerateMobileKeyWhileUsingMobileCredential = 209,
    /// No documentation provided.
    UserGenerateMobileKeyExistingSlotCollision = 210,
    /// No documentation provided.
    UserDisplayNameMissingOrInvalid = 211,
    /// No documentation provided.
    UserCannotLoadAccountProfileData = 212,
    /// No documentation provided.
    UserCannotSaveUserProfileData = 213,
    /// No documentation provided.
    UserEmailMissingOrInvalid = 214,
    /// No documentation provided.
    UserTermsOfUseRequired = 215,
    /// No documentation provided.
    UserCannotCreateNewAccountWhileLoggedIn = 216,
    /// No documentation provided.
    UserCannotResolveCentralAccount = 217,
    /// No documentation provided.
    UserInvalidAvatar = 218,
    /// No documentation provided.
    UserMissingCreatedUserResult = 219,
    /// No documentation provided.
    UserCannotChangeUniqueNameYet = 220,
    /// No documentation provided.
    UserCannotChangeDisplayNameYet = 221,
    /// No documentation provided.
    UserCannotChangeEmail = 222,
    /// No documentation provided.
    UserUniqueNameMustStartWithLetter = 223,
    /// No documentation provided.
    UserNoLinkedAccountsSupportFriendListings = 224,
    /// No documentation provided.
    UserAcknowledgmentTableFull = 225,
    /// No documentation provided.
    UserCreationDestinyMembershipRequired = 226,
    /// No documentation provided.
    UserFriendsTokenNeedsRefresh = 227,
    /// No documentation provided.
    UserEmailValidationUnknown = 228,
    /// No documentation provided.
    UserEmailValidationLimit = 229,
    /// No documentation provided.
    TransactionEmailSendFailure = 230,
    /// No documentation provided.
    MailHookPermissionFailure = 231,
    /// No documentation provided.
    MailServiceRateLimit = 232,
    /// No documentation provided.
    UserEmailMustBeVerified = 233,
    /// No documentation provided.
    UserMustAllowCustomerServiceEmails = 234,
    /// No documentation provided.
    NonTransactionalEmailSendFailure = 235,
    /// No documentation provided.
    UnknownErrorSettingGlobalDisplayName = 236,
    /// No documentation provided.
    DuplicateGlobalDisplayName = 237,
    /// No documentation provided.
    ErrorRunningNameValidationChecks = 238,
    /// No documentation provided.
    ErrorDatabaseGlobalName = 239,
    /// No documentation provided.
    ErrorNoAvailableNameChanges = 240,
    /// No documentation provided.
    ErrorNameAlreadySetToInput = 241,
    /// No documentation provided.
    UserDisplayNameLessThanMinLength = 242,
    /// No documentation provided.
    UserDisplayNameGreaterThanMaxLength = 243,
    /// No documentation provided.
    UserDisplayNameContainsUnacceptableOrInvalidContent = 244,
    /// No documentation provided.
    MessagingUnknownError = 300,
    /// No documentation provided.
    MessagingSelfError = 301,
    /// No documentation provided.
    MessagingSendThrottle = 302,
    /// No documentation provided.
    MessagingNoBody = 303,
    /// No documentation provided.
    MessagingTooManyUsers = 304,
    /// No documentation provided.
    MessagingCanNotLeaveConversation = 305,
    /// No documentation provided.
    MessagingUnableToSend = 306,
    /// No documentation provided.
    MessagingDeletedUserForbidden = 307,
    /// No documentation provided.
    MessagingCannotDeleteExternalConversation = 308,
    /// No documentation provided.
    MessagingGroupChatDisabled = 309,
    /// No documentation provided.
    MessagingMustIncludeSelfInPrivateMessage = 310,
    /// No documentation provided.
    MessagingSenderIsBanned = 311,
    /// No documentation provided.
    MessagingGroupOptionalChatExceededMaximum = 312,
    /// No documentation provided.
    PrivateMessagingRequiresDestinyMembership = 313,
    /// No documentation provided.
    AddSurveyAnswersUnknownSqlException = 400,
    /// No documentation provided.
    ForumBodyCannotBeEmpty = 500,
    /// No documentation provided.
    ForumSubjectCannotBeEmptyOnTopicPost = 501,
    /// No documentation provided.
    ForumCannotLocateParentPost = 502,
    /// No documentation provided.
    ForumThreadLockedForReplies = 503,
    /// No documentation provided.
    ForumUnknownSqlResultDuringCreatePost = 504,
    /// No documentation provided.
    ForumUnknownTagCreationError = 505,
    /// No documentation provided.
    ForumUnknownSqlResultDuringTagItem = 506,
    /// No documentation provided.
    ForumUnknownExceptionCreatePost = 507,
    /// No documentation provided.
    ForumQuestionMustBeTopicPost = 508,
    /// No documentation provided.
    ForumExceptionDuringTagSearch = 509,
    /// No documentation provided.
    ForumExceptionDuringTopicRetrieval = 510,
    /// No documentation provided.
    ForumAliasedTagError = 511,
    /// No documentation provided.
    ForumCannotLocateThread = 512,
    /// No documentation provided.
    ForumUnknownExceptionEditPost = 513,
    /// No documentation provided.
    ForumCannotLocatePost = 514,
    /// No documentation provided.
    ForumUnknownExceptionGetOrCreateTags = 515,
    /// No documentation provided.
    ForumEditPermissionDenied = 516,
    /// No documentation provided.
    ForumUnknownSqlResultDuringTagIdRetrieval = 517,
    /// No documentation provided.
    ForumCannotGetRating = 518,
    /// No documentation provided.
    ForumUnknownExceptionGetRating = 519,
    /// No documentation provided.
    ForumRatingsAccessError = 520,
    /// No documentation provided.
    ForumRelatedPostAccessError = 521,
    /// No documentation provided.
    ForumLatestReplyAccessError = 522,
    /// No documentation provided.
    ForumUserStatusAccessError = 523,
    /// No documentation provided.
    ForumAuthorAccessError = 524,
    /// No documentation provided.
    ForumGroupAccessError = 525,
    /// No documentation provided.
    ForumUrlExpectedButMissing = 526,
    /// No documentation provided.
    ForumRepliesCannotBeEmpty = 527,
    /// No documentation provided.
    ForumRepliesCannotBeInDifferentGroups = 528,
    /// No documentation provided.
    ForumSubTopicCannotBeCreatedAtThisThreadLevel = 529,
    /// No documentation provided.
    ForumCannotCreateContentTopic = 530,
    /// No documentation provided.
    ForumTopicDoesNotExist = 531,
    /// No documentation provided.
    ForumContentCommentsNotAllowed = 532,
    /// No documentation provided.
    ForumUnknownSqlResultDuringEditPost = 533,
    /// No documentation provided.
    ForumUnknownSqlResultDuringGetPost = 534,
    /// No documentation provided.
    ForumPostValidationBadUrl = 535,
    /// No documentation provided.
    ForumBodyTooLong = 536,
    /// No documentation provided.
    ForumSubjectTooLong = 537,
    /// No documentation provided.
    ForumAnnouncementNotAllowed = 538,
    /// No documentation provided.
    ForumCannotShareOwnPost = 539,
    /// No documentation provided.
    ForumEditNoOp = 540,
    /// No documentation provided.
    ForumUnknownDatabaseErrorDuringGetPost = 541,
    /// No documentation provided.
    ForumExceeedMaximumRowLimit = 542,
    /// No documentation provided.
    ForumCannotSharePrivatePost = 543,
    /// No documentation provided.
    ForumCannotCrossPostBetweenGroups = 544,
    /// No documentation provided.
    ForumIncompatibleCategories = 555,
    /// No documentation provided.
    ForumCannotUseTheseCategoriesOnNonTopicPost = 556,
    /// No documentation provided.
    ForumCanOnlyDeleteTopics = 557,
    /// No documentation provided.
    ForumDeleteSqlException = 558,
    /// No documentation provided.
    ForumDeleteSqlUnknownResult = 559,
    /// No documentation provided.
    ForumTooManyTags = 560,
    /// No documentation provided.
    ForumCanOnlyRateTopics = 561,
    /// No documentation provided.
    ForumBannedPostsCannotBeEdited = 562,
    /// No documentation provided.
    ForumThreadRootIsBanned = 563,
    /// No documentation provided.
    ForumCannotUseOfficialTagCategoryAsTag = 564,
    /// No documentation provided.
    ForumAnswerCannotBeMadeOnCreatePost = 565,
    /// No documentation provided.
    ForumAnswerCannotBeMadeOnEditPost = 566,
    /// No documentation provided.
    ForumAnswerPostIdIsNotADirectReplyOfQuestion = 567,
    /// No documentation provided.
    ForumAnswerTopicIdIsNotAQuestion = 568,
    /// No documentation provided.
    ForumUnknownExceptionDuringMarkAnswer = 569,
    /// No documentation provided.
    ForumUnknownSqlResultDuringMarkAnswer = 570,
    /// No documentation provided.
    ForumCannotRateYourOwnPosts = 571,
    /// No documentation provided.
    ForumPollsMustBeTheFirstPostInTopic = 572,
    /// No documentation provided.
    ForumInvalidPollInput = 573,
    /// No documentation provided.
    ForumGroupAdminEditNonMember = 574,
    /// No documentation provided.
    ForumCannotEditModeratorEditedPost = 575,
    /// No documentation provided.
    ForumRequiresDestinyMembership = 576,
    /// No documentation provided.
    ForumUnexpectedError = 577,
    /// No documentation provided.
    ForumAgeLock = 578,
    /// No documentation provided.
    ForumMaxPages = 579,
    /// No documentation provided.
    ForumMaxPagesOldestFirst = 580,
    /// No documentation provided.
    ForumCannotApplyForumIdWithoutTags = 581,
    /// No documentation provided.
    ForumCannotApplyForumIdToNonTopics = 582,
    /// No documentation provided.
    ForumCannotDownvoteCommunityCreations = 583,
    /// No documentation provided.
    ForumTopicsMustHaveOfficialCategory = 584,
    /// No documentation provided.
    ForumRecruitmentTopicMalformed = 585,
    /// No documentation provided.
    ForumRecruitmentTopicNotFound = 586,
    /// No documentation provided.
    ForumRecruitmentTopicNoSlotsRemaining = 587,
    /// No documentation provided.
    ForumRecruitmentTopicKickBan = 588,
    /// No documentation provided.
    ForumRecruitmentTopicRequirementsNotMet = 589,
    /// No documentation provided.
    ForumRecruitmentTopicNoPlayers = 590,
    /// No documentation provided.
    ForumRecruitmentApproveFailMessageBan = 591,
    /// No documentation provided.
    ForumRecruitmentGlobalBan = 592,
    /// No documentation provided.
    ForumUserBannedFromThisTopic = 593,
    /// No documentation provided.
    ForumRecruitmentFireteamMembersOnly = 594,
    /// No documentation provided.
    ForumRequiresDestiny2Progress = 595,
    /// No documentation provided.
    ForumRequiresDestiny2EntitlementPurchase = 596,
    /// No documentation provided.
    GroupMembershipApplicationAlreadyResolved = 601,
    /// No documentation provided.
    GroupMembershipAlreadyApplied = 602,
    /// No documentation provided.
    GroupMembershipInsufficientPrivileges = 603,
    /// No documentation provided.
    GroupIdNotReturnedFromCreation = 604,
    /// No documentation provided.
    GroupSearchInvalidParameters = 605,
    /// No documentation provided.
    GroupMembershipPendingApplicationNotFound = 606,
    /// No documentation provided.
    GroupInvalidId = 607,
    /// No documentation provided.
    GroupInvalidMembershipId = 608,
    /// No documentation provided.
    GroupInvalidMembershipType = 609,
    /// No documentation provided.
    GroupMissingTags = 610,
    /// No documentation provided.
    GroupMembershipNotFound = 611,
    /// No documentation provided.
    GroupInvalidRating = 612,
    /// No documentation provided.
    GroupUserFollowingAccessError = 613,
    /// No documentation provided.
    GroupUserMembershipAccessError = 614,
    /// No documentation provided.
    GroupCreatorAccessError = 615,
    /// No documentation provided.
    GroupAdminAccessError = 616,
    /// No documentation provided.
    GroupPrivatePostNotViewable = 617,
    /// No documentation provided.
    GroupMembershipNotLoggedIn = 618,
    /// No documentation provided.
    GroupNotDeleted = 619,
    /// No documentation provided.
    GroupUnknownErrorUndeletingGroup = 620,
    /// No documentation provided.
    GroupDeleted = 621,
    /// No documentation provided.
    GroupNotFound = 622,
    /// No documentation provided.
    GroupMemberBanned = 623,
    /// No documentation provided.
    GroupMembershipClosed = 624,
    /// No documentation provided.
    GroupPrivatePostOverrideError = 625,
    /// No documentation provided.
    GroupNameTaken = 626,
    /// No documentation provided.
    GroupDeletionGracePeriodExpired = 627,
    /// No documentation provided.
    GroupCannotCheckBanStatus = 628,
    /// No documentation provided.
    GroupMaximumMembershipCountReached = 629,
    /// No documentation provided.
    NoDestinyAccountForClanPlatform = 630,
    /// No documentation provided.
    AlreadyRequestingMembershipForClanPlatform = 631,
    /// No documentation provided.
    AlreadyClanMemberOnPlatform = 632,
    /// No documentation provided.
    GroupJoinedCannotSetClanName = 633,
    /// No documentation provided.
    GroupLeftCannotClearClanName = 634,
    /// No documentation provided.
    GroupRelationshipRequestPending = 635,
    /// No documentation provided.
    GroupRelationshipRequestBlocked = 636,
    /// No documentation provided.
    GroupRelationshipRequestNotFound = 637,
    /// No documentation provided.
    GroupRelationshipBlockNotFound = 638,
    /// No documentation provided.
    GroupRelationshipNotFound = 639,
    /// No documentation provided.
    GroupAlreadyAllied = 641,
    /// No documentation provided.
    GroupAlreadyMember = 642,
    /// No documentation provided.
    GroupRelationshipAlreadyExists = 643,
    /// No documentation provided.
    InvalidGroupTypesForRelationshipRequest = 644,
    /// No documentation provided.
    GroupAtMaximumAlliances = 646,
    /// No documentation provided.
    GroupCannotSetClanOnlySettings = 647,
    /// No documentation provided.
    ClanCannotSetTwoDefaultPostTypes = 648,
    /// No documentation provided.
    GroupMemberInvalidMemberType = 649,
    /// No documentation provided.
    GroupInvalidPlatformType = 650,
    /// No documentation provided.
    GroupMemberInvalidSort = 651,
    /// No documentation provided.
    GroupInvalidResolveState = 652,
    /// No documentation provided.
    ClanAlreadyEnabledForPlatform = 653,
    /// No documentation provided.
    ClanNotEnabledForPlatform = 654,
    /// No documentation provided.
    ClanEnabledButCouldNotJoinNoAccount = 655,
    /// No documentation provided.
    ClanEnabledButCouldNotJoinAlreadyMember = 656,
    /// No documentation provided.
    ClanCannotJoinNoCredential = 657,
    /// No documentation provided.
    NoClanMembershipForPlatform = 658,
    /// No documentation provided.
    GroupToGroupFollowLimitReached = 659,
    /// No documentation provided.
    ChildGroupAlreadyInAlliance = 660,
    /// No documentation provided.
    OwnerGroupAlreadyInAlliance = 661,
    /// No documentation provided.
    AllianceOwnerCannotJoinAlliance = 662,
    /// No documentation provided.
    GroupNotInAlliance = 663,
    /// No documentation provided.
    ChildGroupCannotInviteToAlliance = 664,
    /// No documentation provided.
    GroupToGroupAlreadyFollowed = 665,
    /// No documentation provided.
    GroupToGroupNotFollowing = 666,
    /// No documentation provided.
    ClanMaximumMembershipReached = 667,
    /// No documentation provided.
    ClanNameNotValid = 668,
    /// No documentation provided.
    ClanNameNotValidError = 669,
    /// No documentation provided.
    AllianceOwnerNotDefined = 670,
    /// No documentation provided.
    AllianceChildNotDefined = 671,
    /// No documentation provided.
    ClanCultureIllegalCharacters = 672,
    /// No documentation provided.
    ClanTagIllegalCharacters = 673,
    /// No documentation provided.
    ClanRequiresInvitation = 674,
    /// No documentation provided.
    ClanMembershipClosed = 675,
    /// No documentation provided.
    ClanInviteAlreadyMember = 676,
    /// No documentation provided.
    GroupInviteAlreadyMember = 677,
    /// No documentation provided.
    GroupJoinApprovalRequired = 678,
    /// No documentation provided.
    ClanTagRequired = 679,
    /// No documentation provided.
    GroupNameCannotStartOrEndWithWhiteSpace = 680,
    /// No documentation provided.
    ClanCallsignCannotStartOrEndWithWhiteSpace = 681,
    /// No documentation provided.
    ClanMigrationFailed = 682,
    /// No documentation provided.
    ClanNotEnabledAlreadyMemberOfAnotherClan = 683,
    /// No documentation provided.
    GroupModerationNotPermittedOnNonMembers = 684,
    /// No documentation provided.
    ClanCreationInWorldServerFailed = 685,
    /// No documentation provided.
    ClanNotFound = 686,
    /// No documentation provided.
    ClanMembershipLevelDoesNotPermitThatAction = 687,
    /// No documentation provided.
    ClanMemberNotFound = 688,
    /// No documentation provided.
    ClanMissingMembershipApprovers = 689,
    /// No documentation provided.
    ClanInWrongStateForRequestedAction = 690,
    /// No documentation provided.
    ClanNameAlreadyUsed = 691,
    /// No documentation provided.
    ClanTooFewMembers = 692,
    /// No documentation provided.
    ClanInfoCannotBeWhitespace = 693,
    /// No documentation provided.
    GroupCultureThrottle = 694,
    /// No documentation provided.
    ClanTargetDisallowsInvites = 695,
    /// No documentation provided.
    ClanInvalidOperation = 696,
    /// No documentation provided.
    ClanFounderCannotLeaveWithoutAbdication = 697,
    /// No documentation provided.
    ClanNameReserved = 698,
    /// No documentation provided.
    ClanApplicantInClanSoNowInvited = 699,
    /// No documentation provided.
    ActivitiesUnknownException = 701,
    /// No documentation provided.
    ActivitiesParameterNull = 702,
    /// No documentation provided.
    ActivityCountsDiabled = 703,
    /// No documentation provided.
    ActivitySearchInvalidParameters = 704,
    /// No documentation provided.
    ActivityPermissionDenied = 705,
    /// No documentation provided.
    ShareAlreadyShared = 706,
    /// No documentation provided.
    ActivityLoggingDisabled = 707,
    /// No documentation provided.
    ClanRequiresExistingDestinyAccount = 750,
    /// No documentation provided.
    ClanNameRestricted = 751,
    /// No documentation provided.
    ClanCreationBan = 752,
    /// No documentation provided.
    ClanCreationTenureRequirementsNotMet = 753,
    /// No documentation provided.
    ClanFieldContainsReservedTerms = 754,
    /// No documentation provided.
    ClanFieldContainsInappropriateContent = 755,
    /// No documentation provided.
    ItemAlreadyFollowed = 801,
    /// No documentation provided.
    ItemNotFollowed = 802,
    /// No documentation provided.
    CannotFollowSelf = 803,
    /// No documentation provided.
    GroupFollowLimitExceeded = 804,
    /// No documentation provided.
    TagFollowLimitExceeded = 805,
    /// No documentation provided.
    UserFollowLimitExceeded = 806,
    /// No documentation provided.
    FollowUnsupportedEntityType = 807,
    /// No documentation provided.
    NoValidTagsInList = 900,
    /// No documentation provided.
    BelowMinimumSuggestionLength = 901,
    /// No documentation provided.
    CannotGetSuggestionsOnMultipleTagsSimultaneously = 902,
    /// No documentation provided.
    NotAValidPartialTag = 903,
    /// No documentation provided.
    TagSuggestionsUnknownSqlResult = 904,
    /// No documentation provided.
    TagsUnableToLoadPopularTagsFromDatabase = 905,
    /// No documentation provided.
    TagInvalid = 906,
    /// No documentation provided.
    TagNotFound = 907,
    /// No documentation provided.
    SingleTagExpected = 908,
    /// No documentation provided.
    TagsExceededMaximumPerItem = 909,
    /// No documentation provided.
    IgnoreInvalidParameters = 1000,
    /// No documentation provided.
    IgnoreSqlException = 1001,
    /// No documentation provided.
    IgnoreErrorRetrievingGroupPermissions = 1002,
    /// No documentation provided.
    IgnoreErrorInsufficientPermission = 1003,
    /// No documentation provided.
    IgnoreErrorRetrievingItem = 1004,
    /// No documentation provided.
    IgnoreCannotIgnoreSelf = 1005,
    /// No documentation provided.
    IgnoreIllegalType = 1006,
    /// No documentation provided.
    IgnoreNotFound = 1007,
    /// No documentation provided.
    IgnoreUserGloballyIgnored = 1008,
    /// No documentation provided.
    IgnoreUserIgnored = 1009,
    /// No documentation provided.
    TargetUserIgnored = 1010,
    /// No documentation provided.
    NotificationSettingInvalid = 1100,
    /// No documentation provided.
    PsnApiExpiredAccessToken = 1204,
    /// No documentation provided.
    PSNExForbidden = 1205,
    /// No documentation provided.
    PSNExSystemDisabled = 1218,
    /// No documentation provided.
    PsnApiErrorCodeUnknown = 1223,
    /// No documentation provided.
    PsnApiErrorWebException = 1224,
    /// No documentation provided.
    PsnApiBadRequest = 1225,
    /// No documentation provided.
    PsnApiAccessTokenRequired = 1226,
    /// No documentation provided.
    PsnApiInvalidAccessToken = 1227,
    /// No documentation provided.
    PsnApiBannedUser = 1229,
    /// No documentation provided.
    PsnApiAccountUpgradeRequired = 1230,
    /// No documentation provided.
    PsnApiServiceTemporarilyUnavailable = 1231,
    /// No documentation provided.
    PsnApiServerBusy = 1232,
    /// No documentation provided.
    PsnApiUnderMaintenance = 1233,
    /// No documentation provided.
    PsnApiProfileUserNotFound = 1234,
    /// No documentation provided.
    PsnApiProfilePrivacyRestriction = 1235,
    /// No documentation provided.
    PsnApiProfileUnderMaintenance = 1236,
    /// No documentation provided.
    PsnApiAccountAttributeMissing = 1237,
    /// No documentation provided.
    PsnApiNoPermission = 1238,
    /// No documentation provided.
    PsnApiTargetUserBlocked = 1239,
    /// No documentation provided.
    PsnApiJwksMissing = 1240,
    /// No documentation provided.
    PsnApiJwtMalformedHeader = 1241,
    /// No documentation provided.
    PsnApiJwtMalformedPayload = 1242,
    /// No documentation provided.
    XblExSystemDisabled = 1300,
    /// No documentation provided.
    XblExUnknownError = 1301,
    /// No documentation provided.
    XblApiErrorWebException = 1302,
    /// No documentation provided.
    XblStsTokenInvalid = 1303,
    /// No documentation provided.
    XblStsMissingToken = 1304,
    /// No documentation provided.
    XblStsExpiredToken = 1305,
    /// No documentation provided.
    XblAccessToTheSandboxDenied = 1306,
    /// No documentation provided.
    XblMsaResponseMissing = 1307,
    /// No documentation provided.
    XblMsaAccessTokenExpired = 1308,
    /// No documentation provided.
    XblMsaInvalidRequest = 1309,
    /// No documentation provided.
    XblMsaFriendsRequireSignIn = 1310,
    /// No documentation provided.
    XblUserActionRequired = 1311,
    /// No documentation provided.
    XblParentalControls = 1312,
    /// No documentation provided.
    XblDeveloperAccount = 1313,
    /// No documentation provided.
    XblUserTokenExpired = 1314,
    /// No documentation provided.
    XblUserTokenInvalid = 1315,
    /// No documentation provided.
    XblOffline = 1316,
    /// No documentation provided.
    XblUnknownErrorCode = 1317,
    /// No documentation provided.
    XblMsaInvalidGrant = 1318,
    /// No documentation provided.
    ReportNotYetResolved = 1400,
    /// No documentation provided.
    ReportOverturnDoesNotChangeDecision = 1401,
    /// No documentation provided.
    ReportNotFound = 1402,
    /// No documentation provided.
    ReportAlreadyReported = 1403,
    /// No documentation provided.
    ReportInvalidResolution = 1404,
    /// No documentation provided.
    ReportNotAssignedToYou = 1405,
    /// No documentation provided.
    LegacyGameStatsSystemDisabled = 1500,
    /// No documentation provided.
    LegacyGameStatsUnknownError = 1501,
    /// No documentation provided.
    LegacyGameStatsMalformedSneakerNetCode = 1502,
    /// No documentation provided.
    DestinyAccountAcquisitionFailure = 1600,
    /// No documentation provided.
    DestinyAccountNotFound = 1601,
    /// No documentation provided.
    DestinyBuildStatsDatabaseError = 1602,
    /// No documentation provided.
    DestinyCharacterStatsDatabaseError = 1603,
    /// No documentation provided.
    DestinyPvPStatsDatabaseError = 1604,
    /// No documentation provided.
    DestinyPvEStatsDatabaseError = 1605,
    /// No documentation provided.
    DestinyGrimoireStatsDatabaseError = 1606,
    /// No documentation provided.
    DestinyStatsParameterMembershipTypeParseError = 1607,
    /// No documentation provided.
    DestinyStatsParameterMembershipIdParseError = 1608,
    /// No documentation provided.
    DestinyStatsParameterRangeParseError = 1609,
    /// No documentation provided.
    DestinyStringItemHashNotFound = 1610,
    /// No documentation provided.
    DestinyStringSetNotFound = 1611,
    /// No documentation provided.
    DestinyContentLookupNotFoundForKey = 1612,
    /// No documentation provided.
    DestinyContentItemNotFound = 1613,
    /// No documentation provided.
    DestinyContentSectionNotFound = 1614,
    /// No documentation provided.
    DestinyContentPropertyNotFound = 1615,
    /// No documentation provided.
    DestinyContentConfigNotFound = 1616,
    /// No documentation provided.
    DestinyContentPropertyBucketValueNotFound = 1617,
    /// No documentation provided.
    DestinyUnexpectedError = 1618,
    /// No documentation provided.
    DestinyInvalidAction = 1619,
    /// No documentation provided.
    DestinyCharacterNotFound = 1620,
    /// No documentation provided.
    DestinyInvalidFlag = 1621,
    /// No documentation provided.
    DestinyInvalidRequest = 1622,
    /// No documentation provided.
    DestinyItemNotFound = 1623,
    /// No documentation provided.
    DestinyInvalidCustomizationChoices = 1624,
    /// No documentation provided.
    DestinyVendorItemNotFound = 1625,
    /// No documentation provided.
    DestinyInternalError = 1626,
    /// No documentation provided.
    DestinyVendorNotFound = 1627,
    /// No documentation provided.
    DestinyRecentActivitiesDatabaseError = 1628,
    /// No documentation provided.
    DestinyItemBucketNotFound = 1629,
    /// No documentation provided.
    DestinyInvalidMembershipType = 1630,
    /// No documentation provided.
    DestinyVersionIncompatibility = 1631,
    /// No documentation provided.
    DestinyItemAlreadyInInventory = 1632,
    /// No documentation provided.
    DestinyBucketNotFound = 1633,
    /// Note: This is one of those holdovers from Destiny 1. We didn't change the enum because I am lazy, but in Destiny 2 this would read "DestinyCharacterNotInSocialSpace"
    DestinyCharacterNotInTower = 1634,
    /// No documentation provided.
    DestinyCharacterNotLoggedIn = 1635,
    /// No documentation provided.
    DestinyDefinitionsNotLoaded = 1636,
    /// No documentation provided.
    DestinyInventoryFull = 1637,
    /// No documentation provided.
    DestinyItemFailedLevelCheck = 1638,
    /// No documentation provided.
    DestinyItemFailedUnlockCheck = 1639,
    /// No documentation provided.
    DestinyItemUnequippable = 1640,
    /// No documentation provided.
    DestinyItemUniqueEquipRestricted = 1641,
    /// No documentation provided.
    DestinyNoRoomInDestination = 1642,
    /// No documentation provided.
    DestinyServiceFailure = 1643,
    /// No documentation provided.
    DestinyServiceRetired = 1644,
    /// No documentation provided.
    DestinyTransferFailed = 1645,
    /// No documentation provided.
    DestinyTransferNotFoundForSourceBucket = 1646,
    /// No documentation provided.
    DestinyUnexpectedResultInVendorTransferCheck = 1647,
    /// No documentation provided.
    DestinyUniquenessViolation = 1648,
    /// No documentation provided.
    DestinyErrorDeserializationFailure = 1649,
    /// No documentation provided.
    DestinyValidAccountTicketRequired = 1650,
    /// No documentation provided.
    DestinyShardRelayClientTimeout = 1651,
    /// No documentation provided.
    DestinyShardRelayProxyTimeout = 1652,
    /// No documentation provided.
    DestinyPGCRNotFound = 1653,
    /// No documentation provided.
    DestinyAccountMustBeOffline = 1654,
    /// No documentation provided.
    DestinyCanOnlyEquipInGame = 1655,
    /// No documentation provided.
    DestinyCannotPerformActionOnEquippedItem = 1656,
    /// No documentation provided.
    DestinyQuestAlreadyCompleted = 1657,
    /// No documentation provided.
    DestinyQuestAlreadyTracked = 1658,
    /// No documentation provided.
    DestinyTrackableQuestsFull = 1659,
    /// No documentation provided.
    DestinyItemNotTransferrable = 1660,
    /// No documentation provided.
    DestinyVendorPurchaseNotAllowed = 1661,
    /// No documentation provided.
    DestinyContentVersionMismatch = 1662,
    /// No documentation provided.
    DestinyItemActionForbidden = 1663,
    /// No documentation provided.
    DestinyRefundInvalid = 1664,
    /// No documentation provided.
    DestinyPrivacyRestriction = 1665,
    /// No documentation provided.
    DestinyActionInsufficientPrivileges = 1666,
    /// No documentation provided.
    DestinyInvalidClaimException = 1667,
    /// No documentation provided.
    DestinyLegacyPlatformRestricted = 1668,
    /// No documentation provided.
    DestinyLegacyPlatformInUse = 1669,
    /// No documentation provided.
    DestinyLegacyPlatformInaccessible = 1670,
    /// No documentation provided.
    DestinyCannotPerformActionAtThisLocation = 1671,
    /// No documentation provided.
    DestinyThrottledByGameServer = 1672,
    /// No documentation provided.
    DestinyItemNotTransferrableHasSideEffects = 1673,
    /// No documentation provided.
    DestinyItemLocked = 1674,
    /// No documentation provided.
    DestinyCannotAffordMaterialRequirements = 1675,
    /// No documentation provided.
    DestinyFailedPlugInsertionRules = 1676,
    /// No documentation provided.
    DestinySocketNotFound = 1677,
    /// No documentation provided.
    DestinySocketActionNotAllowed = 1678,
    /// No documentation provided.
    DestinySocketAlreadyHasPlug = 1679,
    /// No documentation provided.
    DestinyPlugItemNotAvailable = 1680,
    /// No documentation provided.
    DestinyCharacterLoggedInNotAllowed = 1681,
    /// No documentation provided.
    DestinyPublicAccountNotAccessible = 1682,
    /// No documentation provided.
    DestinyClaimsItemAlreadyClaimed = 1683,
    /// No documentation provided.
    DestinyClaimsNoInventorySpace = 1684,
    /// No documentation provided.
    DestinyClaimsRequiredLevelNotMet = 1685,
    /// No documentation provided.
    DestinyClaimsInvalidState = 1686,
    /// No documentation provided.
    DestinyNotEnoughRoomForMultipleRewards = 1687,
    /// No documentation provided.
    DestinyDirectBabelClientTimeout = 1688,
    /// No documentation provided.
    FbInvalidRequest = 1800,
    /// No documentation provided.
    FbRedirectMismatch = 1801,
    /// No documentation provided.
    FbAccessDenied = 1802,
    /// No documentation provided.
    FbUnsupportedResponseType = 1803,
    /// No documentation provided.
    FbInvalidScope = 1804,
    /// No documentation provided.
    FbUnsupportedGrantType = 1805,
    /// No documentation provided.
    FbInvalidGrant = 1806,
    /// No documentation provided.
    InvitationExpired = 1900,
    /// No documentation provided.
    InvitationUnknownType = 1901,
    /// No documentation provided.
    InvitationInvalidResponseStatus = 1902,
    /// No documentation provided.
    InvitationInvalidType = 1903,
    /// No documentation provided.
    InvitationAlreadyPending = 1904,
    /// No documentation provided.
    InvitationInsufficientPermission = 1905,
    /// No documentation provided.
    InvitationInvalidCode = 1906,
    /// No documentation provided.
    InvitationInvalidTargetState = 1907,
    /// No documentation provided.
    InvitationCannotBeReactivated = 1908,
    /// No documentation provided.
    InvitationNoRecipients = 1910,
    /// No documentation provided.
    InvitationGroupCannotSendToSelf = 1911,
    /// No documentation provided.
    InvitationTooManyRecipients = 1912,
    /// No documentation provided.
    InvitationInvalid = 1913,
    /// No documentation provided.
    InvitationNotFound = 1914,
    /// No documentation provided.
    TokenInvalid = 2000,
    /// No documentation provided.
    TokenBadFormat = 2001,
    /// No documentation provided.
    TokenAlreadyClaimed = 2002,
    /// No documentation provided.
    TokenAlreadyClaimedSelf = 2003,
    /// No documentation provided.
    TokenThrottling = 2004,
    /// No documentation provided.
    TokenUnknownRedemptionFailure = 2005,
    /// No documentation provided.
    TokenPurchaseClaimFailedAfterTokenClaimed = 2006,
    /// No documentation provided.
    TokenUserAlreadyOwnsOffer = 2007,
    /// No documentation provided.
    TokenInvalidOfferKey = 2008,
    /// No documentation provided.
    TokenEmailNotValidated = 2009,
    /// No documentation provided.
    TokenProvisioningBadVendorOrOffer = 2010,
    /// No documentation provided.
    TokenPurchaseHistoryUnknownError = 2011,
    /// No documentation provided.
    TokenThrottleStateUnknownError = 2012,
    /// No documentation provided.
    TokenUserAgeNotVerified = 2013,
    /// No documentation provided.
    TokenExceededOfferMaximum = 2014,
    /// No documentation provided.
    TokenNoAvailableUnlocks = 2015,
    /// No documentation provided.
    TokenMarketplaceInvalidPlatform = 2016,
    /// No documentation provided.
    TokenNoMarketplaceCodesFound = 2017,
    /// No documentation provided.
    TokenOfferNotAvailableForRedemption = 2018,
    /// No documentation provided.
    TokenUnlockPartialFailure = 2019,
    /// No documentation provided.
    TokenMarketplaceInvalidRegion = 2020,
    /// No documentation provided.
    TokenOfferExpired = 2021,
    /// No documentation provided.
    RAFExceededMaximumReferrals = 2022,
    /// No documentation provided.
    RAFDuplicateBond = 2023,
    /// No documentation provided.
    RAFNoValidVeteranDestinyMembershipsFound = 2024,
    /// No documentation provided.
    RAFNotAValidVeteranUser = 2025,
    /// No documentation provided.
    RAFCodeAlreadyClaimedOrNotFound = 2026,
    /// No documentation provided.
    RAFMismatchedDestinyMembershipType = 2027,
    /// No documentation provided.
    RAFUnableToAccessPurchaseHistory = 2028,
    /// No documentation provided.
    RAFUnableToCreateBond = 2029,
    /// No documentation provided.
    RAFUnableToFindBond = 2030,
    /// No documentation provided.
    RAFUnableToRemoveBond = 2031,
    /// No documentation provided.
    RAFCannotBondToSelf = 2032,
    /// No documentation provided.
    RAFInvalidPlatform = 2033,
    /// No documentation provided.
    RAFGenerateThrottled = 2034,
    /// No documentation provided.
    RAFUnableToCreateBondVersionMismatch = 2035,
    /// No documentation provided.
    RAFUnableToRemoveBondVersionMismatch = 2036,
    /// No documentation provided.
    RAFRedeemThrottled = 2037,
    /// No documentation provided.
    NoAvailableDiscountCode = 2038,
    /// No documentation provided.
    DiscountAlreadyClaimed = 2039,
    /// No documentation provided.
    DiscountClaimFailure = 2040,
    /// No documentation provided.
    DiscountConfigurationFailure = 2041,
    /// No documentation provided.
    DiscountGenerationFailure = 2042,
    /// No documentation provided.
    DiscountAlreadyExists = 2043,
    /// No documentation provided.
    TokenRequiresCredentialXuid = 2044,
    /// No documentation provided.
    TokenRequiresCredentialPsnid = 2045,
    /// No documentation provided.
    OfferRequired = 2046,
    /// No documentation provided.
    UnknownEververseHistoryError = 2047,
    /// No documentation provided.
    MissingEververseHistoryError = 2048,
    /// No documentation provided.
    BungieRewardEmailStateInvalid = 2049,
    /// No documentation provided.
    BungieRewardNotYetClaimable = 2050,
    /// No documentation provided.
    MissingOfferConfig = 2051,
    /// No documentation provided.
    RAFQuestEntitlementRequiresBnet = 2052,
    /// No documentation provided.
    RAFQuestEntitlementTransportFailure = 2053,
    /// No documentation provided.
    RAFQuestEntitlementUnknownFailure = 2054,
    /// No documentation provided.
    RAFVeteranRewardUnknownFailure = 2055,
    /// No documentation provided.
    RAFTooEarlyToCancelBond = 2056,
    /// No documentation provided.
    LoyaltyRewardAlreadyRedeemed = 2057,
    /// No documentation provided.
    UnclaimedLoyaltyRewardEntryNotFound = 2058,
    /// No documentation provided.
    PartnerOfferPartialFailure = 2059,
    /// No documentation provided.
    PartnerOfferAlreadyClaimed = 2060,
    /// No documentation provided.
    PartnerOfferSkuNotFound = 2061,
    /// No documentation provided.
    PartnerOfferSkuExpired = 2062,
    /// No documentation provided.
    PartnerOfferPermissionFailure = 2063,
    /// No documentation provided.
    PartnerOfferNoDestinyAccount = 2064,
    /// No documentation provided.
    PartnerOfferApplyDataNotFound = 2065,
    /// No documentation provided.
    ApiExceededMaxKeys = 2100,
    /// No documentation provided.
    ApiInvalidOrExpiredKey = 2101,
    /// No documentation provided.
    ApiKeyMissingFromRequest = 2102,
    /// No documentation provided.
    ApplicationDisabled = 2103,
    /// No documentation provided.
    ApplicationExceededMax = 2104,
    /// No documentation provided.
    ApplicationDisallowedByScope = 2105,
    /// No documentation provided.
    AuthorizationCodeInvalid = 2106,
    /// No documentation provided.
    OriginHeaderDoesNotMatchKey = 2107,
    /// No documentation provided.
    AccessNotPermittedByApplicationScope = 2108,
    /// No documentation provided.
    ApplicationNameIsTaken = 2109,
    /// No documentation provided.
    RefreshTokenNotYetValid = 2110,
    /// No documentation provided.
    AccessTokenHasExpired = 2111,
    /// No documentation provided.
    ApplicationTokenFormatNotValid = 2112,
    /// No documentation provided.
    ApplicationNotConfiguredForBungieAuth = 2113,
    /// No documentation provided.
    ApplicationNotConfiguredForOAuth = 2114,
    /// No documentation provided.
    OAuthAccessTokenExpired = 2115,
    /// No documentation provided.
    ApplicationTokenKeyIdDoesNotExist = 2116,
    /// No documentation provided.
    ProvidedTokenNotValidRefreshToken = 2117,
    /// No documentation provided.
    RefreshTokenExpired = 2118,
    /// No documentation provided.
    AuthorizationRecordInvalid = 2119,
    /// No documentation provided.
    TokenPreviouslyRevoked = 2120,
    /// No documentation provided.
    TokenInvalidMembership = 2121,
    /// No documentation provided.
    AuthorizationCodeStale = 2122,
    /// No documentation provided.
    AuthorizationRecordExpired = 2123,
    /// No documentation provided.
    AuthorizationRecordRevoked = 2124,
    /// No documentation provided.
    AuthorizationRecordInactiveApiKey = 2125,
    /// No documentation provided.
    AuthorizationRecordApiKeyMatching = 2126,
    /// No documentation provided.
    PartnershipInvalidType = 2200,
    /// No documentation provided.
    PartnershipValidationError = 2201,
    /// No documentation provided.
    PartnershipValidationTimeout = 2202,
    /// No documentation provided.
    PartnershipAccessFailure = 2203,
    /// No documentation provided.
    PartnershipAccountInvalid = 2204,
    /// No documentation provided.
    PartnershipGetAccountInfoFailure = 2205,
    /// No documentation provided.
    PartnershipDisabled = 2206,
    /// No documentation provided.
    PartnershipAlreadyExists = 2207,
    /// No documentation provided.
    CommunityStreamingUnavailable = 2300,
    /// No documentation provided.
    TwitchNotLinked = 2500,
    /// No documentation provided.
    TwitchAccountNotFound = 2501,
    /// No documentation provided.
    TwitchCouldNotLoadDestinyInfo = 2502,
    /// No documentation provided.
    TwitchCouldNotRegisterUser = 2503,
    /// No documentation provided.
    TwitchCouldNotUnregisterUser = 2504,
    /// No documentation provided.
    TwitchRequiresRelinking = 2505,
    /// No documentation provided.
    TwitchNoPlatformChosen = 2506,
    /// No documentation provided.
    TwitchDropHistoryPermissionFailure = 2507,
    /// No documentation provided.
    TwitchDropsRepairPartialFailure = 2508,
    /// No documentation provided.
    TwitchNotAuthorized = 2509,
    /// No documentation provided.
    TwitchUnknownAuthorizationFailure = 2510,
    /// No documentation provided.
    TrendingCategoryNotFound = 2600,
    /// No documentation provided.
    TrendingEntryTypeNotSupported = 2601,
    /// No documentation provided.
    ReportOffenderNotInPgcr = 2700,
    /// No documentation provided.
    ReportRequestorNotInPgcr = 2701,
    /// No documentation provided.
    ReportSubmissionFailed = 2702,
    /// No documentation provided.
    ReportCannotReportSelf = 2703,
    /// No documentation provided.
    AwaTypeDisabled = 2800,
    /// No documentation provided.
    AwaTooManyPendingRequests = 2801,
    /// No documentation provided.
    AwaTheFeatureRequiresARegisteredDevice = 2802,
    /// No documentation provided.
    AwaRequestWasUnansweredForTooLong = 2803,
    /// No documentation provided.
    AwaWriteRequestMissingOrInvalidToken = 2804,
    /// No documentation provided.
    AwaWriteRequestTokenExpired = 2805,
    /// No documentation provided.
    AwaWriteRequestTokenUsageLimitReached = 2806,
    /// No documentation provided.
    SteamWebApiError = 2900,
    /// No documentation provided.
    SteamWebNullResponseError = 2901,
    /// No documentation provided.
    SteamAccountRequired = 2902,
    /// No documentation provided.
    SteamNotAuthorized = 2903,
    /// No documentation provided.
    ClanFireteamNotFound = 3000,
    /// No documentation provided.
    ClanFireteamAddNoAlternatesForImmediate = 3001,
    /// No documentation provided.
    ClanFireteamFull = 3002,
    /// No documentation provided.
    ClanFireteamAltFull = 3003,
    /// No documentation provided.
    ClanFireteamBlocked = 3004,
    /// No documentation provided.
    ClanFireteamPlayerEntryNotFound = 3005,
    /// No documentation provided.
    ClanFireteamPermissions = 3006,
    /// No documentation provided.
    ClanFireteamInvalidPlatform = 3007,
    /// No documentation provided.
    ClanFireteamCannotAdjustSlotCount = 3008,
    /// No documentation provided.
    ClanFireteamInvalidPlayerPlatform = 3009,
    /// No documentation provided.
    ClanFireteamNotReadyForInvitesNotEnoughPlayers = 3010,
    /// No documentation provided.
    ClanFireteamGameInvitesNotSupportForPlatform = 3011,
    /// No documentation provided.
    ClanFireteamPlatformInvitePreqFailure = 3012,
    /// No documentation provided.
    ClanFireteamInvalidAuthContext = 3013,
    /// No documentation provided.
    ClanFireteamInvalidAuthProviderPsn = 3014,
    /// No documentation provided.
    ClanFireteamPs4SessionFull = 3015,
    /// No documentation provided.
    ClanFireteamInvalidAuthToken = 3016,
    /// No documentation provided.
    ClanFireteamScheduledFireteamsDisabled = 3017,
    /// No documentation provided.
    ClanFireteamNotReadyForInvitesNotScheduledYet = 3018,
    /// No documentation provided.
    ClanFireteamNotReadyForInvitesClosed = 3019,
    /// No documentation provided.
    ClanFireteamScheduledFireteamsRequireAdminPermissions = 3020,
    /// No documentation provided.
    ClanFireteamNonPublicMustHaveClan = 3021,
    /// No documentation provided.
    ClanFireteamPublicCreationRestriction = 3022,
    /// No documentation provided.
    ClanFireteamAlreadyJoined = 3023,
    /// No documentation provided.
    ClanFireteamScheduledFireteamsRange = 3024,
    /// No documentation provided.
    ClanFireteamPublicCreationRestrictionExtended = 3025,
    /// No documentation provided.
    ClanFireteamExpired = 3026,
    /// No documentation provided.
    ClanFireteamInvalidAuthProvider = 3027,
    /// No documentation provided.
    ClanFireteamInvalidAuthProviderXuid = 3028,
    /// No documentation provided.
    ClanFireteamThrottle = 3029,
    /// No documentation provided.
    ClanFireteamTooManyOpenScheduledFireteams = 3030,
    /// No documentation provided.
    ClanFireteamCannotReopenScheduledFireteams = 3031,
    /// No documentation provided.
    ClanFireteamJoinNoAccountSpecified = 3032,
    /// No documentation provided.
    ClanFireteamMinDestiny2ProgressForCreation = 3033,
    /// No documentation provided.
    ClanFireteamMinDestiny2ProgressForJoin = 3034,
    /// No documentation provided.
    ClanFireteamSMSOrPurchaseRequiredCreate = 3035,
    /// No documentation provided.
    ClanFireteamPurchaseRequiredCreate = 3036,
    /// No documentation provided.
    ClanFireteamSMSOrPurchaseRequiredJoin = 3037,
    /// No documentation provided.
    ClanFireteamPurchaseRequiredJoin = 3038,
    /// No documentation provided.
    CrossSaveOverriddenAccountNotFound = 3200,
    /// No documentation provided.
    CrossSaveTooManyOverriddenPlatforms = 3201,
    /// No documentation provided.
    CrossSaveNoOverriddenPlatforms = 3202,
    /// No documentation provided.
    CrossSavePrimaryAccountNotFound = 3203,
    /// No documentation provided.
    CrossSaveRequestInvalid = 3204,
    /// No documentation provided.
    CrossSaveBungieAccountValidationFailure = 3206,
    /// No documentation provided.
    CrossSaveOverriddenPlatformNotAllowed = 3207,
    /// No documentation provided.
    CrossSaveThresholdExceeded = 3208,
    /// No documentation provided.
    CrossSaveIncompatibleMembershipType = 3209,
    /// No documentation provided.
    CrossSaveCouldNotFindLinkedAccountForMembershipType = 3210,
    /// No documentation provided.
    CrossSaveCouldNotCreateDestinyProfileForMembershipType = 3211,
    /// No documentation provided.
    CrossSaveErrorCreatingDestinyProfileForMembershipType = 3212,
    /// No documentation provided.
    CrossSaveCannotOverrideSelf = 3213,
    /// No documentation provided.
    CrossSaveRecentSilverPurchase = 3214,
    /// No documentation provided.
    CrossSaveSilverBalanceNegative = 3215,
    /// No documentation provided.
    CrossSaveAccountNotAuthenticated = 3216,
    /// No documentation provided.
    ErrorOneAccountAlreadyActive = 3217,
    /// No documentation provided.
    ErrorOneAccountDestinyRestriction = 3218,
    /// No documentation provided.
    CrossSaveMustMigrateToSteam = 3219,
    /// No documentation provided.
    CrossSaveSteamAlreadyPaired = 3220,
    /// No documentation provided.
    CrossSaveCannotPairJustSteamAndBlizzard = 3221,
    /// No documentation provided.
    CrossSaveCannotPairSteamAloneBeforeShadowkeep = 3222,
    /// No documentation provided.
    AuthVerificationNotLinkedToAccount = 3300,
    /// No documentation provided.
    PCMigrationMissingBlizzard = 3400,
    /// No documentation provided.
    PCMigrationMissingSteam = 3401,
    /// No documentation provided.
    PCMigrationInvalidBlizzard = 3402,
    /// No documentation provided.
    PCMigrationInvalidSteam = 3403,
    /// No documentation provided.
    PCMigrationUnknownFailure = 3404,
    /// No documentation provided.
    PCMigrationUnknownException = 3405,
    /// No documentation provided.
    PCMigrationNotLinked = 3406,
    /// No documentation provided.
    PCMigrationAccountsAlreadyUsed = 3407,
    /// No documentation provided.
    PCMigrationStepFailed = 3408,
    /// No documentation provided.
    PCMigrationInvalidBlizzardCrossSaveState = 3409,
    /// No documentation provided.
    PCMigrationDestinationBanned = 3410,
    /// No documentation provided.
    PCMigrationDestinyFailure = 3411,
    /// No documentation provided.
    PCMigrationSilverTransferFailed = 3412,
    /// No documentation provided.
    PCMigrationEntitlementTransferFailed = 3413,
    /// No documentation provided.
    PCMigrationCannotStompClanFounder = 3414,
    /// No documentation provided.
    UnsupportedBrowser = 3500,
    /// No documentation provided.
    StadiaAccountRequired = 3600,
    /// No documentation provided.
    ErrorPhoneValidationTooManyUses = 3702,
    /// No documentation provided.
    ErrorPhoneValidationNoAssociatedPhone = 3703,
    /// No documentation provided.
    ErrorPhoneValidationCodeInvalid = 3705,
    /// No documentation provided.
    ErrorPhoneValidationBanned = 3706,
    /// No documentation provided.
    ErrorPhoneValidationCodeTooRecentlySent = 3707,
    /// No documentation provided.
    ErrorPhoneValidationCodeExpired = 3708,
    /// No documentation provided.
    ErrorPhoneValidationInvalidNumberType = 3709,
    /// No documentation provided.
    ErrorPhoneValidationCodeTooRecentlyChecked = 3710,
    /// No documentation provided.
    ApplePushErrorUnknown = 3800,
    /// No documentation provided.
    ApplePushErrorNull = 3801,
    /// No documentation provided.
    ApplePushErrorTimeout = 3802,
    /// No documentation provided.
    ApplePushBadRequest = 3803,
    /// No documentation provided.
    ApplePushFailedAuth = 3804,
    /// No documentation provided.
    ApplePushThrottled = 3805,
    /// No documentation provided.
    ApplePushServiceUnavailable = 3806,
    /// No documentation provided.
    NotAnImageOrVideo = 3807,
    /// No documentation provided.
    ErrorBungieFriendsBlockFailed = 3900,
    /// No documentation provided.
    ErrorBungieFriendsAutoReject = 3901,
    /// No documentation provided.
    ErrorBungieFriendsNoRequestFound = 3902,
    /// No documentation provided.
    ErrorBungieFriendsAlreadyFriends = 3903,
    /// No documentation provided.
    ErrorBungieFriendsUnableToRemoveRequest = 3904,
    /// No documentation provided.
    ErrorBungieFriendsUnableToRemove = 3905,
    /// No documentation provided.
    ErrorBungieFriendsIdenticalSourceTarget = 3906,
    /// No documentation provided.
    ErrorBungieFriendsSelf = 3907,
    /// No documentation provided.
    ErrorBungieBlockSelf = 3908,
    /// No documentation provided.
    ErrorBungieFriendsListFull = 3910,
    /// No documentation provided.
    ErrorBungieBlockListFull = 3911,
    /// No documentation provided.
    ErrorEgsUnknown = 4000,
    /// No documentation provided.
    ErrorEgsBadRequest = 4001,
    /// No documentation provided.
    ErrorEgsNotAuthorized = 4002,
    /// No documentation provided.
    ErrorEgsForbidden = 4003,
    /// No documentation provided.
    ErrorEgsAccountNotFound = 4004,
    /// No documentation provided.
    ErrorEgsWebException = 4005,
    /// No documentation provided.
    ErrorEgsUnavailable = 4006,
    /// No documentation provided.
    ErrorEgsJwksMissing = 4007,
    /// No documentation provided.
    ErrorEgsJwtMalformedHeader = 4008,
    /// No documentation provided.
    ErrorEgsJwtMalformedPayload = 4009,
}
