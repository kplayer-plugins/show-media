extern crate kplayer_rust_wrap;

use kplayer_rust_wrap::kplayer;

struct ShowOverlay {}
impl ShowOverlay {
    fn new() -> Self {
        ShowOverlay {}
    }
}

impl kplayer::plugin::BasePlugin for ShowOverlay {
    fn get_name(&self) -> String {
        String::from("show-media")
    }
    fn get_args(
        &mut self,
        _custom_args: std::collections::HashMap<String, String>,
    ) -> std::vec::Vec<std::string::String> {
        let mut args: Vec<std::string::String> = Vec::new();
        args.push(String::from("x=0"));
        args.push(String::from("y=0"));
        args.push(String::from("shortest=1"));
        args
    }
    fn get_author(&self) -> std::string::String {
        String::from("kplayer")
    }
    fn get_filter_name(&self) -> std::string::String {
        String::from("overlay")
    }
    fn get_media_type(&self) -> kplayer::plugin::MediaType {
        kplayer::plugin::MediaType::MediaTypeVideo
    }
    fn validate_user_args(
        &self,
        _args: std::collections::HashMap<String, String>,
    ) -> std::result::Result<bool, &'static str> {
        Ok(true)
    }
}

// movie
struct ShowMovie {}
impl ShowMovie {
    fn new() -> Self {
        ShowMovie {}
    }
}

impl kplayer::plugin::BasePlugin for ShowMovie {
    fn get_name(&self) -> String {
        String::from("show-media")
    }
    fn get_args(
        &mut self,
        _custom_args: std::collections::HashMap<String, String>,
    ) -> std::vec::Vec<std::string::String> {
        let mut args: Vec<std::string::String> = Vec::new();
        args.push(String::from("filename=exmaple.png"));
        args.push(String::from("loop=0"));
        args
    }
    fn get_author(&self) -> std::string::String {
        String::from("kplayer")
    }
    fn get_filter_name(&self) -> std::string::String {
        String::from("movie")
    }
    fn get_media_type(&self) -> kplayer::plugin::MediaType {
        kplayer::plugin::MediaType::MediaTypeVideo
    }
    fn validate_user_args(
        &self,
        _args: std::collections::HashMap<String, String>,
    ) -> std::result::Result<bool, &'static str> {
        for (key, value) in _args {
            // validate image file exist
            if key == String::from("filename") {
                if !kplayer::util::os::file_exist(&value) {
                    self.print_log(
                        kplayer::util::os::PrintLogLevel::ERROR,
                        format!("image file not eixst: {}", &value).as_str(),
                    );
                    let result_err =
                        Box::leak(format!("image file not exist: {}", &value).into_boxed_str());
                    return Err(result_err);
                }
            }
        }
        Ok(true)
    }
}

// setpts
struct SetPts {
    framerate: String,
}
impl SetPts {
    fn new() -> Self {
        SetPts {
            framerate: String::from("5"),
        }
    }
}
impl kplayer::plugin::BasePlugin for SetPts {
    fn get_name(&self) -> String {
        String::from("show-media")
    }
    fn get_args(
        &mut self,
        _custom_args: std::collections::HashMap<String, String>,
    ) -> std::vec::Vec<std::string::String> {
        let value = _custom_args.get("framerate");
        if value != None {
            self.framerate = value.unwrap().to_string();
        }

        let mut args: Vec<std::string::String> = Vec::new();
        args.push(format!("N/{}/TB=null", self.framerate));
        args
    }
    fn get_author(&self) -> std::string::String {
        String::from("kplayer")
    }
    fn get_filter_name(&self) -> std::string::String {
        String::from("setpts")
    }
    fn get_media_type(&self) -> kplayer::plugin::MediaType {
        kplayer::plugin::MediaType::MediaTypeVideo
    }
    fn validate_user_args(
        &self,
        _args: std::collections::HashMap<String, String>,
    ) -> std::result::Result<bool, &'static str> {
        let framerate = self.framerate.parse::<i32>();
        match framerate {
            Err(_err) => {
                return Err("framerate value of number invalid");
            }
            Ok(_) => {}
        }

        Ok(true)
    }
}

// scale
struct Scale {}
impl Scale {
    fn new() -> Self {
        Scale {}
    }
}
impl kplayer::plugin::BasePlugin for Scale {
    fn get_name(&self) -> String {
        String::from("show-media")
    }
    fn get_args(
        &mut self,
        _custom_args: std::collections::HashMap<String, String>,
    ) -> std::vec::Vec<std::string::String> {
        let mut args: Vec<std::string::String> = Vec::new();
        args.push(String::from("w=100"));
        args.push(String::from("h=100"));
        args
    }
    fn get_author(&self) -> std::string::String {
        String::from("kplayer")
    }
    fn get_filter_name(&self) -> std::string::String {
        String::from("scale")
    }
    fn get_media_type(&self) -> kplayer::plugin::MediaType {
        kplayer::plugin::MediaType::MediaTypeVideo
    }
    fn validate_user_args(
        &self,
        _args: std::collections::HashMap<String, String>,
    ) -> std::result::Result<bool, &'static str> {
        Ok(true)
    }
    fn get_instance_type(&mut self) -> kplayer::plugin::InstanceType {
        kplayer::plugin::InstanceType::InstanceTypeSide
    }
}

kplayer_rust_wrap::export!(ShowOverlay, Scale, SetPts, ShowMovie);
