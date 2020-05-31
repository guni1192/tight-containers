use std::collections::HashMap;
use std::path::PathBuf;

use serde_derive::{Deserialize, Serialize};

pub static OCI_VERSION: &str = "1.0.1-dev";

#[derive(Serialize, Deserialize, Copy, Clone, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub enum Status {
    #[serde(rename = "creating")]
    Creating,
    #[serde(rename = "created")]
    Created,
    #[serde(rename = "running")]
    Running,
    #[serde(rename = "stopped")]
    Stopped,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct State {
    pub oci_version: String,
    pub id: String,
    pub status: Status,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pid: Option<i32>,
    pub bundle: PathBuf,
    pub rootfs: PathBuf,
    pub owner: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotation: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Spec {
    pub oci_version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process: Option<Process>,
    pub root: Root,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    pub mounts: Vec<Mount>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hooks: Option<Hooks>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotations: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linux: Option<Linux>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solaris: Option<Solaris>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub windows: Option<Windows>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vm: Option<VM>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Process {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terminal: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub console_size: Option<Box>,
    pub user: User,
    pub args: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command_line: Option<String>,
    pub cwd: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<LinuxCapabilities>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rlimits: Option<Vec<POSIXRlimit>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_new_privileges: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apparmor_profile: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oom_score_adj: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selinux_label: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LinuxCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bounding: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inheritable: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permitted: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ambient: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Box {
    height: u64,
    width: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readonly: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Mount {
    pub destination: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub source: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Hook {
    pub path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<u32>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Hooks {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prestart: Option<Vec<Hook>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_runtime: Option<Vec<Hook>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_container: Option<Vec<Hook>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_container: Option<Vec<Hook>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub poststart: Option<Vec<Hook>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub poststop: Option<Vec<Hook>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Linux {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid_mappings: Option<Vec<LinuxIDMapping>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gid_mappings: Option<Vec<LinuxIDMapping>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sysctl: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<LinuxResources>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cgroups_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespaces: Option<Vec<LinuxNamespace>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub devices: Option<Vec<LinuxDevice>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seccomp: Option<LinuxSeccomp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rootfs_propagation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub masked_paths: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readonly_paths: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mount_label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intel_rdt: Option<LinuxIntelRdt>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LinuxIDMapping {
    #[serde(rename = "containerID")]
    pub container_id: u32,
    #[serde(rename = "hostID")]
    pub host_id: u32,
    pub size: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LinuxResources {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub devices: Option<Vec<LinuxDeviceCgroup>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<LinuxMemory>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu: Option<LinuxCPU>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pids: Option<LinuxPids>,
    #[serde(rename = "blockIO", skip_serializing_if = "Option::is_none")]
    pub block_io: Option<LinuxBlockIO>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hugepage_limits: Option<Vec<LinuxHugepageLimit>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<LinuxNetwork>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rdma: Option<HashMap<String, LinuxRdma>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LinuxDevice {
    pub path: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub major: i64,
    pub minor: i64,
    // TODO: FileMode
    // Golang FileMode *os.FileMode
    // -> Rust file_mode Option<T>
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gid: Option<u32>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LinuxDeviceCgroup {
    pub allow: bool,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub major: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minor: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LinuxSeccomp {
    pub default_action: LinuxSeccompAction,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub architectures: Option<Vec<Arch>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub syscalls: Option<Vec<LinuxSyscall>>,
}

pub type Arch = String;

// TODO: define Architecures as const

pub type LinuxSeccompAction = String;

// TODO: define seccomp actions as const

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LinuxSyscall {
    pub names: Vec<String>,
    pub action: LinuxSeccompAction,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<LinuxSeccompArg>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LinuxSeccompArg {
    pub index: u64,
    pub value: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_two: Option<u64>,
    pub op: LinuxSeccompOperator,
}

pub type LinuxSeccompOperator = String;

// TODO: define seccomp operators as const

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LinuxMemory {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reservation: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kernel: Option<i64>,
    #[serde(rename = "kernelTCP", skip_serializing_if = "Option::is_none")]
    pub kernel_tcp: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swappiness: Option<u64>,
    #[serde(rename = "disableOOMKiller", skip_serializing_if = "Option::is_none")]
    pub disable_oom_killer: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LinuxCPU {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shares: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quota: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub realtime_runtime: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub realtime_period: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpus: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mems: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LinuxPids {
    limit: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LinuxBlockIO {
    #[serde(skip_serializing_if = "Option::is_none")]
    weight: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    leaf_weight: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    weight_device: Option<Vec<LinuxWeightDevice>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    throttle_read_bps_device: Option<Vec<LinuxThrottleDevice>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    throttle_write_bps_device: Option<Vec<LinuxThrottleDevice>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    throttle_read_iops_device: Option<Vec<LinuxThrottleDevice>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    throttle_write_iops_device: Option<Vec<LinuxThrottleDevice>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LinuxWeightDevice {
    major: i64,
    minor: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    weight: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    leaf_weight: Option<u16>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LinuxThrottleDevice {
    major: i64,
    minor: i64,
    rate: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LinuxNetwork {
    #[serde(rename = "classID", skip_serializing_if = "Option::is_none")]
    class_id: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    priorities: Option<Vec<LinuxInterfacePriority>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LinuxInterfacePriority {
    name: String,
    priority: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LinuxRdma {
    #[serde(skip_serializing_if = "Option::is_none")]
    hca_handles: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hca_objects: Option<u32>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LinuxIntelRdt {
    #[serde(rename = "closID", skip_serializing_if = "Option::is_none")]
    clos_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    l3_cache_schema: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mem_bw_schema: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub uid: u32,
    pub gid: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_gids: Option<Vec<u32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct POSIXRlimit {
    #[serde(rename = "type")]
    pub type_: String,
    pub hard: u64,
    pub soft: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LinuxHugepageLimit {
    page_size: String,
    limit: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LinuxNamespace {
    #[serde(rename = "type")]
    pub type_: LinuxNamespaceType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub enum LinuxNamespaceType {
    Mount = 0x0002_0000,
    Cgroup = 0x0200_0000,
    Uts = 0x0400_0000,
    Ipc = 0x0800_0000,
    User = 0x1000_0000,
    Pid = 0x2000_0000,
    Network = 0x4000_0000,
}

// TODO
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Solaris {}

// TODO
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Windows {}

// TODO
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct VM {}

#[macro_export]
macro_rules! vec_str_convert {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push(String::from($x));
            )*
            temp_vec
        }
    };
}

impl Default for Spec {
    fn default() -> Self {
        Self::new()
    }
}

impl Spec {
    pub fn new() -> Self {
        Spec {
            oci_version: OCI_VERSION.into(),
            root: Root {
                path: "rootfs".into(),
                readonly: Some(true),
            },
            process: Some(Process {
                terminal: Some(true),
                console_size: None,
                user: User {
                    uid: 0,
                    gid: 0,
                    additional_gids: None,
                    username: None,
                },
                args: vec_str_convert!["sh"],
                env: Some(vec_str_convert![
                    "PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin",
                    "TERM=xterm"
                ]),
                cwd: "/".into(),
                command_line: None,
                no_new_privileges: Some(true),
                capabilities: Some(LinuxCapabilities {
                    bounding: Some(vec_str_convert![
                        "CAP_AUDIT_WRITE",
                        "CAP_KILL",
                        "CAP_NET_BIND_SERVICE"
                    ]),
                    permitted: Some(vec_str_convert![
                        "CAP_AUDIT_WRITE",
                        "CAP_KILL",
                        "CAP_NET_BIND_SERVICE"
                    ]),
                    inheritable: Some(vec_str_convert![
                        "CAP_AUDIT_WRITE",
                        "CAP_KILL",
                        "CAP_NET_BIND_SERVICE"
                    ]),
                    ambient: Some(vec_str_convert![
                        "CAP_AUDIT_WRITE",
                        "CAP_KILL",
                        "CAP_NET_BIND_SERVICE"
                    ]),
                    effective: Some(vec_str_convert![
                        "CAP_AUDIT_WRITE",
                        "CAP_KILL",
                        "CAP_NET_BIND_SERVICE"
                    ]),
                }),
                rlimits: Some(vec![POSIXRlimit {
                    type_: "RLIMIT_NOFILE".into(),
                    hard: 1024,
                    soft: 1024,
                }]),
                apparmor_profile: None,
                oom_score_adj: None,
                selinux_label: None,
            }),
            hostname: Some("runrs".into()),
            mounts: vec![
                Mount {
                    destination: "/proc".into(),
                    type_: "proc".into(),
                    source: "proc".into(),
                    options: None,
                },
                Mount {
                    destination: "/dev".into(),
                    type_: "tmpfs".into(),
                    source: "tmpfs".into(),
                    options: Some(vec_str_convert!(
                        "nosuid",
                        "strictatime",
                        "mode=755",
                        "size=65536k"
                    )),
                },
                Mount {
                    destination: "/dev/pts".into(),
                    type_: "devpts".into(),
                    source: "devpts".into(),
                    options: Some(vec_str_convert!(
                        "nosuid",
                        "noexec",
                        "newinstance",
                        "ptmxmode=0666",
                        "mode=0620",
                        "gid=5"
                    )),
                },
                Mount {
                    destination: "/dev/shm".into(),
                    type_: "tmpfs".into(),
                    source: "shm".into(),
                    options: Some(vec_str_convert![
                        "nosuid",
                        "noexec",
                        "nodev",
                        "mode=1777",
                        "size=65536k"
                    ]),
                },
                Mount {
                    destination: "/dev/mqueue".into(),
                    type_: "mqueue".into(),
                    source: "mqueue".into(),
                    options: Some(vec_str_convert!["nosuid", "noexec", "nodev", "ro"]),
                },
            ],
            linux: Some(Linux {
                masked_paths: Some(vec_str_convert![
                    "/proc/kcore",
                    "/proc/latency_stats",
                    "/proc/timer_list",
                    "/proc/timer_stats",
                    "/proc/sched_debug",
                    "/sys/firmware",
                    "/proc/scsi"
                ]),
                readonly_paths: Some(vec_str_convert![
                    "/proc/asound",
                    "/proc/bus",
                    "/proc/fs",
                    "/proc/irq",
                    "/proc/sys",
                    "/proc/sysrq-trigger"
                ]),
                resources: Some(LinuxResources {
                    devices: Some(vec![LinuxDeviceCgroup {
                        allow: false,
                        access: Some("rwm".into()),
                        type_: None,
                        major: None,
                        minor: None,
                    }]),
                    block_io: None,
                    cpu: None,
                    memory: None,
                    network: None,
                    hugepage_limits: None,
                    pids: None,
                    rdma: None,
                }),
                namespaces: Some(vec![
                    LinuxNamespace {
                        type_: LinuxNamespaceType::Ipc,
                        path: None,
                    },
                    LinuxNamespace {
                        type_: LinuxNamespaceType::Mount,
                        path: None,
                    },
                    LinuxNamespace {
                        type_: LinuxNamespaceType::Uts,
                        path: None,
                    },
                    LinuxNamespace {
                        type_: LinuxNamespaceType::Pid,
                        path: None,
                    },
                    LinuxNamespace {
                        type_: LinuxNamespaceType::Network,
                        path: None,
                    },
                ]),
                cgroups_path: None,
                devices: None,
                uid_mappings: None,
                gid_mappings: None,
                intel_rdt: None,
                mount_label: None,
                seccomp: None,
                rootfs_propagation: None,
                sysctl: None,
            }),
            hooks: None,
            annotations: None,
            solaris: None,
            windows: None,
            vm: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state() {
        let status_string = serde_json::to_string(&Status::Created).unwrap();

        assert_eq!(status_string, "\"created\"");
    }

    #[test]
    fn test_user_convert_json() {
        let user = User {
            uid: 0,
            gid: 0,
            additional_gids: Some(vec![1000, 1001, 1002]),
            username: Some("root".into()),
        };
        let json_string = serde_json::to_string(&user).unwrap();
        assert_eq!(
            r#"{"uid":0,"gid":0,"additionalGids":[1000,1001,1002],"username":"root"}"#,
            json_string
        )
    }

    #[test]
    fn test_linux_id_mapping_convert_json() {
        let user = LinuxIDMapping {
            container_id: 0,
            host_id: 1000,
            size: 1,
        };
        let json_string = serde_json::to_string(&user).unwrap();
        assert_eq!(r#"{"containerID":0,"hostID":1000,"size":1}"#, json_string)
    }
}
