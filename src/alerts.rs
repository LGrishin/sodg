// Copyright (c) 2022-2023 Yegor Bugayenko
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

use crate::{Alert, Sodg};
use anyhow::anyhow;
use anyhow::Result;

impl<const M: usize, const N: usize> Sodg<M, N> {
    /// Attach a new alert to this graph.
    ///
    /// For example, you don't want
    /// more than one edge to depart from any vertex:
    ///
    /// ```
    /// use std::str::FromStr;
    /// use sodg::{Label, Sodg};
    /// let mut g : Sodg<16, 4> = Sodg::empty();
    /// g.alerts_on().unwrap();
    /// g.alert_on(|g, vx| {
    ///   for v in vx {
    ///     if g.kids(v).unwrap().len() > 1 {
    ///       return vec![format!("Too many kids at ν{v}")];
    ///     }
    ///   }
    ///   return vec![];
    /// });
    /// g.add(0).unwrap();
    /// g.add(1).unwrap();
    /// g.add(2).unwrap();
    /// g.bind(0, 1, Label::from_str("first").unwrap()).unwrap();
    /// assert!(g.bind(0, 2, Label::from_str("second").unwrap()).is_err());
    /// ```
    pub fn alert_on(&mut self, a: Alert<M, N>) {
        self.alerts.push(a);
    }

    /// Disable all alerts.
    pub fn alerts_off(&mut self) {
        self.alerts_active = false;
    }

    /// Enable all alerts.
    ///
    /// This function also runs all vertices through
    /// all checks and returns the list of errors found. If everything
    /// was fine, an empty vector is returned.
    ///
    /// # Errors
    ///
    /// An error may be returned if validation fails, after the alerts are turned ON.
    pub fn alerts_on(&mut self) -> Result<()> {
        self.alerts_active = true;
        let mut keys = vec![];
        for (v, _) in self.vertices.iter() {
            keys.push(*v);
        }
        self.validate(keys)
    }

    /// Check all alerts for the given list of vertices.
    ///
    /// # Errors
    ///
    /// If any of them have any issues, `Err` is returned.
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn validate(&self, vx: Vec<u32>) -> Result<()> {
        if self.alerts_active {
            for a in &self.alerts {
                let msgs = a(self, vx.clone());
                if !msgs.is_empty() {
                    return Err(anyhow!("{}", msgs.join("; ")));
                }
            }
        }
        Ok(())
    }
}

#[test]
fn panic_on_simple_alert() -> Result<()> {
    let mut g: Sodg<10, 10> = Sodg::empty();
    g.alerts_on()?;
    g.alert_on(|_, _| vec![format!("{}", "oops")]);
    assert!(g.add(0).is_err());
    Ok(())
}

#[test]
fn dont_panic_when_alerts_disabled() -> Result<()> {
    let mut g: Sodg<4, 4> = Sodg::empty();
    g.alert_on(|_, _| vec!["should never happen".to_string()]);
    g.alerts_off();
    assert!(g.add(0).is_ok());
    Ok(())
}

#[test]
fn panic_on_complex_alert() -> Result<()> {
    let mut g: Sodg<4, 4> = Sodg::empty();
    g.alert_on(|_, vx| {
        let v = 42;
        if vx.contains(&v) {
            vec![format!("Vertex ν{v} is not allowed")]
        } else {
            vec![]
        }
    });
    g.alerts_on()?;
    assert!(g.add(42).is_err());
    Ok(())
}
