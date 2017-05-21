
#[macro_use]
extern crate errloc_macros;

#[cfg(windows)]
extern crate winapi;
#[cfg(windows)]
extern crate user32;
#[cfg(windows)]
extern crate advapi32;

#[cfg(windows)]
fn widen(st: &str) -> std::vec::Vec<u16> {
    use std::os::windows::ffi::OsStrExt;
    let wst = std::ffi::OsStr::new(st);
    let it = wst.encode_wide();
    let mut res: std::vec::Vec<u16> = std::vec::Vec::new();
    res.extend(it);
    res.push(0);
    res.shrink_to_fit();
    return res;
}

#[cfg(windows)]
fn narrow(wst: &[u16]) -> std::string::String {
    let st: std::ffi::OsString = std::os::windows::ffi::OsStringExt::from_wide(wst);
    return st.into_string().expect(errloc!());
}

#[cfg(windows)]
fn query_reg() -> std::string::String {
    let keyname = "SOFTWARE\\JavaSoft\\Java Development Kit\\1.8.0_131_1";
    let subkeyname = "JavaHome";
    let widekey = widen(keyname);
    let widesubkey = widen(subkeyname);
    let ret = unsafe {
        // todo: ret checks
        let mut key: winapi::minwindef::HKEY = std::ptr::null_mut();
        let mut pkey: winapi::minwindef::PHKEY = &mut key;
        let err = advapi32::RegOpenKeyExW(winapi::HKEY_LOCAL_MACHINE, widekey.as_ptr(), 0, winapi::winnt::KEY_READ, pkey);
        let mut len: winapi::minwindef::DWORD = 0;
        let mut plen: winapi::minwindef::PDWORD = &mut len;
        advapi32::RegQueryValueExW(key, widesubkey.as_ptr(), std::ptr::null_mut(), std::ptr::null_mut(), std::ptr::null_mut(), plen);
        let mut res: std::vec::Vec<u16> = std::vec::Vec::new();
        res.resize((len + 1) as usize, 0);
        let mut pres: winapi::minwindef::LPBYTE = res.as_mut_ptr() as winapi::minwindef::LPBYTE;
        advapi32::RegQueryValueExW(key, widesubkey.as_ptr(), std::ptr::null_mut(), std::ptr::null_mut(), pres, plen);
        advapi32::RegCloseKey(key);
        narrow(&res[..])
    };
    ret
}

#[cfg(windows)]
fn print_message(msg: &str) -> Result<i32, std::io::Error> {
    let wide: std::vec::Vec<u16> = widen(msg);
    let nar: std::string::String = narrow(&wide[..]);
    let wide2: std::vec::Vec<u16> = widen(nar.as_str());
    let reg = query_reg();
    let wide3 = widen(reg.as_str());
    let ret = unsafe {
        user32::MessageBoxW(std::ptr::null_mut(), wide3.as_ptr(), wide2.as_ptr(), winapi::MB_OK)
    };
    if ret == 0 { 
        Err(std::io::Error::last_os_error()) 
    } else { 
        Ok(ret) 
    }
}

#[cfg(not(windows))]
fn print_message(msg: &str) -> Result<(), Error> {
    println!("{}", msg);
    Ok(())
}

pub fn callwin(msg: &str) {
    print_message(msg).expect(errloc!());
}
