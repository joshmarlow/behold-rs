#[macro_use]
extern crate lazy_static;

use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct Behold {
    /// Context to determine when to speak up
    context: Arc<Mutex<BTreeMap<String, bool>>>,
    /// Determine if this behold instance should produce output
    speak_up: bool,
    /// Suffix used when displaying output
    tag: Option<String>,
}

impl Behold {
    pub fn new() -> Self {
        BEHOLD.clone()
    }

    /// Set the value of the *global* Behold context
    pub fn set_context(&self, key: String, value: bool) {
        let context = (*self.context).lock();

        if let Ok(mut context) = context {
            (*context).insert(key, value);
        } else if let Err(err) = context {
            panic!(
                "when_context called on an instance of Behold - mutex already acquired - {:?}!",
                err
            );
        }
    }

    /// Return a Behold instance that appends output with the specified tag
    pub fn tag(&self, tag: String) -> Self {
        Behold {
            context: self.context.clone(),
            speak_up: self.speak_up,
            tag: Some(tag),
        }
    }

    pub fn tag_str(&self, tag: &str) -> Self {
        self.tag(tag.to_string())
    }

    /// Produce a behold instance which can speak up or not, depending on the parameter
    pub fn when(&self, speak_up: bool) -> Self {
        Behold {
            context: self.context.clone(),
            speak_up: speak_up,
            tag: self.tag.clone(),
        }
    }

    /// Produce a behold instance which can speak up or not, depending on the specified context
    pub fn when_context(&self, key: &String) -> Self {
        let speak_up = match (*self.context).lock() {
            Ok(context) => (*context).get(key).cloned().unwrap_or_default(),
            Err(err) => {
                panic!(
                    "when_context called on an instance of Behold - mutex already acquired - {:?}!",
                    err
                );
            }
        };
        Behold {
            context: self.context.clone(),
            speak_up: speak_up,
            tag: self.tag.clone(),
        }
    }

    /// Print the provided string if this behold instance is configured to speak up
    pub fn show(&self, msg: String) {
        if self.speak_up {
            if let Some(ref tag) = self.tag {
                println!("{}, {}", msg, tag);
            } else {
                println!("{}", msg);
            }
        }
    }

    pub fn show_str(&self, msg: &str) {
        self.show(msg.to_string())
    }
}

pub fn behold() -> Behold {
    Behold::new()
}

lazy_static! {
    pub static ref BEHOLD: Behold = {
    	Behold {
	    	context: Arc::new(Mutex::new(BTreeMap::new())),
	    	speak_up: true,
	    	tag: None,
	    }
	};
}


#[cfg(test)]
include!(concat!(env!("OUT_DIR"), "/skeptic-tests.rs"));