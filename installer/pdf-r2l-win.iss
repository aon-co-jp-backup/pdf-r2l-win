[Setup]
AppName=PDF R2L Win
AppVersion=1.0.0
AppPublisher=aon-co-jp
DefaultDirName={autopf}\PDF R2L Win
DefaultGroupName=PDF R2L Win
OutputDir=..\dist
OutputBaseFilename=pdf-r2l-win-setup
Compression=lzma2
SolidCompression=yes
ArchitecturesInstallIn64BitMode=x64compatible
LicenseFile=..\LICENSES.txt
UninstallDisplayIcon={app}\pdf-r2l-win.exe

[Languages]
Name: "japanese"; MessagesFile: "compiler:Languages\Japanese.isl"
Name: "english"; MessagesFile: "compiler:Default.isl"

[Tasks]
Name: "desktopicon"; Description: "Create a desktop icon / デスクトップにアイコンを作成"

[Files]
Source: "..\target\release\pdf-r2l-win.exe"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\LICENSES.txt"; DestDir: "{app}"; Flags: ignoreversion

[Icons]
Name: "{group}\PDF R2L Win"; Filename: "{app}\pdf-r2l-win.exe"
Name: "{autodesktop}\PDF R2L Win"; Filename: "{app}\pdf-r2l-win.exe"; Tasks: desktopicon

[Run]
Filename: "{app}\pdf-r2l-win.exe"; Description: "Launch PDF R2L Win / 起動"; Flags: nowait postinstall skipifsilent
