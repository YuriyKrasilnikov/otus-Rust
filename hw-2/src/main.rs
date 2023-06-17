#![allow(dead_code)]

extern crate uuid;

use std::string::ToString;
use uuid::Uuid;

// Умная розетка
#[derive(Clone, Copy)]
struct SmartOutlet<I, P> {
    id: Uuid,
    on: bool,
    info: I,
    power: P,
}

// Предоставлять текстовое описание
trait GetInfo {
    fn get_info(&self) -> String;
}

impl<I: ToString, P> GetInfo for SmartOutlet<I, P> {
    fn get_info(&self) -> String {
        self.info.to_string()
    }
}

// Включаться и выключаться
trait Switch {
    fn switch_mutable(&mut self) -> Self;
    fn switch_immutable(&self) -> Self;
}

impl<I, P> Switch for SmartOutlet<I, P>
where
    I: Copy,
    P: Copy,
{
    fn switch_mutable(&mut self) -> Self {
        self.on = !self.on;
        *self
    }
    fn switch_immutable(&self) -> Self {
        Self {
            on: !self.on,
            ..*self
        }
    }
}

// Предоставлять данные о текущей потребляемой мощности
trait GetPower {
    fn get_power(&self) -> String;
}

impl<I, P: ToString> GetPower for SmartOutlet<I, P> {
    fn get_power(&self) -> String {
        self.power.to_string()
    }
}

// Термометр
struct Thermometer<T> {
    id: Uuid,
    temp: T,
}

// Выдавать данные о текущей температуре
trait GetTemp {
    fn get_temp(&self) -> String;
}

impl<T: ToString> GetTemp for Thermometer<T> {
    fn get_temp(&self) -> String {
        self.temp.to_string()
    }
}

fn main() {
    todo!()
}
