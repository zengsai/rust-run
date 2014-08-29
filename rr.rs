use std::os;
use std::io;

fn main() {
    let args = os::args();
    let _prog = args[0].clone();
    let src_file = args[1].clone();
    let exec_args = args.slice_from(2);

    match io::Command::new("rustc").arg("-o").arg("/tmp/rust-exe").arg(src_file).spawn() {
        Ok(mut p) => {
            let output = p.stdout.get_mut_ref().read_to_string().unwrap() + p.stderr.get_mut_ref().read_to_string().unwrap();

            if output.len() != 0 {
                println!("COMPILE:\n================================================================================");
                print!("{}", output);

                if output.as_slice().contains("error:") {
                    return;
                }
                println!("\nEXECUTE:\n================================================================================");
            }
        },
        Err(e) => fail!("failed to execute process: {}", e),
    };

    match io::Command::new("/tmp/rust-exe").args(exec_args.as_slice()).spawn() {
        Ok(mut p) => {
            let output = p.stdout.get_mut_ref().read_to_string().unwrap() + p.stderr.get_mut_ref().read_to_string().unwrap();
            if output.len() != 0 {
                print!("{}", output);
            }
        },
        Err(e) => fail!("failed to execute process: {}", e),
    };
}
