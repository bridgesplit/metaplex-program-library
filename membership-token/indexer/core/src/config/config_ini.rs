use ini::Ini;

use super::lodable_config::LoadableConfig;

#[derive(Default)]
pub struct ConfigIni {
    conf: Ini,
}

impl<'a> LoadableConfig<'a> for ConfigIni {
    fn load(&mut self) -> Result<&Self, &'a str> {
        if let Ok(conf) = Ini::load_from_file("data_loader.ini") {
            self.conf = conf;
            Ok(self)
        } else {
            Err("dataloader.ini file is missed")
        }
    }

    fn set(&self, _section: &str, _key: &str, _valuee: &str) {
        todo!()
    }

    fn get(&self, sec: &str, key: &str) -> Option<&str> {
        if let Some(section) = self.conf.section(Some(sec)) {
            section.get(key)
        } else {
            None
        }
    }
}
