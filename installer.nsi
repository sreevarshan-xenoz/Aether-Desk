; Aether-Desk Installer Script
; NSIS (Nullsoft Scriptable Install System) script for creating a Windows installer

; Include modern UI
!include "MUI2.nsh"

; Define application information
!define APPNAME "Aether-Desk"
!define COMPANYNAME "SreeVarshan"
!define DESCRIPTION "A modern wallpaper engine for Windows and Linux"
!define VERSIONMAJOR 0
!define VERSIONMINOR 1
!define VERSIONBUILD 0

; Define installer file name
OutFile "Aether-Desk-Setup.exe"

; Default installation folder
InstallDir "$PROGRAMFILES\${APPNAME}"

; Request application privileges
RequestExecutionLevel admin

; Interface Settings
!define MUI_ABORTWARNING
!define MUI_ICON "assets\icon.ico"
!define MUI_UNICON "assets\icon.ico"

; Pages
!insertmacro MUI_PAGE_WELCOME
!insertmacro MUI_PAGE_LICENSE "LICENSE"
!insertmacro MUI_PAGE_DIRECTORY
!insertmacro MUI_PAGE_INSTFILES
!insertmacro MUI_PAGE_FINISH

; Uninstaller pages
!insertmacro MUI_UNPAGE_CONFIRM
!insertmacro MUI_UNPAGE_INSTFILES

; Language files
!insertmacro MUI_LANGUAGE "English"

; Installer sections
Section "Install"
  SetOutPath "$INSTDIR"
  
  ; Add files
  File "dist\windows\aether-desk.exe"
  File "dist\windows\README.md"
  File "dist\windows\LICENSE"
  
  ; Create directories
  CreateDirectory "$INSTDIR\config"
  CreateDirectory "$INSTDIR\plugins"
  
  ; Create uninstaller
  WriteUninstaller "$INSTDIR\Uninstall.exe"
  
  ; Create start menu shortcuts
  CreateDirectory "$SMPROGRAMS\${APPNAME}"
  CreateShortcut "$SMPROGRAMS\${APPNAME}\${APPNAME}.lnk" "$INSTDIR\aether-desk.exe"
  CreateShortcut "$SMPROGRAMS\${APPNAME}\Uninstall.lnk" "$INSTDIR\Uninstall.exe"
  
  ; Create desktop shortcut
  CreateShortcut "$DESKTOP\${APPNAME}.lnk" "$INSTDIR\aether-desk.exe"
  
  ; Write registry information
  WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${APPNAME}" "DisplayName" "${APPNAME}"
  WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${APPNAME}" "UninstallString" "$\"$INSTDIR\Uninstall.exe$\""
  WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${APPNAME}" "QuietUninstallString" "$\"$INSTDIR\Uninstall.exe$\" /S"
  WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${APPNAME}" "InstallLocation" "$INSTDIR"
  WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${APPNAME}" "DisplayIcon" "$INSTDIR\aether-desk.exe"
  WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${APPNAME}" "Publisher" "${COMPANYNAME}"
  WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${APPNAME}" "DisplayVersion" "${VERSIONMAJOR}.${VERSIONMINOR}.${VERSIONBUILD}"
  WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${APPNAME}" "URLInfoAbout" "https://github.com/sreevarshan-xenoz/aether-desk"
  WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${APPNAME}" "HelpLink" "https://github.com/sreevarshan-xenoz/aether-desk/issues"
  WriteRegDWORD HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${APPNAME}" "VersionMajor" ${VERSIONMAJOR}
  WriteRegDWORD HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${APPNAME}" "VersionMinor" ${VERSIONMINOR}
  WriteRegDWORD HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${APPNAME}" "NoModify" 1
  WriteRegDWORD HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${APPNAME}" "NoRepair" 1
SectionEnd

; Uninstaller section
Section "Uninstall"
  ; Remove files
  Delete "$INSTDIR\aether-desk.exe"
  Delete "$INSTDIR\README.md"
  Delete "$INSTDIR\LICENSE"
  Delete "$INSTDIR\Uninstall.exe"
  
  ; Remove directories
  RMDir /r "$INSTDIR\config"
  RMDir /r "$INSTDIR\plugins"
  RMDir "$INSTDIR"
  
  ; Remove shortcuts
  Delete "$SMPROGRAMS\${APPNAME}\${APPNAME}.lnk"
  Delete "$SMPROGRAMS\${APPNAME}\Uninstall.lnk"
  RMDir "$SMPROGRAMS\${APPNAME}"
  Delete "$DESKTOP\${APPNAME}.lnk"
  
  ; Remove registry information
  DeleteRegKey HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${APPNAME}"
SectionEnd 