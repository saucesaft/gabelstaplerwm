/*
 * Copyright Inokentiy Babushkin and contributors (c) 2016-2017
 *
 * All rights reserved.

 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *
 *     * Redistributions of source code must retain the above copyright
 *       notice, this list of conditions and the following disclaimer.
 *
 *     * Redistributions in binary form must reproduce the above
 *       copyright notice, this list of conditions and the following
 *       disclaimer in the documentation and/or other materials provided
 *       with the distribution.
 *
 *     * Neither the name of Inokentiy Babushkin nor the names of other
 *       contributors may be used to endorse or promote products derived
 *       from this software without specific prior written permission.

 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

use std::cmp::Ordering;
use std::process::Command;

use toml;

use xkb;

use kbd::error::*;

/// An index representing a mode.
pub type Mode = usize;

/// A mode switching action.
#[derive(Clone, Copy, Debug)]
pub enum ModeSwitch {
    /// A mode switching action changing the current mode permanently.
    Permanent(Mode),
    /// A temporary mode switching action, changing behaviour only for the next chain.
    Temporary(Mode),
}

/// A command to be executed in reaction to specific key events.
#[derive(Debug)]
pub enum Cmd {
    /// A string to be passed to a shell to execute the command.
    Shell(String),
    /// A mode to switch to.
    ModeSwitch(ModeSwitch),
}

impl Cmd {
    /// Run a command and possibly return an resulting mode switching action to perform.
    pub fn run(&self) -> Option<ModeSwitch> {
        match *self {
            Cmd::Shell(ref repr) => {
                let _ = Command::new("sh").args(&["-c", repr]).spawn();
                None
            },
            Cmd::ModeSwitch(ref switch) => {
                Some(*switch)
            },
        }
    }

    /// Construct a command from a TOML value.
    pub fn from_value(bind_str: String, value: toml::Value) -> KbdResult<Cmd> {
        if let toml::Value::String(repr) = value {
            Ok(Cmd::Shell(repr))
        } else {
            Err(KbdError::KeyTypeMismatch(bind_str, true))
        }
    }
}

/// A keysy wrapper used for various trait implementations.
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub struct Keysym(pub xkb::Keysym); // TODO: encapsulate

impl Ord for Keysym {
    fn cmp(&self, other: &Keysym) -> Ordering {
        let self_inner: u32 = self.0.into();

        self_inner.cmp(&other.0.into())
    }
}

impl PartialOrd for Keysym {
    fn partial_cmp(&self, other: &Keysym) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}