use std::path::PathBuf;
use windows::core::*;
use windows::Win32::Foundation::*;
use windows::Win32::System::Com::*;
use windows::Win32::UI::Shell::*;
use windows::Win32::UI::WindowsAndMessaging::*;
use windows::Win32::*;

pub fn select_folder() -> Option<PathBuf> {
    unsafe {
        // Initialize COM library
        CoInitializeEx(None, COINIT_APARTMENTTHREADED).unwrap();

        // Create FileOpenDialog instance
        let file_dialog: IFileOpenDialog =
            CoCreateInstance(&FileOpenDialog, None, CLSCTX_ALL).unwrap();

        // Configure dialog options
        let options = file_dialog.GetOptions().unwrap();
        file_dialog
            .SetOptions(options | FOS_PICKFOLDERS | FOS_FORCEFILESYSTEM)
            .unwrap();

        // Set the dialog title
        file_dialog.SetTitle(w!("Select a folder")).unwrap();

        // Show the dialog
        match file_dialog.Show(None) {
            Ok(_) => {
                // Get the selected item
                let result = file_dialog.GetResult().unwrap();
                let path = result.GetDisplayName(SIGDN_FILESYSPATH).unwrap();

                // Convert wide string path to PathBuf
                let path_str = path.to_string().unwrap();
                Some(PathBuf::from(path_str))
            }
            Err(_) => None, // User cancelled or error occurred
        }
    }
}
