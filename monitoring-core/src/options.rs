use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CollectorOptions {
    cpu: bool, 
    memory: bool, 
    os: bool, 
    network: bool, 
    filesystem: bool, 
    swap: bool
}

impl CollectorOptions {
    pub fn new(cpu: bool, memory: bool, os: bool, network: bool, filesystem: bool, swap: bool) -> Self {
        Self {
            cpu, 
            memory, 
            os, 
            network, 
            filesystem, 
            swap,
        }
    }

    pub fn cpu(&self) -> bool {
        self.cpu
    }

    pub fn memory(&self) -> bool {
        self.memory
    }

    pub fn os(&self) -> bool {
        self.os
    }

    pub fn filesystem(&self) -> bool {
        self.filesystem
    }

    pub fn swap(&self) -> bool {
        self.swap
    }

    pub fn network(&self) -> bool {
        self.network
    }
}