/// wasm binary toolkits

use crate::child;
use crate::emoji;
use crate::target;
use crate::PBAR;
use binary_install::Cache;
use log::debug;
use std::path::{Path, PathBuf};
use std::process::Command;

/// wasm toolkits from upstream WebAssembly/binaryen.
pub enum Toolkit {
    ///  Loads WebAssembly and runs Binaryen IR passes on it
    WasmOpt,
    /// Un-assembles WebAssembly in binary format into text format
    WasmDis,
}

/// Possible results of `find_tool`
enum FindExec {
    /// Couldn't install tool because downloads are forbidden
    CannotInstall,
    /// The current platform doesn't support precompiled binaries
    PlatformNotSupported,
    /// We found `wasm-opt` at the specified path
    Found(PathBuf),
}

impl Toolkit {
    pub fn run(
        cache: &Cache,
        out_dir: &Path,
        args: &[String],
        install_permitted: bool,
    ) -> Result<(), failure::Error> {
        let name = self.name();
        let wasm_opt = match self.find_exec(cache, install_permitted)? {
            FindExec::Found(path) => path,
            FindExec::CannotInstall => {
                PBAR.info("Skipping {} as no downloading was requested", name);
                return Ok(());
            }
            FindExec::PlatformNotSupported => {
                PBAR.info("Skipping wasm-opt because it is not supported on this platform", name);
                return Ok(());
            }
        };

        PBAR.info("Optimizing wasm binaries with `{}`...", name);

        for file in out_dir.read_dir()? {
            let file = file?;
            let path = file.path();
            if path.extension().and_then(|s| s.to_str()) != Some("wasm") {
                continue;
            }

            let tmp = path.with_extension("wasm-opt.wasm");
            let mut cmd = Command::new(&wasm_opt);
            cmd.arg(&path).arg("-o").arg(&tmp).args(args);
            child::run(cmd, "wasm-opt")?;
            std::fs::rename(&tmp, &path)?;
        }

        Ok(())
    }

    /// Attempts to find `wasm-opt` in `PATH` locally, or failing that downloads a
    /// precompiled binary.
    ///
    /// Returns `Some` if a binary was found or it was successfully downloaded.
    /// Returns `None` if a binary wasn't found in `PATH` and this platform doesn't
    /// have precompiled binaries. Returns an error if we failed to download the
    /// binary.
    fn find_exec(
        &self,
        cache: &Cache,
        install_permitted: bool,
    ) -> Result<FindTool, failure::Error> {
        let tool = self.as_str();
        // First attempt to look up in PATH. If found assume it works.
        if let Ok(path) = which::which(tool) {
            debug!("found {} at {:?}", tool, path);
            return Ok(FindTool::Found(path));
        }

        // ... and if that fails download a precompiled version.
        let target = if target::LINUX && target::x86_64 {
            "x86_64-linux"
        } else if target::MACOS && target::x86_64 {
            "x86_64-apple-darwin"
        } else if target::WINDOWS && target::x86_64 {
            "x86_64-windows"
        } else {
            return Ok(FindTool::PlatformNotSupported);
        };
        let url = format!(
            "https://github.com/WebAssembly/binaryen/releases/download/{vers}/binaryen-{vers}-{target}.tar.gz",
            vers = "version_78",
            target = target,
        );

        let download =
            |permit_install| cache.download(permit_install, tool, &[tool], &url);

        let dl = match download(false)? {
            Some(dl) => dl,
            None if !install_permitted => return Ok(FindTool::CannotInstall),
            None => {
                let msg = format!("{}Installing {}...", tool, emoji::DOWN_ARROW);
                PBAR.info(&msg);

                match download(install_permitted)? {
                    Some(dl) => dl,
                    None => return Ok(FindTool::CannotInstall),
                }
            }
        };

        Ok(FindTool::Found(dl.binary(tool)?))
    }

    fn name(&self) -> &str {
        use Self::*;
        match self {
            WasmOpt => "wasm-opt",
            WasmDis => "wasm-dis",
        }
    }
}
