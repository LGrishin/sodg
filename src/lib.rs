// Copyright (c) 2022 Yegor Bugayenko
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included
// in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NON-INFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

//! This is a memory structure with vertices and edges between them,
//! which we call Surging Object DiGraph (SODG), because it expects
//! modifications comping from a user (through [`Sodg::add`],
//! [`Sodg::bind`], and [`Sodg::put`]) and then decides itself when
//! it's time to delete some vertices (something similar to
//! "garbage collection"). For example, here is how you create a simple
//! graph with two vertices and an edge between them:
//!
//! ```
//! use sodg::Sodg;
//! let mut sodg = Sodg::empty();
//! sodg.add(0).unwrap();
//! sodg.add(1).unwrap();
//! sodg.bind(0, 1, "foo").unwrap();
//! ```

#![doc(html_root_url = "https://docs.rs/sodg/0.0.0")]
#![deny(warnings)]

mod alerts;
mod ctors;
mod debug;
mod edge;
mod hex;
mod inspect;
mod merge;
mod misc;
mod next;
mod ops;
mod script;
mod serialization;
mod slice;
mod vertex;
mod xml;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub use crate::hex::Hex;
/// Instances of this type can be used in [`Sodg::alert_on`] method,
/// in order to ensure runtime consistency of data inside the graph.
pub type Alert = fn(g: &Sodg, vx: Vec<u32>) -> Vec<String>;

/// This struct represents a Surging Object DiGraph (SODG). You add vertices
/// to it, bind them one to one with edges, put data into some of them,
/// and read data back:
///
/// ```
/// use sodg::Sodg;
/// let mut sodg = Sodg::empty();
/// sodg.add(0).unwrap();
/// sodg.add(1).unwrap();
/// sodg.bind(0, 1, "a").unwrap();
/// sodg.add(2).unwrap();
/// sodg.bind(1, 2, "b").unwrap();
/// assert_eq!(2, sodg.find(0, "a.b").unwrap());
/// ```
///
/// This package is used in [reo](https://github.com/objectionary/reo)
/// project, as a memory model for objects and dependencies between them.
#[derive(Serialize, Deserialize)]
pub struct Sodg {
    vertices: HashMap<u32, Vertex>,
    #[serde(skip_serializing, skip_deserializing)]
    next_v: u32,
    #[serde(skip_serializing, skip_deserializing)]
    alerts: Vec<Alert>,
    #[serde(skip_serializing, skip_deserializing)]
    alerts_active: bool,
}

/// It is a wrapper of a plain text with graph-modifying
/// instructions, for example:
///
/// ```text
/// ADD(0);
/// ADD($ν1); # adding new vertex
/// BIND(0, $ν1, foo);
/// PUT($ν1, d0-bf-D1-80-d0-B8-d0-b2-d0-b5-d1-82);
/// ```
///
/// In the script you can use "variables", similar to `$ν1` used
/// in the text above. They will be replaced by autogenerated numbers
/// during the deployment of this script to a [`Sodg`].
pub struct Script {
    txt: String,
    vars: HashMap<String, u32>,
    root: u32,
}

/// Edge between vertices in the graph.
#[derive(Clone, Serialize, Deserialize, Eq, PartialOrd, PartialEq, Ord)]
struct Edge {
    pub to: u32,
    pub a: String,
}

/// A vertex in the graph.
#[derive(Clone, Serialize, Deserialize)]
struct Vertex {
    pub edges: Vec<Edge>,
    pub data: Hex,
}

#[cfg(test)]
use simple_logger::SimpleLogger;

#[cfg(test)]
use log::LevelFilter;

#[cfg(test)]
#[ctor::ctor]
fn init() {
    SimpleLogger::new()
        .without_timestamps()
        .with_level(LevelFilter::Trace)
        .init()
        .unwrap();
}
