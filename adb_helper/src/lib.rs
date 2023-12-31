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

    // 自动发现连接的设备
    pub fn init(mut self) -> ADB {
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

    pub fn start_app(&self, device: &Device, package: &str, activity: &str) {
        println!("启动游戏:{}", package);
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
        ADB::command_base().arg("start-server");
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
    pub fn tcpip(port: u16) {}
}

pub fn search_port(port: u16) -> Vec<String> {
    let stdout = String::from_utf8(
        Command::new("lsof")
            .arg(format!("-i:{port}"))
            .output()
            .expect("lsof 报错啦")
            .stdout,
    )
    .unwrap();

    let mut lines = stdout
        .split_terminator('\n')
        .map(|line| line.to_string())
        .collect::<Vec<String>>();
    match lines.is_empty() {
        true => lines,
        false => lines.drain(1..).collect(),
    }
}
