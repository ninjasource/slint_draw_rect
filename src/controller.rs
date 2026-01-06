// Copyright © 2025 David Haig
// SPDX-License-Identifier: MIT

use embassy_sync::channel::Channel;

use crate::error;

#[derive(Debug, Clone)]
pub enum Action {}

type ActionChannelType =
    Channel<embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex, Action, 2>;

pub static ACTION: ActionChannelType = Channel::new();

pub struct Controller {}

impl Controller {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn run(&mut self) {
        self.set_action_event_handlers();

        loop {
            let action = ACTION.receive().await;

            match self.process_action(action).await {
                Ok(()) => {
                    // all good
                }
                Err(e) => {
                    error!("process action: {:?}", e);
                }
            }
        }
    }

    pub async fn process_action(&mut self, _action: Action) -> Result<(), ()> {
        Ok(())
    }

    // user initiated action event handlers
    fn set_action_event_handlers(&self) {}
}
