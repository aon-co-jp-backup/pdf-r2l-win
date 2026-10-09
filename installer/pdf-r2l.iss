[Setup]
AppName=PDF R2L
AppVersion=1.0.0
AppPublisher=aon-co-jp
DefaultDirName={autopf}\PDF R2L
DefaultGroupName=PDF R2L
OutputDir=..\dist
OutputBaseFilename=pdf-r2l-setup
Compression=lzma2
SolidCompression=yes
ArchitecturesInstallIn64BitMode=x64compatible
LicenseFile=..\LICENSE
UninstallDisplayIcon={app}\pdf-r2l.exe

[Languages]
Name: "japanese"; MessagesFile: "compiler:Languages\Japanese.isl"

[Tasks]
Name: "desktopicon"; Description: "デスクトップにアイコンを作成"

[Files]
Source: "..\target\release\pdf-r2l.exe"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\LICENSE"; DestDir: "{app}"; Flags: ignoreversion

[Icons]
Name: "{group}\PDF R2L"; Filename: "{app}\pdf-r2l.exe"
Name: "{autodesktop}\PDF R2L"; Filename: "{app}\pdf-r2l.exe"; Tasks: desktopicon

[Run]
Filename: "{app}\pdf-r2l.exe"; Description: "PDF R2Lを起動"; Flags: nowait postinstall skipifsilent
