extern crate sysctl;

use sysctl::Sysctl;
use clap::App;

const CMD_INCREASE: &str = "increase";
const CMD_DECREASE: &str = "decrease";
const CMD_STEP: i32 = 25;

fn main() {
   let matches = App::new("Keyboard backlight control")
        .version("1.0")
        .author("Vasco K. <vasco.kk@gmail.com>")
        .about("Change keyboard backlight intensity")
        .subcommand(App::new(CMD_INCREASE)
                    .about("Increase backlight intensity"))
        .subcommand(App::new(CMD_DECREASE)
                    .about("Decrease backlight intensity"))
        .get_matches();

    let mib = "dev.asmc.0.light.control";
    let ctl = sysctl::Ctl::new(mib).unwrap();
    println!("Description: {}", ctl.description().unwrap());
    let current_val: i32 = ctl.value_string().expect("Can't get the MIB value").parse().unwrap();
    println!("Value: {}", current_val);

    match matches.subcommand_name().expect("Missing subcommand"){
        CMD_INCREASE => increase(&ctl, current_val),
        CMD_DECREASE => decrease(&ctl, current_val),
        _ => panic!("Unrecognised subcommand!")
    };
}

fn set_new_value(ctl: &sysctl::Ctl, new_value: i32){
    ctl.set_value(sysctl::CtlValue::Int(new_value)).unwrap();
}

fn increase(ctl: &sysctl::Ctl, current_val: i32) {
        let new_value = current_val + CMD_STEP;
        if new_value > 255 {
            set_new_value(ctl, 255);
        } else {
            set_new_value(ctl, new_value);
        }
}

fn decrease(ctl: &sysctl::Ctl, current_val: i32) {
       let new_value = current_val - CMD_STEP;
          if new_value < 0 {
              set_new_value(ctl, 0);
          } else {
              set_new_value(ctl, new_value);
          }
}
