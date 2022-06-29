// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
#![cfg(windows)]
extern crate user32;
use user32::*;
#[inline(never)] fn bb<T>(_: T) {}
#[cfg(target_pointer_width = "64")]
#[test]
fn functions_x64() {
    bb(GetClassLongPtrA);
    bb(GetClassLongPtrW);
    bb(GetWindowLongPtrA);
    bb(GetWindowLongPtrW);
    bb(SetClassLongPtrA);
    bb(SetClassLongPtrW);
    bb(SetWindowLongPtrA);
    bb(SetWindowLongPtrW);
}
#[test]
fn functions() {
    bb(ActivateKeyboardLayout);
    // bb(AddClipboardFormatListener);
    bb(AdjustWindowRect);
    bb(AdjustWindowRectEx);
    bb(AllowSetForegroundWindow);
    bb(AnimateWindow);
    bb(AnyPopup);
    bb(ArrangeIconicWindows);
    bb(AttachThreadInput);
    bb(BeginPaint);
    bb(BlockInput);
    bb(BringWindowToTop);
    // bb(CalculatePopupWindowPosition);
    bb(CallMsgFilterA);
    bb(CallMsgFilterW);
    bb(CallNextHookEx);
    bb(CallWindowProcA);
    bb(CallWindowProcW);
    bb(CascadeWindows);
    bb(ChangeClipboardChain);
    bb(ChangeDisplaySettingsA);
    bb(ChangeDisplaySettingsExA);
    bb(ChangeDisplaySettingsExW);
    bb(ChangeDisplaySettingsW);
    bb(ChangeMenuA);
    bb(ChangeMenuW);
    // bb(ChangeWindowMessageFilter);
    // bb(ChangeWindowMessageFilterEx);
    bb(CharLowerA);
    bb(CharLowerBuffA);
    bb(CharLowerBuffW);
    bb(CharLowerW);
    bb(CharNextA);
    bb(CharNextExA);
    bb(CharNextW);
    bb(CharPrevA);
    bb(CharPrevExA);
    bb(CharPrevW);
    bb(CharUpperA);
    bb(CharUpperBuffA);
    bb(CharUpperBuffW);
    bb(CharUpperW);
    bb(ChildWindowFromPoint);
    bb(ChildWindowFromPointEx);
    bb(ClipCursor);
    bb(CloseClipboard);
    bb(CloseDesktop);
    bb(CloseWindow);
    bb(CloseWindowStation);
    bb(CopyAcceleratorTableA);
    bb(CopyAcceleratorTableW);
    bb(CopyIcon);
    bb(CopyImage);
    bb(CopyRect);
    bb(CountClipboardFormats);
    bb(CreateAcceleratorTableA);
    bb(CreateAcceleratorTableW);
    bb(CreateCaret);
    bb(CreateCursor);
    bb(CreateDesktopA);
    bb(CreateDesktopW);
    bb(CreateDialogIndirectParamA);
    bb(CreateDialogIndirectParamW);
    bb(CreateDialogParamA);
    bb(CreateDialogParamW);
    bb(CreateIcon);
    bb(CreateIconFromResource);
    bb(CreateIconFromResourceEx);
    bb(CreateIconIndirect);
    bb(CreateMDIWindowA);
    bb(CreateMDIWindowW);
    bb(CreateMenu);
    bb(CreatePopupMenu);
    bb(CreateWindowExA);
    bb(CreateWindowExW);
    bb(CreateWindowStationA);
    bb(CreateWindowStationW);
    bb(DefDlgProcA);
    bb(DefDlgProcW);
    bb(DefFrameProcA);
    bb(DefFrameProcW);
    bb(DefMDIChildProcA);
    bb(DefMDIChildProcW);
    bb(DefRawInputProc);
    bb(DefWindowProcA);
    bb(DefWindowProcW);
    bb(DeferWindowPos);
    bb(DeleteMenu);
    bb(DeregisterShellHookWindow);
    bb(DestroyAcceleratorTable);
    bb(DestroyCaret);
    bb(DestroyCursor);
    bb(DestroyIcon);
    bb(DestroyMenu);
    bb(DestroyWindow);
    bb(DialogBoxIndirectParamA);
    bb(DialogBoxIndirectParamW);
    bb(DialogBoxParamA);
    bb(DialogBoxParamW);
    bb(DispatchMessageA);
    bb(DispatchMessageW);
    bb(DragDetect);
    bb(DragObject);
    bb(DrawAnimatedRects);
    bb(DrawCaption);
    bb(DrawEdge);
    bb(DrawFocusRect);
    bb(DrawFrameControl);
    bb(DrawIcon);
    bb(DrawIconEx);
    bb(DrawMenuBar);
    bb(DrawStateA);
    bb(DrawStateW);
    bb(DrawTextA);
    bb(DrawTextExA);
    bb(DrawTextExW);
    bb(DrawTextW);
    bb(EmptyClipboard);
    bb(EnableMenuItem);
    //bb(EnableMouseInPointer);
    bb(EnableScrollBar);
    bb(EnableWindow);
    bb(EndDeferWindowPos);
    bb(EndDialog);
    //bb(EndMenu);
    bb(EndPaint);
    bb(EndTask);
    bb(EnumClipboardFormats);
    bb(EnumDesktopWindows);
    bb(EnumDesktopsA);
    bb(EnumDesktopsW);
    bb(EnumDisplayDevicesA);
    bb(EnumDisplayDevicesW);
    bb(EnumDisplayMonitors);
    bb(EnumDisplaySettingsA);
    bb(EnumDisplaySettingsExA);
    bb(EnumDisplaySettingsExW);
    bb(EnumDisplaySettingsW);
    bb(EnumPropsA);
    bb(EnumPropsExA);
    bb(EnumPropsExW);
    bb(EnumPropsW);
    bb(EnumThreadWindows);
    bb(EnumWindowStationsA);
    bb(EnumWindowStationsW);
    bb(EnumWindows);
    bb(FillRect);
    bb(FindWindowA );
    bb(FindWindowExA);
    bb(FindWindowExW);
    bb(FindWindowW);
    bb(FlashWindowEx);
    bb(GetActiveWindow);
    bb(GetAncestor);
    bb(GetAsyncKeyState);
    bb(GetCaretBlinkTime);
    bb(GetCaretPos);
    bb(GetClassInfoExW);
    bb(GetClassInfoW);
    bb(GetClassLongA);
    bb(GetClassLongW);
    bb(GetClassWord);
    bb(GetClientRect);
    bb(GetClipCursor);
    bb(GetClipboardData);
    bb(GetClipboardFormatNameA);
    bb(GetClipboardFormatNameW);
    bb(GetClipboardOwner);
    bb(GetClipboardSequenceNumber);
    bb(GetClipboardViewer);
    bb(GetCursor);
    bb(GetCursorPos);
    bb(GetDC);
    bb(GetDesktopWindow);
    bb(GetDoubleClickTime);
    bb(GetFocus);
    bb(GetForegroundWindow);
    bb(GetIconInfo);
    bb(GetKBCodePage);
    bb(GetKeyNameTextA);
    bb(GetKeyNameTextW);
    bb(GetKeyState);
    bb(GetKeyboardLayout);
    bb(GetKeyboardLayoutList);
    bb(GetKeyboardLayoutNameA);
    bb(GetKeyboardLayoutNameW);
    bb(GetKeyboardState);
    bb(GetKeyboardType);
    bb(GetMessageW);
    bb(GetOpenClipboardWindow);
    bb(GetParent);
    bb(GetQueueStatus);
    // bb(GetPhysicalCursorPos);
    bb(GetScrollPos);
    bb(GetScrollRange);
    bb(GetShellWindow);
    bb(GetSysColor);
    bb(GetSysColorBrush);
    bb(GetSystemMetrics);
    bb(GetThreadDesktop);
    // bb(GetUpdatedClipboardFormats);
    bb(GetWindow);
    bb(GetWindowLongA);
    bb(GetWindowLongW);
    bb(GetWindowModuleFileNameA);
    bb(GetWindowModuleFileNameW);
    bb(GetWindowPlacement);
    bb(GetWindowRect);
    bb(GetWindowRgn);
    //bb(GetWindowRgnBox);
    bb(GetWindowTextA);
    bb(GetWindowTextLengthA);
    bb(GetWindowTextLengthW);
    bb(GetWindowTextW);
    bb(GetWindowThreadProcessId);
    bb(GrayStringA);
    bb(GrayStringW);
    bb(HideCaret);
    bb(InflateRect);
    bb(InsertMenuA);
    bb(InsertMenuW);
    bb(InsertMenuItemA);
    bb(InsertMenuItemW);
    bb(InternalGetWindowText);
    bb(IntersectRect);
    bb(InvalidateRect);
    bb(InvalidateRgn);
    bb(IsCharAlphaA);
    bb(IsCharAlphaNumericA);
    bb(IsCharAlphaNumericW);
    bb(IsCharAlphaW);
    bb(IsCharLowerA);
    bb(IsCharLowerW);
    bb(IsCharUpperA);
    bb(IsCharUpperW);
    bb(IsChild);
    bb(IsClipboardFormatAvailable);
    bb(IsDialogMessageA);
    bb(IsDialogMessageW);
    bb(IsDlgButtonChecked);
    bb(IsGUIThread);
    bb(IsHungAppWindow);
    bb(IsIconic);
    bb(IsMenu);
    bb(IsWindow);
    bb(IsWindowEnabled);
    bb(IsWindowUnicode);
    bb(IsWindowVisible);
    //bb(IsWow64Message);
    bb(IsZoomed);
    bb(KillTimer);
    bb(LoadAcceleratorsA);
    bb(LoadAcceleratorsW);
    bb(LoadBitmapA);
    bb(LoadBitmapW);
    bb(LoadCursorA);
    bb(LoadCursorFromFileA);
    bb(LoadCursorFromFileW);
    bb(LoadCursorW);
    bb(LoadIconA);
    bb(LoadIconW);
    bb(LoadImageA);
    bb(LoadImageW);
    bb(LoadKeyboardLayoutA);
    bb(LoadKeyboardLayoutW);
    bb(LoadMenuA);
    bb(LoadMenuIndirectA);
    bb(LoadMenuIndirectW);
    bb(LoadMenuW);
    bb(LoadStringA);
    bb(LoadStringW);
    bb(LockSetForegroundWindow);
    bb(LockWindowUpdate);
    bb(LockWorkStation);
    bb(LookupIconIdFromDirectory);
    bb(LookupIconIdFromDirectoryEx);
    bb(MapDialogRect);
    bb(MapVirtualKeyA);
    bb(MapVirtualKeyExA);
    bb(MapVirtualKeyExW);
    bb(MapVirtualKeyW);
    bb(MapWindowPoints);
    bb(MenuItemFromPoint);
    bb(MessageBeep);
    bb(MessageBoxA);
    bb(MessageBoxExA);
    bb(MessageBoxExW);
    bb(MessageBoxIndirectA);
    bb(MessageBoxIndirectW);
    bb(MessageBoxW);
    bb(ModifyMenuA);
    bb(ModifyMenuW);
    bb(MsgWaitForMultipleObjects);
    bb(MsgWaitForMultipleObjectsEx);
    bb(NotifyWinEvent);
    bb(OemKeyScan);
    bb(OemToCharA);
    bb(OemToCharBuffA);
    bb(OemToCharBuffW);
    bb(OemToCharW);
    bb(OffsetRect);
    bb(OpenClipboard);
    bb(OpenDesktopA);
    bb(OpenDesktopW);
    bb(OpenIcon);
    bb(OpenInputDesktop);
    bb(OpenWindowStationA);
    bb(OpenWindowStationW);
    bb(PackDDElParam);
    bb(PaintDesktop);
    bb(PeekMessageA);
    bb(PeekMessageW);
    bb(PostMessageA);
    bb(PostMessageW);
    bb(PostQuitMessage);
    bb(RegisterClassExW);
    bb(RegisterClipboardFormatA);
    bb(RegisterClipboardFormatW);
    bb(RegisterDeviceNotificationA);
    bb(RegisterDeviceNotificationW);
    bb(RegisterHotKey);
    bb(RegisterRawInputDevices);
    bb(RegisterWindowMessageA);
    bb(RegisterWindowMessageW);
    bb(ReleaseCapture);
    bb(ReleaseDC);
    bb(RemovePropA);
    bb(RemovePropW);
    bb(ScreenToClient);
    bb(ScrollDC);
    bb(ScrollWindow);
    bb(ScrollWindowEx);
    bb(SendDlgItemMessageA);
    bb(SendDlgItemMessageW);
    bb(SendInput);
    bb(SendMessageA);
    bb(SendMessageTimeoutA);
    bb(SendMessageTimeoutW);
    bb(SendMessageW);
    bb(SendNotifyMessageA);
    bb(SendNotifyMessageW);
    bb(SetActiveWindow);
    bb(SetCaretBlinkTime);
    bb(SetCaretPos);
    bb(SetClassLongA);
    bb(SetClassLongW);
    bb(SetClassWord);
    bb(SetClipboardViewer);
    bb(SetCursor);
    bb(SetCursorPos);
    bb(SetDlgItemInt);
    bb(SetDlgItemTextA);
    bb(SetDlgItemTextW);
    bb(SetDoubleClickTime);
    bb(SetFocus);
    bb(SetForegroundWindow);
    bb(SetKeyboardState);
    bb(SetLastErrorEx);
    bb(SetMenu);
    bb(SetParent);
    // bb(SetPhysicalCursorPos);
    bb(SetPropA);
    bb(SetPropW);
    bb(SetRect);
    bb(SetRectEmpty);
    bb(SetScrollPos);
    bb(SetScrollRange);
    bb(SetSysColors);
    bb(SetSystemCursor);
    bb(SetThreadDesktop);
    bb(SetTimer);
    bb(SetWinEventHook);
    bb(SetWindowLongA);
    bb(SetWindowLongW);
    bb(SetWindowPos);
    bb(SetWindowRgn);
    bb(SetWindowTextA);
    bb(SetWindowTextW);
    bb(SetWindowsHookExA);
    bb(SetWindowsHookExW);
    bb(ShowCaret);
    bb(ShowCursor);
    bb(ShowWindow);
    bb(ShowWindowAsync);
    bb(SwapMouseButton);
    bb(SwitchDesktop);
    bb(SystemParametersInfoA);
    bb(SystemParametersInfoW);
    bb(TabbedTextOutA);
    bb(TabbedTextOutW);
    bb(TrackMouseEvent);
    bb(TranslateMessage);
    bb(UnhookWinEvent);
    bb(UnhookWindowsHookEx);
    bb(UnionRect);
    bb(UnloadKeyboardLayout);
    bb(UnpackDDElParam);
    bb(UnregisterClassA);
    bb(UnregisterClassW);
    bb(UnregisterDeviceNotification);
    bb(UnregisterHotKey);
    bb(UpdateLayeredWindow);
    bb(UpdateWindow);
    bb(UserHandleGrantAccess);
    bb(ValidateRect);
    bb(ValidateRgn);
    bb(VkKeyScanA);
    bb(VkKeyScanExA);
    bb(VkKeyScanExW);
    bb(VkKeyScanW);
    bb(WaitForInputIdle);
    bb(WaitMessage);
    bb(WinHelpA);
    bb(WinHelpW);
    bb(WindowFromDC);
    bb(WindowFromPoint);
    bb(keybd_event);
    bb(mouse_event);
    bb(GetScrollInfo);
    bb(SetScrollInfo);
}
