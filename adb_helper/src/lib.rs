// use strum_macros::EnumString;
static ADB_LISTEN_PORT: u16 = 5037;

pub struct Point {
    pub x: i32,
    pub y: i32,
}
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}
use std::net::Ipv4Addr;
use std::path::PathBuf;
use std::process::Command;
use std::str::FromStr;
use strum_macros::EnumString;

struct Package {
    version: Version,
    name: &'static str
}

impl Package {
    // 获取apk包的版本号
    // pub fn get_version(package_name:&str) -> Version{
    //     let stdout = String::from_utf8(
    //         Command::new("dumpsys")
    //             .arg("package")
    //             .arg(name)
    //     )
    // }
}

#[derive(EnumString, Debug)]
pub enum DeviceStatus {
    device,
    offline,
    bootloader,
}
#[derive(Debug)]
pub struct Device {
    searial: String,
    status: DeviceStatus, // host: Ipv4Addr,
                          // port: u16,
}

impl Device {
    pub fn new(searial: &str, status: DeviceStatus) -> Self {
        Self {
            searial: searial.to_string(),
            status,
        }
    }
}

#[derive(Debug)]
pub struct ADB {
    pub devices: Vec<Device>,
}

impl ADB {
    pub fn new() -> Self {
        Self {
            devices: Vec::new(),
        }
    }

    fn esixt() -> bool {
        match ADB::command_base().output() {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    fn download_adb() {
        let url = match OSType::init() {
            OSType::Windows => {
                "https://googledownloads.cn/android/repository/platform-tools-latest-windows.zip"
            } // windows,
            OSType::Linux => {
                "https://googledownloads.cn/android/repository/platform-tools-latest-linux.zip"
            } // linux,
            OSType::Undefine => todo!(),
        };
    }
    // 自动发现连接的设备
    pub fn init(mut self) -> ADB {
        if !ADB::esixt() {
            println!("adb not exist");
            match OSType::init() {
                OSType::Windows => todo!(),
                OSType::Linux => todo!(),
                OSType::Undefine => todo!(),
            }
            ADB::download_adb();
        }
        let process_list = search_port(ADB_LISTEN_PORT);
        match process_list.is_empty() {
            true => ADB::start_server(),
            false => (), // 杀死占用的进程或者报错
        }
        self.devices.append(&mut ADB::search_devices());

        self
    }

    pub fn search_devices() -> Vec<Device> {
        let stdout = String::from_utf8(
            ADB::command_base()
                .arg("devices")
                .output()
                .expect("adb devices failed")
                .stdout,
        )
        .unwrap();
        let lines = stdout
            .split_terminator("\n")
            .collect::<Vec<&str>>()
            .drain(1..)
            .map(|line| line)
            .filter(|line| *line != "")
            .collect::<Vec<&str>>();

        let devices = match lines.is_empty() {
            true => Vec::new(),
            false => lines
                .into_iter()
                .map(|line| {
                    let device_line = line.split('\t').collect::<Vec<&str>>();
                    let searial = device_line[0];
                    let status = DeviceStatus::from_str(device_line[1]).unwrap();
                    Device::new(searial, status)
                })
                .collect::<Vec<Device>>(),
        };

        devices
    }

    fn command_base() -> Command {
        Command::new("adb")
    }

    pub fn connect(self, host: Ipv4Addr, port: u16) {
        // 尝试和设备建立连接，如果连接成功则添加到设备列表中
        // 如果连接失败，则不添加并返回连接错误的描述
        todo!();
    }

    pub fn disconnect(self, host: Ipv4Addr, port: Option<u16>) {
        todo!()
    }

    pub fn disconnect_all(self) {
        // 删除所有的设备
        ADB::command_base().arg("disconnect");
    }

    /// 获取当前显示的 activity
    pub fn get_current_activity(&self, device: &Device) {
        let stdout = String::from_utf8(
            ADB::command_base()
                .args(["-s", &device.searial])
                .arg("shell")
                .args([
                    "dumpsys", "activity", "top", "|", "grep", "ACTIVITY", "|", "tail", "-n", "1",
                ])
                .output()
                .expect("get_current_activity failed")
                .stdout,
        )
        .unwrap();
        let current_activity = stdout.trim();
        // .split_terminator("\n")
        // .collect::<Vec<&str>>();
        println!("{:#?}", current_activity);
        // current_activity
    }

    pub fn start_app(&self, device: &Device, package: &str, activity: &str) {
        println!("启动 APP:{}", package);
        ADB::command_base()
            .args(["-s", &device.searial])
            .arg("shell")
            .args(["am", "start"])
            .arg(format!("{}/{}", package, activity))
            .output()
            .expect("启动失败");
    }

    pub fn screenshot(&self, device: &Device) {
        // 截图
        todo!()
    }

    pub fn tap(&self, device: &Device, point: Point) {
        // 点击
        todo!()
    }

    pub fn start_server() {
        // 启动adb 服务
        // 判断当前系统是什么系统
        // 查看端口5037 是否被占用
        println!("启动adb中...");
        let output = match ADB::command_base().arg("start-server").output() {
            Ok(output) => output,
            Err(_) => {
                // download_adb
                todo!()
            }
        };
    }

    pub fn kill_server() {
        ADB::command_base().arg("kill-server");
    }

    pub fn push(device: Device, local: PathBuf, remote: PathBuf) {
        // 将文件传到设备上
        todo!()
    }

    pub fn pull(device: Device, remote: PathBuf, local: PathBuf) {
        // 将文件从设备拉出来
        todo!()
    }

    pub fn shell(device: Device) {
        ADB::command_base().arg("shell");
    }

    pub fn install(device: Device, package: PathBuf) {
        // 安装软件
        todo!()
    }
    pub fn uninstall(device: Device, package: PathBuf) {
        // 安装软件
        todo!()
    }

    pub fn root() {}
    pub fn unroot() {}
    pub fn usb() {}
    pub fn tcpip(port: u16) {
        ADB::command_base().arg(port.to_string());
    }
}

#[derive(Debug)]
pub enum OSType {
    Windows,
    Linux,
    Undefine,
}

impl OSType {
    pub fn init() -> Self {
        match std::env::consts::OS {
            "linux" => OSType::Linux,
            "windows" => OSType::Windows,
            _ => OSType::Undefine,
        }
    }
}

pub fn search_port(port: u16) -> Vec<String> {
    let os_type: OSType = OSType::init();
    println!("{:#?}", os_type);
    let output = match os_type {
        OSType::Linux => Command::new("lsof")
            .arg(format!("-i:{port}"))
            .output()
            .expect("lsof 报错啦"),
        OSType::Windows => Command::new("netstat")
            .arg("-ano|findstr")
            .arg(format!("\"{}\"", port))
            .output()
            .expect("netstat 报错啦"),
        _ => todo!(),
    };

    let stdout = String::from_utf8(output.stdout).unwrap();

    let mut lines = stdout
        .split_terminator('\n')
        .map(|line| line.to_string())
        .collect::<Vec<String>>();

    match lines.is_empty() {
        true => lines,
        false => lines.drain(1..).collect(),
    }
}
