use std::io::Write;
use std::path::Path;
use std::time::Instant;
use git2::{Cred, CredentialType, Progress, RemoteCallbacks};

use crate::enums::auth_method::AuthMethod;
use crate::info;

pub fn handle_credentials(method: AuthMethod, rc: &mut RemoteCallbacks) {
    rc.credentials(move |_, username, allowed| {
        let username = username.unwrap_or("git");
                
        if method == AuthMethod::NONE || !allowed.contains(method.clone().into()) {
            let mut expected_creds = String::new(); 
            if !allowed.is_empty() { expected_creds += "Expects:\n"; }
            if allowed.contains(CredentialType::SSH_MEMORY) { expected_creds += "- SSH Agent\n"; }
            if allowed.contains(CredentialType::SSH_KEY) { expected_creds += "- SSH Key\n"; }
            if allowed.contains(CredentialType::USER_PASS_PLAINTEXT) { expected_creds += "- Auth Token\n"}
            eprintln!("{expected_creds}");
        }

        match &method {
            AuthMethod::SSH_AGENT => Cred::ssh_key_from_agent(username),
            AuthMethod::SSH_KEY(key) => Cred::ssh_key(username, None, &Path::new(&key), None),
            AuthMethod::AUTH_TOKEN(token) => Cred::userpass_plaintext(username, &token),
            AuthMethod::NONE => Cred::username(username),
        }
    });
}

pub fn handle_sideband_progress(rc: &mut RemoteCallbacks) {
    rc.sideband_progress(|bytes| {
        if let Ok(str) = str::from_utf8(bytes) {
            info!("{str}");
            let _ = std::io::stdout().flush();
        }
        true
    });
}

pub fn handle_transfer_progress(rc: &mut RemoteCallbacks) {
    let start = Instant::now();
    let mut all_objs_received = false;

    rc.transfer_progress(move |progress| {
        if !all_objs_received {
            handle_object_progress(start.elapsed().as_secs_f32(), &progress);
            all_objs_received = progress.received_objects() == progress.total_objects();
        } else {
            handle_delta_resolution(&progress);
        }

        true
    });
}

fn handle_object_progress(elapsed: f32, progress: &Progress<'_>) {
    let cur_objs = progress.received_objects();
    let max_objs = progress.total_objects(); 
    let bytes = progress.received_bytes();
    let percent = cur_objs as f32 / max_objs as f32 * 100.0;
    let all_objs = cur_objs == max_objs;
    let mib = bytes as f32 / (1 << 20) as f32;
    let mibps = mib / elapsed;
    
    let msg = format!("{}{} {:>3.0}% ({}/{}){}{}",
        "\r\x1b[2K",
        "Recieving objects:",
        percent,
        cur_objs,
        max_objs,
        if bytes != 0 { format!(", {:.2} MiB | {:.2} MiB/s", mib, mibps) } else { String::new() },
        if all_objs { ", done.\n" } else { "\r" }
    );

    info!("{msg}");
}

fn handle_delta_resolution(progress: &Progress<'_>) {
    let total = progress.total_deltas();
    let indexed = progress.indexed_deltas();
    let percent = indexed as f32 / total as f32 * 100.0;

    if total == 0 { return }

    info!(
        "{}{} {:>3.0}% ({}/{}){}",
        "\r\x1b[2K",
        "Resolving deltas:",
        percent,
        indexed,
        total,
        if indexed == total { ", done.\n" } else { "\r" }
    );
}



