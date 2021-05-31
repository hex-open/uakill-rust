extern crate embed_resource;
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
    embed_resource::compile("uakiller-manifest.rc");
}
