//! Native JS Regular Expression wrapper module

use js_sys::RegExp as JsRegExp;
pub struct RegExp(JsRegExp);

#[derive(Clone)]
pub struct Matches {
    pub global: String,
    pub groups: Vec<String>,
}

impl RegExp {
    pub fn new(pattern: &str) -> RegExp {
        RegExp(JsRegExp::new(pattern, "g"))
    }

    pub fn exec(&self, text: &str) -> Vec<Matches> {
        let mut res = Vec::new();
        loop {
            if let Some(matches) = self.0.exec(text) {
                let mut matches_set = Matches {
                    global: String::new(),
                    groups: Vec::new(),
                };
                matches.for_each(&mut |elem, index, _| {
                    if index == 0 {
                        matches_set.global = elem.as_string().unwrap();
                    } else {
                        matches_set.groups.push(elem.as_string().unwrap());
                    }
                    res.push(matches_set.clone())
                })
            } else {
                break;
            }
        }
        res
    }
}
