use std::process::{ Command, Stdio };
// use crate::gradients;
#[repr(C)]
struct Df {
    total: i32,
    used: i32,
}
#[repr(C)]
struct Color {
    r: i32,
    g: i32,
    b: i32,
}

extern "C" {
    fn get_vol() -> f32;
    fn get_muted() -> i32;
    fn df_root() -> Df;
    fn rg_gradient(o: i32) -> Color;
    fn br_gradient(o: i32) -> Color;
}

fn color_to_string(c: Color) -> String {
    format!("#{:02X}{:02X}{:02X}", c.r, c.g, c.b)
}

pub fn parse_volume() -> String {
    let muted = (unsafe { get_muted() }) == 1;

    let mut vol_num = (unsafe { get_vol() }).round() as i32;

    let max_vol = 100;
    if vol_num > max_vol {
        Command::new("pactl")
            .arg("set-sink-volume")
            .arg("@DEFAULT_SINK@")
            .arg(max_vol.to_string() + "%")
            .stdout(Stdio::null())
            .spawn()
            .unwrap();
        vol_num = max_vol;
    }
    vol_num *= max_vol / 100;
    let vol_num_2 = vol_num;
    let mut vol_str = Vec::<String>::new();
    for i in 0..100 {
        if vol_num > 0 {
            vol_num -= max_vol / 100;
            vol_str.push(
                format!(
                    "%{{B{}}}%{{F{0}}}-%{{B-}}%{{F-}}",
                    color_to_string(unsafe { rg_gradient(i) })
                )
            );
            // if vol_num == 0 { vol_str.push("%{B-}".to_owned()); }
        } else {
            vol_str.push(format!("%{{F{}}}-%{{F-}}", color_to_string(unsafe { rg_gradient(i) })));
        }
    }
    vol_num = vol_num_2;
    let mut vol_vec = Vec::<String>::new();
    for i in (0..100).step_by(2) {
        vol_vec.push(vol_str[i].clone());
    }
    format!(
        "Volume ({}){} [{}] ",
        if !muted {
            vol_num.to_string() + "%"
        } else {
            "%{F#FF0000}MUTED%{F-}".to_owned()
        },
        if muted {
            ""
        } else {
            if vol_num >= 100 {
                " "
            } else {
                if vol_num < 10 { "   " } else { "  " }
            }
        },
        vol_vec.join("")
    )
}
pub fn parse_battery(mut bat: String, charging: bool) -> String {
    let mut chars = bat.chars();
    chars.next_back();
    bat = String::from(chars.as_str());
    let mut bat_num = match bat.parse::<i32>() {
        Ok(v) => v,
        Err(_) => 0,
    };
    let bat_num_2 = bat_num;
    let mut bat_vec = Vec::<String>::new();
    for _i in 0..20 {
        if bat_num > 5 || bat_num_2 == 100 {
            bat_vec.push("%{B#D5DE2F}%{F#D5DE2F}-%{F-}%{B-}".to_owned());
            bat_num -= 5;
        } else {
            bat_vec.push("%{F#D5DE2F}-%{F-}".to_owned());
        }
    }

    format!(
        "Battery ({}{}%{{F-}}){} [{}] ",
        if charging {
            "%{F#00FF00}"
        } else if bat_num_2 < 16 {
            "%{F#FF0000}"
        } else {
            ""
        },
        bat_num_2.to_string() + "%",
        if bat_num_2 < 10 {
            "  "
        } else {
            if bat_num_2 == 100 { "" } else { " " }
        },
        bat_vec.join("")
    )
}
pub fn disk() -> String {
    let df: Df;
    unsafe {
        df = df_root();
    }
    let n = ((df.used as f32) / (df.total as f32)) * 100.0;
    let mut disk_int = n as i32;

    let mut disk_vec = Vec::<String>::new();
    for i in 0..20 {
        if disk_int > 5 {
            disk_vec.push(
                format!(
                    "%{{B{}}}%{{F{0}}}-%{{F-}}%{{B-}}",
                    color_to_string(unsafe { br_gradient(i * 5) })
                )
            );
            disk_int -= 5;
        } else {
            disk_vec.push(
                format!("%{{F{}}}-%{{F-}}", color_to_string(unsafe { br_gradient(i * 5) }))
            );
        }
    }
    format!(
        "Disk ({:.2}%){} [{}] ",
        n,
        if disk_int < 10 {
            "  "
        } else {
            if disk_int == 100 { "" } else { " " }
        },
        disk_vec.join("")
    )
}