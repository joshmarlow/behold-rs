//! # Behold
//! `behold` is a simple library that allows contextual debugging.
#[macro_use]
extern crate lazy_static;

use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

/// The core data structure - stores shared global context and instance specific configuration
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
    /// Create a new Behold instance
    pub fn new() -> Self {
        BEHOLD.clone()
    }

    /// Set the value of the global Behold context
    /// # Examples
    /// ```
    /// use behold::behold;
    /// behold().when_context("do-it").show("Hello world!".to_string())
    /// ```
    /// Will output nothing.
    /// ```
    /// use behold::behold;
    /// behold().set_context("do-it", true);
    /// behold().when_context("do-it").show("Hello world!".to_string())
    /// ```
    /// Will produce the output:
    /// ```ignore
    /// "Hello world!"
    /// ```
    pub fn set_context(&self, key: &str, value: bool) {
        let context = (*self.context).lock();

        if let Ok(mut context) = context {
            (*context).insert(key.to_string(), value);
        } else if let Err(err) = context {
            panic!(
                "when_context called on an instance of Behold - mutex already acquired - {:?}!",
                err
            );
        }
    }

    /// Return a Behold instance that appends output with the specified tag
    /// # Examples
    /// ```
    /// use behold::behold;
    /// behold().tag("apples").show("Hello world!".to_string());
    /// ```
    /// Will produce the output:
    /// ```ignore
    /// "Hello world!, apples"
    /// ```
    pub fn tag(&self, tag: &str) -> Self {
        Behold {
            context: self.context.clone(),
            speak_up: self.speak_up,
            tag: Some(tag.to_string()),
        }
    }

    /// Produce a behold instance which can speak up or not, depending on the parameter
    /// # Examples
    /// ```
    /// use behold::behold;
    /// behold().when(0 % 2 == 1).show("Hello world!".to_string())
    /// ```
    /// Will output nothing.
    /// ```
    /// use behold::behold;
    /// behold().when(0 % 2 == 0).show("Hello world!".to_string())
    /// ```
    /// Will output
    /// ```ignore
    /// "Hello world!, apples"
    /// ```
    pub fn when(&self, speak_up: bool) -> Self {
        Behold {
            context: self.context.clone(),
            speak_up: speak_up,
            tag: self.tag.clone(),
        }
    }

    /// Produce a behold instance which can speak up or not, depending on the specified context
    /// # Examples
    /// ```
    /// use behold::behold;
    /// behold().when_context("do-it").show("Hello world!".to_string())
    /// ```
    /// Will output nothing.
    /// ```
    /// use behold::behold;
    /// behold().set_context("do-it", true);
    /// behold().when_context("do-it").show("Hello world!".to_string())
    /// ```
    /// Will output
    /// ```ignore
    /// "Hello world!"
    /// ```
    pub fn when_context(&self, key: &str) -> Self {
        let speak_up = match (*self.context).lock() {
            Ok(context) => (*context).get(&key.to_string()).cloned().unwrap_or_default(),
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
    /// # Examples
    /// ```
    /// use behold::behold;
    /// behold().show("Hello world!".to_string());
    /// ```
    /// Will produce the output:
    /// ```ignore
    /// "Hello world!"
    /// ```
    pub fn show(&self, msg: String) {
        if self.speak_up {
            if let Some(ref tag) = self.tag {
                println!("{}, {}", msg, tag);
            } else {
                println!("{}", msg);
            }
        }
    }

    /// Call the provided function if this behold instance is configured to speak up
    /// # Examples
    /// ```
    /// use behold::behold;
    /// behold().call(&|| { println!("Hello world!"); } );
    /// ```
    /// Will output:
    /// ```ignore
    /// "Hello world!"
    /// ```
    /// Whereas the following:
    ///```
    /// use behold::behold;
    /// behold().when(false).call(&|| { println!("Hello world!"); } );
    /// ```
    /// Will output nothing.
    pub fn call(&self, f: &Fn()) {
        if self.speak_up {
            f()
        }
    }
}

/// Convenience function for quickly constructing a behold instance.
///
/// # Examples
///
/// ```rust
/// use behold::behold;
/// behold().show("Hello world!".to_string());
/// ```
/// Will produce the output:
/// ```ignore
/// "Hello world!"
/// ```
pub fn behold() -> Behold {
    Behold::new()
}

lazy_static! {
    static ref BEHOLD: Behold = {
    	Behold {
	    	context: Arc::new(Mutex::new(BTreeMap::new())),
	    	speak_up: true,
	    	tag: None,
	    }
	};
}


#[cfg(test)]
include!(concat!(env!("OUT_DIR"), "/skeptic-tests.rs"));
