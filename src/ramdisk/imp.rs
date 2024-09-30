use super::{structs::VolAttach, RamDisk};
use log::{debug, info, log_enabled};

use anyhow::Result;
use std::{process::Command, str};

use super::structs::{HdiUtilOutput, RdiskCreate};

impl RamDisk {
    pub fn initialize(&self) -> Result<()> {
        let dsize = self.size * 2048;

        info!(
            "Creating temp vol named: {} with size: {}",
            self.name, dsize
        );

        // Create the base ramdisk

        let create_rdisk = Command::new("hdiutil")
            .arg("attach")
            .arg("-nomount")
            .arg("-plist")
            .arg(format!("ram://{dsize}"))
            .output()?;

        if log_enabled!(log::Level::Debug) {
            let sout = str::from_utf8(&create_rdisk.stdout).expect("Should be parsable");
            debug!("Create disk returned: {}", sout);
        }

        let create_result: HdiUtilOutput<RdiskCreate> = plist::from_bytes(&create_rdisk.stdout)?;

        let entry = &create_result.system_entities[0].dev_entry;
        // Erase it

        info!("Erasing disk: {}", entry);

        let erase_disk = Command::new("diskutil")
            .arg("eraseDisk")
            .arg("HFS+")
            .arg("%noformat%")
            .arg(entry)
            .output()?;

        if log_enabled!(log::Level::Debug) {
            let sout = str::from_utf8(&erase_disk.stdout).expect("Should be parsable");
            debug!("Erase disk returned: {}", sout);
        }

        // Format it
        let format_drive = Command::new("newfs_hfs")
            .arg("-v")
            .arg(self.name.clone())
            .arg(format!("{entry}s1"))
            .output()?;

        if log_enabled!(log::Level::Debug) {
            let sout = str::from_utf8(&format_drive.stdout).expect("Should be parsable");
            debug!("Format disk returned: {}", sout);
        }

        // Attach it with nomount
        let attach_disk = Command::new("hdiutil")
            .arg("attach")
            .arg("-nomount")
            .arg("-plist")
            .arg(entry)
            .output()?;

        if log_enabled!(log::Level::Debug) {
            let sout = str::from_utf8(&attach_disk.stdout).expect("Should be parsable");
            debug!("Attach disk returned: {}", sout);
        }

        let attach_result: HdiUtilOutput<VolAttach> = plist::from_bytes(&attach_disk.stdout)?;

        let disk_vol = &attach_result
            .system_entities
            .into_iter()
            .find(|elem| elem.volume_kind.is_some())
            .expect("Should have sub-disk")
            .dev_entry;

        info!("Attaching disk {} with -nobrowse", disk_vol);

        // Attach vol with nobrowse
        let attach_vol = Command::new("hdiutil")
            .arg("attach")
            .arg("-nobrowse")
            .arg("-plist")
            .arg(disk_vol)
            .output()?;

        if log_enabled!(log::Level::Debug) {
            let sout = str::from_utf8(&attach_vol.stdout).expect("Should be parsable");
            debug!("Attach disk returned: {}", sout);
        }

        let vol_attach_result: HdiUtilOutput<VolAttach> = plist::from_bytes(&attach_vol.stdout)?;

        let vol = &vol_attach_result
            .system_entities
            .into_iter()
            .find(|elem| elem.mount_point.is_some())
            .expect("Should have volume");

        let mt = vol.mount_point.as_ref().expect("Should have mount point");

        info!("Mounted: {} to {}", vol.dev_entry, mt.display());

        Ok(())
    }
}
