use hklib::mem::process::Process;

const DLL_PATH: &str = r"C:\Users\user\dev\learning\memhack\testhacks\target\i686-pc-windows-msvc\debug\testhacks.dll";

fn main() {

    let process = match unsafe { Process::find_by_name("ac_client.exe") } {
        Some(process) => {
            match unsafe { process.eject_dll(DLL_PATH) } {
                Ok(_) => println!("DLL ejected successfully"),
                Err(e) => println!("DLL not found: {}", e),
            }
            Some(process)
        },
        None => {
            println!("Process not found");
            None
        }
    };

    std::process::Command::new("cargo")
        .args(["build", "--target", "i686-pc-windows-msvc"])
        .current_dir(".")
        .status()
        .expect("failed to build testhacks");

    if let Some(process) = process {
        match unsafe { process.inject_dll(DLL_PATH) } {
            Ok(_thread_id) => println!("DLL injected successfully"),
            Err(e) => println!("Failed to inject DLL: {}", e),
        }
    }
}