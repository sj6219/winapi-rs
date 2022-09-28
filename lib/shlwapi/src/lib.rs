// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! FFI bindings to shlwapi.
#![cfg(windows)]
extern crate winapi;
use winapi::*;
#[cfg(target_pointer_width = "32")]
extern "cdecl" {
    // pub fn ShellMessageBoxA();
    // pub fn ShellMessageBoxW();
    // pub fn wnsprintfA();
    // pub fn wnsprintfW();
}
#[cfg(any(target_arch = "x86", target_arch = "x86_64", target_arch = "arm"))]
extern "system" {
    // pub fn AssocCreate();
    // pub fn AssocGetPerceivedType();
    // pub fn AssocIsDangerous();
    // pub fn AssocQueryKeyA();
    // pub fn AssocQueryKeyW();
    // pub fn AssocQueryStringA();
    // pub fn AssocQueryStringByKeyA();
    // pub fn AssocQueryStringByKeyW();
    // pub fn AssocQueryStringW();
    // pub fn ChrCmpIA();
    // pub fn ChrCmpIW();
    // pub fn ColorAdjustLuma();
    // pub fn ColorHLSToRGB();
    // pub fn ColorRGBToHLS();
    // pub fn ConnectToConnectionPoint();
    // pub fn GetAcceptLanguagesA();
    // pub fn GetAcceptLanguagesW();
    // pub fn GetMenuPosFromID();
    // pub fn HashData();
    // pub fn IStream_Copy();
    // pub fn IStream_Read();
    // pub fn IStream_ReadPidl();
    // pub fn IStream_ReadStr();
    // pub fn IStream_Reset();
    // pub fn IStream_Size();
    // pub fn IStream_Write();
    // pub fn IStream_WritePidl();
    // pub fn IStream_WriteStr();
    // pub fn IUnknown_AtomicRelease();
    // pub fn IUnknown_GetSite();
    // pub fn IUnknown_GetWindow();
    // pub fn IUnknown_QueryService();
    // pub fn IUnknown_Set();
    // pub fn IUnknown_SetSite();
    // pub fn IntlStrEqWorkerA();
    // pub fn IntlStrEqWorkerW();
    // pub fn IsCharSpaceA();
    // pub fn IsCharSpaceW();
    // pub fn IsInternetESCEnabled();
    // pub fn IsOS();
    // pub fn MLLoadLibraryA();
    // pub fn MLLoadLibraryW();
    // pub fn ParseURLA();
    // pub fn ParseURLW();
    // pub fn PathAddBackslashA();
    // pub fn PathAddBackslashW();
    // pub fn PathAddExtensionA();
    // pub fn PathAddExtensionW();
    // pub fn PathAppendA();
    // pub fn PathAppendW();
    // pub fn PathBuildRootA();
    // pub fn PathBuildRootW();
    // pub fn PathCanonicalizeA();
    // pub fn PathCanonicalizeW();
    // pub fn PathCombineA();
    // pub fn PathCombineW();
    // pub fn PathCommonPrefixA();
    // pub fn PathCommonPrefixW();
    // pub fn PathCompactPathA();
    // pub fn PathCompactPathExA();
    // pub fn PathCompactPathExW();
    // pub fn PathCompactPathW();
    // pub fn PathCreateFromUrlA();
    // pub fn PathCreateFromUrlAlloc();
    // pub fn PathCreateFromUrlW();
    // pub fn PathFileExistsA();
    // pub fn PathFileExistsW();
    // pub fn PathFindExtensionA();
    // pub fn PathFindExtensionW();
    // pub fn PathFindFileNameA();
    // pub fn PathFindFileNameW();
    // pub fn PathFindNextComponentA();
    // pub fn PathFindNextComponentW();
    // pub fn PathFindOnPathA();
    // pub fn PathFindOnPathW();
    // pub fn PathFindSuffixArrayA();
    // pub fn PathFindSuffixArrayW();
    // pub fn PathGetArgsA();
    // pub fn PathGetArgsW();
    // pub fn PathGetCharTypeA();
    // pub fn PathGetCharTypeW();
    // pub fn PathGetDriveNumberA();
    // pub fn PathGetDriveNumberW();
    // pub fn PathIsContentTypeA();
    // pub fn PathIsContentTypeW();
    // pub fn PathIsDirectoryA();
    // pub fn PathIsDirectoryEmptyA();
    // pub fn PathIsDirectoryEmptyW();
    // pub fn PathIsDirectoryW();
    // pub fn PathIsFileSpecA();
    // pub fn PathIsFileSpecW();
    // pub fn PathIsLFNFileSpecA();
    // pub fn PathIsLFNFileSpecW();
    // pub fn PathIsNetworkPathA();
    // pub fn PathIsNetworkPathW();
    // pub fn PathIsPrefixA();
    // pub fn PathIsPrefixW();
    // pub fn PathIsRelativeA();
    // pub fn PathIsRelativeW();
    // pub fn PathIsRootA();
    // pub fn PathIsRootW();
    // pub fn PathIsSameRootA();
    // pub fn PathIsSameRootW();
    // pub fn PathIsSystemFolderA();
    // pub fn PathIsSystemFolderW();
    // pub fn PathIsUNCA();
    // pub fn PathIsUNCServerA();
    // pub fn PathIsUNCServerShareA();
    // pub fn PathIsUNCServerShareW();
    // pub fn PathIsUNCServerW();
    // pub fn PathIsUNCW();
    // pub fn PathIsURLA();
    // pub fn PathIsURLW();
    // pub fn PathMakePrettyA();
    // pub fn PathMakePrettyW();
    // pub fn PathMakeSystemFolderA();
    // pub fn PathMakeSystemFolderW();
    // pub fn PathMatchSpecA();
    // pub fn PathMatchSpecExA();
    // pub fn PathMatchSpecExW();
    // pub fn PathMatchSpecW();
    // pub fn PathParseIconLocationA();
    // pub fn PathParseIconLocationW();
    // pub fn PathQuoteSpacesA();
    // pub fn PathQuoteSpacesW();
    // pub fn PathRelativePathToA();
    // pub fn PathRelativePathToW();
    // pub fn PathRemoveArgsA();
    // pub fn PathRemoveArgsW();
    // pub fn PathRemoveBackslashA();
    // pub fn PathRemoveBackslashW();
    // pub fn PathRemoveBlanksA();
    // pub fn PathRemoveBlanksW();
    // pub fn PathRemoveExtensionA();
    // pub fn PathRemoveExtensionW();
    // pub fn PathRemoveFileSpecA();
    // pub fn PathRemoveFileSpecW();
    // pub fn PathRenameExtensionA();
    // pub fn PathRenameExtensionW();
    // pub fn PathSearchAndQualifyA();
    // pub fn PathSearchAndQualifyW();
    // pub fn PathSetDlgItemPathA();
    // pub fn PathSetDlgItemPathW();
    // pub fn PathSkipRootA();
    // pub fn PathSkipRootW();
    // pub fn PathStripPathA();
    // pub fn PathStripPathW();
    // pub fn PathStripToRootA();
    // pub fn PathStripToRootW();
    // pub fn PathUnExpandEnvStringsA();
    // pub fn PathUnExpandEnvStringsW();
    // pub fn PathUndecorateA();
    // pub fn PathUndecorateW();
    // pub fn PathUnmakeSystemFolderA();
    // pub fn PathUnmakeSystemFolderW();
    // pub fn PathUnquoteSpacesA();
    // pub fn PathUnquoteSpacesW();
    // pub fn QISearch();
    // pub fn SHAllocShared();
    // pub fn SHAnsiToAnsi();
    // pub fn SHAnsiToUnicode();
    // pub fn SHAutoComplete();
    // pub fn SHCopyKeyA();
    // pub fn SHCopyKeyW();
    // pub fn SHCreateMemStream();
    // pub fn SHCreateShellPalette();
    // pub fn SHCreateStreamOnFileA();
    // pub fn SHCreateStreamOnFileEx();
    // pub fn SHCreateStreamOnFileW();
    // pub fn SHCreateThread();
    // pub fn SHCreateThreadRef();
    // pub fn SHCreateThreadWithHandle();
    // pub fn SHDeleteEmptyKeyA();
    // pub fn SHDeleteEmptyKeyW();
    // pub fn SHDeleteKeyA();
    // pub fn SHDeleteKeyW();
    // pub fn SHDeleteValueA();
    // pub fn SHDeleteValueW();
    // pub fn SHEnumKeyExA();
    // pub fn SHEnumKeyExW();
    // pub fn SHEnumValueA();
    // pub fn SHEnumValueW();
    // pub fn SHFormatDateTimeA();
    // pub fn SHFormatDateTimeW();
    // pub fn SHFreeShared();
    // pub fn SHGetInverseCMAP();
    // pub fn SHGetThreadRef();
    // pub fn SHGetValueA();
    // pub fn SHGetValueW();
    // pub fn SHGetViewStatePropertyBag();
    // pub fn SHGlobalCounterDecrement();
    // pub fn SHGlobalCounterGetValue();
    // pub fn SHGlobalCounterIncrement();
    // pub fn SHIsChildOrSelf();
    // pub fn SHIsLowMemoryMachine();
    // pub fn SHLoadIndirectString();
    // pub fn SHLockShared();
    // pub fn SHMessageBoxCheckA();
    // pub fn SHMessageBoxCheckW();
    // pub fn SHOpenRegStream2A();
    // pub fn SHOpenRegStream2W();
    // pub fn SHOpenRegStreamA();
    // pub fn SHOpenRegStreamW();
    // pub fn SHPropertyBag_ReadStrAlloc();
    // pub fn SHPropertyBag_WriteBSTR();
    // pub fn SHQueryInfoKeyA();
    // pub fn SHQueryInfoKeyW();
    // pub fn SHQueryValueExA();
    // pub fn SHQueryValueExW();
    // pub fn SHRegCloseUSKey();
    // pub fn SHRegCreateUSKeyA();
    // pub fn SHRegCreateUSKeyW();
    // pub fn SHRegDeleteEmptyUSKeyA();
    // pub fn SHRegDeleteEmptyUSKeyW();
    // pub fn SHRegDeleteUSValueA();
    // pub fn SHRegDeleteUSValueW();
    // pub fn SHRegDuplicateHKey();
    // pub fn SHRegEnumUSKeyA();
    // pub fn SHRegEnumUSKeyW();
    // pub fn SHRegEnumUSValueA();
    // pub fn SHRegEnumUSValueW();
    // pub fn SHRegGetBoolUSValueA();
    // pub fn SHRegGetBoolUSValueW();
    // pub fn SHRegGetIntW();
    // pub fn SHRegGetPathA();
    // pub fn SHRegGetPathW();
    // pub fn SHRegGetUSValueA();
    // pub fn SHRegGetUSValueW();
    // pub fn SHRegGetValueA();
    // pub fn SHRegGetValueW();
    // pub fn SHRegOpenUSKeyA();
    // pub fn SHRegOpenUSKeyW();
    // pub fn SHRegQueryInfoUSKeyA();
    // pub fn SHRegQueryInfoUSKeyW();
    // pub fn SHRegQueryUSValueA();
    // pub fn SHRegQueryUSValueW();
    // pub fn SHRegSetPathA();
    // pub fn SHRegSetPathW();
    // pub fn SHRegSetUSValueA();
    // pub fn SHRegSetUSValueW();
    // pub fn SHRegWriteUSValueA();
    // pub fn SHRegWriteUSValueW();
    // pub fn SHReleaseThreadRef();
    // pub fn SHRunIndirectRegClientCommand();
    // pub fn SHSendMessageBroadcastA();
    // pub fn SHSendMessageBroadcastW();
    // pub fn SHSetThreadRef();
    // pub fn SHSetValueA();
    // pub fn SHSetValueW();
    // pub fn SHSkipJunction();
    // pub fn SHStrDupA();
    // pub fn SHStrDupW();
    // pub fn SHStripMneumonicA();
    // pub fn SHStripMneumonicW();
    // pub fn SHUnicodeToAnsi();
    // pub fn SHUnicodeToUnicode();
    // pub fn SHUnlockShared();
    // pub fn StrCSpnA();
    // pub fn StrCSpnIA();
    // pub fn StrCSpnIW();
    // pub fn StrCSpnW();
    // pub fn StrCatBuffA();
    // pub fn StrCatBuffW();
    // pub fn StrCatChainW();
    // pub fn StrCatW();
    // pub fn StrChrA();
    // pub fn StrChrIA();
    // pub fn StrChrIW();
    // pub fn StrChrNIW();
    // pub fn StrChrNW();
    // pub fn StrChrW();
    // pub fn StrCmpCA();
    // pub fn StrCmpCW();
    // pub fn StrCmpICA();
    // pub fn StrCmpICW();
    // pub fn StrCmpIW();
    // pub fn StrCmpLogicalW();
    // pub fn StrCmpNA();
    // pub fn StrCmpNCA();
    // pub fn StrCmpNCW();
    // pub fn StrCmpNIA();
    // pub fn StrCmpNICA();
    // pub fn StrCmpNICW();
    // pub fn StrCmpNIW();
    // pub fn StrCmpNW();
    // pub fn StrCmpW();
    // pub fn StrCpyNW();
    // pub fn StrCpyW();
    // pub fn StrDupA();
    // pub fn StrDupW();
    // pub fn StrFormatByteSize64A();
    // pub fn StrFormatByteSizeA();
    // pub fn StrFormatByteSizeEx();
    // pub fn StrFormatByteSizeW();
    // pub fn StrFormatKBSizeA();
    // pub fn StrFormatKBSizeW();
    // pub fn StrFromTimeIntervalA();
    // pub fn StrFromTimeIntervalW();
    // pub fn StrIsIntlEqualA();
    // pub fn StrIsIntlEqualW();
    // pub fn StrNCatA();
    // pub fn StrNCatW();
    // pub fn StrPBrkA();
    // pub fn StrPBrkW();
    // pub fn StrRChrA();
    // pub fn StrRChrIA();
    // pub fn StrRChrIW();
    // pub fn StrRChrW();
    // pub fn StrRStrIA();
    // pub fn StrRStrIW();
    // pub fn StrRetToBSTR();
    // pub fn StrRetToBufA();
    // pub fn StrRetToBufW();
    // pub fn StrRetToStrA();
    // pub fn StrRetToStrW();
    // pub fn StrSpnA();
    // pub fn StrSpnW();
    // pub fn StrStrA();
    // pub fn StrStrIA();
    // pub fn StrStrIW();
    // pub fn StrStrNIW();
    // pub fn StrStrNW();
    // pub fn StrStrW();
    // pub fn StrToInt64ExA();
    // pub fn StrToInt64ExW();
    // pub fn StrToIntA();
    // pub fn StrToIntExA();
    // pub fn StrToIntExW();
    // pub fn StrToIntW();
    // pub fn StrTrimA();
    // pub fn StrTrimW();
    // pub fn UrlApplySchemeA();
    // pub fn UrlApplySchemeW();
    // pub fn UrlCanonicalizeA();
    // pub fn UrlCanonicalizeW();
    // pub fn UrlCombineA();
    // pub fn UrlCombineW();
    // pub fn UrlCompareA();
    // pub fn UrlCompareW();
    // pub fn UrlCreateFromPathA();
    // pub fn UrlCreateFromPathW();
    // pub fn UrlEscapeA();
    // pub fn UrlEscapeW();
    // pub fn UrlFixupW();
    // pub fn UrlGetLocationA();
    // pub fn UrlGetLocationW();
    // pub fn UrlGetPartA();
    // pub fn UrlGetPartW();
    // pub fn UrlHashA();
    // pub fn UrlHashW();
    // pub fn UrlIsA();
    // pub fn UrlIsNoHistoryA();
    // pub fn UrlIsNoHistoryW();
    // pub fn UrlIsOpaqueA();
    // pub fn UrlIsOpaqueW();
    // pub fn UrlIsW();
    // pub fn UrlUnescapeA();
    // pub fn UrlUnescapeW();
    // pub fn WhichPlatform();
    // pub fn wvnsprintfA();
    // pub fn wvnsprintfW();
}
#[cfg(any(target_arch = "x86_64", target_arch = "arm"))]
extern "system" {
    // pub fn ShellMessageBoxA();
    // pub fn ShellMessageBoxW();
    // pub fn wnsprintfA();
    // pub fn wnsprintfW();
}