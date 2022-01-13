use std::{
    env::var,
    fs::{read_link, read_to_string},
    process::Command,
    str::from_utf8
};

use chrono::prelude::*;
use sysinfo::{ComponentExt, DiskExt, System, SystemExt};

fn main() {
    let mut s = System::new();
    s.refresh_components_list();
    s.refresh_disks_list();
    s.refresh_memory();

    // may as well hardcode this stuff as it's unlikely to change
    let wm = "awesome";
    let term = "alacritty";
    let editor = "neovim";
    let browser = "qutebrowser";
    let font = "Iosevka [ [3mVictor Mono[0m ]";
    // wallpaper

    let dt = Local::now();
    let mid = dt.format("%H:%M   %a %d %b");

    let ctemp = s.components()[5].temperature();
    // gpu temp
    let ram = fmt_ram(s.used_memory(), s.total_memory());

    // disk usage --
    let d = &s.disks()[0];
    let d_a = d.available_space();
    let d_t = d.total_space();
    let d_bar = [&fmt_disk(d_a, d_t), " [ "].join("");
    let d_used = (((d_t - d_a) as f64 / 1000000000.0) * 10.0).round() / 10.0;
    let d_close = "GB ]";
    // --

    let kernel = s.kernel_version().unwrap();

    // pacman --
    let mut pkgs = read_to_string(
        [
            var("XDG_CACHE_HOME").unwrap(),
            "moodfetch".to_string(),
            "pkg_stats".to_string()
        ]
        .join("/")
    )
    .unwrap_or_else(|_| "0 0 ".to_string());
    pkgs.pop();
    let p_vec = pkgs.split(' ').collect::<Vec<_>>();
    let npkg = match p_vec[0] == "0" {
        true => "?!",
        false => p_vec[0]
    };
    let mpkg = match p_vec[1] == "0" {
        true => "?!",
        false => p_vec[1]
    };
    // --
    // uptime

    println!(
        "                        [1m[38;5;16m`
                       [1m[38;5;16m-:                        [0m
                      [1m[38;5;16m.//:                       [1;3m[38;5;16mEnvironment \
         Highlights...[0m
                     [1m[38;5;16m`////-                      \
         [0m[38;5;18m─────────────────────────[0m
                    [1m[38;5;16m`://///.                     [1;31m  ▐ [0m{}
                    [1m[38;5;16m:///////.                    [1;32m  ▐ [0m{}
                   [1m[38;5;16m-/////////.                   [1;33m  ▐ [0m{}
                  [1m[38;5;16m`://////////`                  [1;34m爵 ▐ [0m{}
                 [1m[38;5;16m-:..://///////`                 [1;35m  ▐ [0m{}
                [1m[38;5;16m-////:::////////`                [1;36m  ▐ [0m{}
               [1m[38;5;16m-/////////////////`               [0m
              [1m[38;5;16m-//////////++++/////`      [0m[38;5;18m┌──────────────────────────┐[0m
             [1m[38;5;16m-////[0m[38;5;17m++++oooooooooo+++.     [0m[38;5;20m    {} [0m
            [0m[38;5;17m-/+++oooooooooooooooooo+.    [0m[38;5;18m└──────────────────────────┘[0m
           [0m[38;5;17m:+oooooooo+-..-/+oooooooo+.           [0m
         [0m[38;5;17m`/ooooooooo:`     .+oooooooo+.          [1;3m[38;5;17mSystem Information...[0m
        [0m[38;5;17m`/ooooooooo/        .ooooooooo+-         [0m[38;5;18m─────────────────────[0m
       [0m[38;5;17m`/oooooooooo`         /oooooo++++-        [1;91m糖 ▐ [0m[3;90mcpu:[0m {} \
         [3;90mgpu: [0m{}
      [0m[38;5;17m`+ooooooooooo`         :oooooo++/-:.       [1;92m  ▐ [0m{}
     [0m[38;5;17m.+ooooooooooo+`         :+oooooooo+/-`      [1;93m  ▐ [0m{}{:.1}{}
    [0m[38;5;17m.+oooooo++/:-.`          `..-:/++ooooo+:     [1;94m  ▐ [0m{}
   [0m[38;5;17m.+oo++/-.`                      `.-:++ooo:    [1;95m  ▐ [0m[3;90mOfficial:[0m {} \
         [3;90mAUR/Local:[0m {}
  [0m[38;5;17m-++/-.`                               `-:++/`  [1;96m  ▐ [0m{}
  [0m[38;5;17m-++/-.`         [1;3;30m[48;5;16m Arch Linux! [0m[38;5;17m         `-:++/`  [0m
 [0m[38;5;17m.:.`                                       .--[0m",
        wm,
        term,
        editor,
        browser,
        font,
        wall(),
        mid,
        ctemp,
        gtemp(),
        ram,
        d_bar,
        d_used,
        d_close,
        kernel,
        npkg,
        mpkg,
        uptime(s.uptime())
    );
}

fn wall() -> String {
    let root = var("XDG_CONFIG_HOME").unwrap();
    let sl = [&root, "awesome", "assets", "wallpaper"];
    let p = match read_link(sl.join("/")) {
        Ok(x) => x,
        Err(_) => return "[0m[3m[38;5;18mUnknown![0m".to_string()
    };
    let mut f = p.file_name().unwrap().to_string_lossy().to_string();
    match f.len() <= 28 {
        true => f,
        false => {
            f.truncate(26);
            [f, "…".to_string()].join("")
        }
    }
}

fn gtemp() -> String {
    let mut fb = false;
    let nvsmi = match Command::new("/usr/bin/nvidia-smi")
        .args(&["stats", "-d", "temp", "-c", "1"])
        .output()
    {
        Ok(r) => from_utf8(&r.stdout).unwrap().to_owned(),
        Err(_) => {
            fb = true;
            "!?".to_string()
        }
    };
    match fb {
        true => nvsmi,
        false => {
            let x = nvsmi.split(", ").collect::<Vec<&str>>();
            let mut r = x.last().unwrap().to_string();
            r.pop();
            match r.parse::<u8>() {
                Ok(_) => r,
                Err(_) => nvsmi
            }
        }
    }
}

fn get_bar(n: i8) -> String {
    let r = match n {
        0..=10 => "[38;5;20m[38;5;18m[0m",
        11..=20 => "[38;5;20m[38;5;18m[0m",
        21..=30 => "[38;5;20m[38;5;18m[0m",
        31..=40 => "[38;5;20m[38;5;18m[0m",
        41..=50 => "[38;5;20m[38;5;18m[0m",
        51..=60 => "[38;5;20m[38;5;18m[0m",
        61..=70 => "[38;5;20m[38;5;18m[0m",
        71..=80 => "[38;5;20m[38;5;18m[0m",
        81..=90 => "[38;5;202m[38;5;18m[0m",
        91..=100 => "[38;5;196m[0m",
        _ => unreachable!()
    };
    r.to_string()
}

fn fmt_ram(u: u64, t: u64) -> String {
    let used = [(u / 1000).to_string(), "MB".to_string()].join("");
    let mut perc = u as f64 / t as f64 * 100.00;
    perc = perc.round();
    let bar = get_bar(perc as i8);
    [bar, " [ ".to_string(), used, " ]".to_string()].join("")
}

fn fmt_disk(a: u64, t: u64) -> String {
    let u = t - a;
    let mut perc = u as f64 / t as f64 * 100.00;
    perc = perc.round();
    get_bar(perc as i8)
}

fn uptime(s: u64) -> String {
    let mut r = vec![];

    let mins = s as i64 / 60;
    let hrs = mins / 60;
    let days = hrs / 24;

    match days > 0 {
        true if days == 1 => r.push([days.to_string(), " day, ".to_string()].join("")),
        true => r.push([days.to_string(), " days, ".to_string()].join("")),
        false => {}
    }

    match hrs > 0 {
        true if hrs == 1 => r.push([(hrs % 24).to_string(), " hour, ".to_string()].join("")),
        true => r.push([(hrs % 24).to_string(), " hours, ".to_string()].join("")),
        false => {}
    }

    match mins > 0 {
        true if mins == 1 => r.push([(mins % 60).to_string(), " minute".to_string()].join("")),
        true => r.push([(mins % 60).to_string(), " minutes".to_string()].join("")),
        false => r.push([(s % 60).to_string(), " seconds".to_string()].join(""))
    }

    r.join("")
}
