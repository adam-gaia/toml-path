use log::debug;

const RAW_OUTPUT_DEFAULT: bool = true;
const JSON_OUTPUT_DEFAULT: bool = false;
const COMPACT_OUTPUT_DEFAULT: bool = false;

fn spacing(compact: bool) -> String {
    if compact {
        String::new()
    } else {
        String::from(" ")
    }
}

fn separator(compact: bool, json: bool) -> String {
    let mut s = String::new();
    s.push_str(&spacing(compact));
    if json {
        s.push(':');
    } else {
        s.push('=');
    }
    s.push_str(&spacing(compact));
    s
}

/// TODO: doc comments
#[derive(Debug)]
pub struct Settings {
    pub raw_output: bool,
    pub compact_output: bool,
    pub json_output: bool,
    pub separator: String,
    pub spacing: String,
}

impl Settings {
    fn build(raw_output: bool, json_output: bool, compact_output: bool) -> Self {
        debug!(
            "raw: {}, json: {}, compact: {}",
            raw_output, json_output, compact_output
        );
        // TODO: There is definitly a better way to enforce this
        // TODO: at least note this panics in docs
        assert!(
            !(raw_output && json_output),
            "Settings 'raw output' and 'json output' are mutualy exclusive"
        );
        Self {
            raw_output,
            json_output,
            compact_output,
            separator: separator(compact_output, json_output),
            spacing: spacing(compact_output),
        }
    }

    pub fn new() -> Self {
        Self::default()
    }

    pub fn builder() -> SettingsBuilder {
        SettingsBuilder::default()
    }
}

impl Default for Settings {
    fn default() -> Self {
        let raw_output = RAW_OUTPUT_DEFAULT;
        let json_output = JSON_OUTPUT_DEFAULT;
        let compact_output = COMPACT_OUTPUT_DEFAULT;
        Settings::build(raw_output, json_output, compact_output)
    }
}

#[derive(Debug)]
pub struct SettingsBuilder {
    raw_output: bool,
    compact_output: bool,
    json_output: bool,
}

impl Default for SettingsBuilder {
    fn default() -> Self {
        Self {
            raw_output: RAW_OUTPUT_DEFAULT,
            compact_output: COMPACT_OUTPUT_DEFAULT,
            json_output: JSON_OUTPUT_DEFAULT,
        }
    }
}

impl SettingsBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    /// Setting this also disables json output
    pub fn raw_output(mut self, enable: bool) -> Self {
        self.raw_output = enable;
        if enable {
            self.json_output = false;
        }
        self
    }

    pub fn compact_output(mut self, enable: bool) -> Self {
        self.compact_output = enable;
        self
    }

    /// Setting this also disables raw output
    pub fn json_output(mut self, enable: bool) -> Self {
        self.json_output = enable;
        if enable {
            self.raw_output = false;
        }
        self
    }

    pub fn build(mut self) -> Settings {
        Settings::build(self.raw_output, self.json_output, self.compact_output)
    }
}
