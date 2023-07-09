use std::{
    env, fs,
    process::{exit, Command},
};

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let check_path = format!("{out_dir}/check-fcntl.out");

    let ec = Command::new("cc")
        .args(["-o", &check_path, "check.c"])
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
    assert!(ec.success());
    let out = Command::new(&check_path).output().unwrap();
    if !out.status.success() {
        eprintln!("{}", String::from_utf8_lossy(&out.stderr));
        exit(1);
    }

    let out = String::from_utf8_lossy(&out.stdout);
    let mut f = out.split(' ').take(3);
    let (Some(fd), Some(cmd), Some(arg)) = (f.next(), f.next(), f.next()) else {
        panic!("invalid output from check.c: {out}");
    };

    let consts_path = format!("{out_dir}/_consts.rs");
    fs::write(
        &consts_path,
        format!(
            concat!(
                "const STDOUT_FD: c_int = {};\n",
                "const F_SETFL: c_int = {};\n",
                "const O_NONBLOCK: c_int = {};\n",
            ),
            fd, cmd, arg
        ),
    )
    .unwrap();
}
