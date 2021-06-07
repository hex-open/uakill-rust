extern crate embed_resource;
extern crate winres;

fn main() {
    windows::build!(
        Windows::Win32::System::Diagnostics::ToolHelp::{
            CreateToolhelp32Snapshot, Process32First, Process32Next, PROCESSENTRY32, TH32CS_SNAPPROCESS,
        },
        Windows::Win32::System::SystemServices::{CHAR, THREAD_SUSPEND_RESUME},
        Windows::Win32::System::Threading::{
            GetCurrentProcess, OpenProcess, OpenThread, SuspendThread, TerminateProcess,
            PROCESS_TERMINATE,
        },
        Windows::Win32::System::WindowsProgramming::CloseHandle,
        Windows::Win32::UI::WindowsAndMessaging::{MessageBoxA, MB_OK},
    );

    embed_resource::compile("./uakiller-manifest.rc");

    let mut res = winres::WindowsResource::new();
    res.set_manifest(
        r#"
<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<assembly xmlns="urn:schemas-microsoft-com:asm.v1" manifestVersion="1.0">
  <assemblyIdentity processorArchitecture="*" type="win32" name="uakiller" version="1.0.0.0" />
  <description>UniAgent Killer</description>
  <trustInfo xmlns="urn:schemas-microsoft-com:asm.v2">
    <security>
      <requestedPrivileges xmlns="urn:schemas-microsoft-com:asm.v3">
        <!-- <requestedExecutionLevel level="asInvoker" uiAccess="false" /> -->
        <requestedExecutionLevel level="requireAdministrator" uiAccess="false" />
      </requestedPrivileges>
    </security>
  </trustInfo>
</assembly>
"#,
    );
    res.compile().unwrap();
}
