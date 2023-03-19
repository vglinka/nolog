// Copyright (c) 2022-present Vadim Glinka
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option.

#![allow(unused_variables)]

// use `cargo run --features trace`

#[macro_use]
extern crate nolog;

type Fauna = Vec<Box<dyn Creature>>;

struct Planet<'a> {
    name: &'a str,
    fauna: Fauna,
}

struct Fish<'a> {
    name: &'a str,
    repr: &'a str,
    speech: &'a str,
}

struct Rabbit<'a> {
    name: &'a str,
    repr: &'a str,
    speech: &'a str,

    #[allow(dead_code)]
    reincarnations: u128,
}

trait Creature {
    fn new() -> Self
    where
        Self: Sized;
    fn repr(&self) -> String;
    fn speech(&self) -> String;
    fn name(&self) -> String;
}

impl Creature for Fish<'_> {
    fn new() -> Self {
        Self {
            name: "Fish",
            repr: "><>",
            speech: "Yohooo-ho!",
        }
    }
    fn repr(&self) -> String {
        self.repr.into()
    }
    fn speech(&self) -> String {
        self.speech.into()
    }
    fn name(&self) -> String {
        self.name.into()
    }
}

impl Creature for Rabbit<'_> {
    fn new() -> Self {
        Self {
            name: "Rabbit",
            repr: ">o.o<",
            speech: "Mmmm!",
            reincarnations: Default::default(),
        }
    }
    fn repr(&self) -> String {
        self.repr.into()
    }
    fn speech(&self) -> String {
        self.speech.into()
    }
    fn name(&self) -> String {
        self.name.into()
    }
}

impl Drop for Planet<'_> {
    fn drop(&mut self) {
        crit!("The planet {} has been destroyed.", self.name);
    }
}

impl<'a> Planet<'a> {
    fn new(name: &'a str) -> Self {
        crit!(r#"The planet "{name}" has been created."#);
        trace!("{name}" => "Am I a planet?! Oh no...");
        Self {
            name,
            fauna: Default::default(),
        }
    }

    fn get_answer(&self) -> u32 {
        let name = self.name;
        debug!("{name}" => "{}?", 2*7);
        info!("{name}" => "{}?", 3*7);
        warn!("{name}" => "{}.. maybe..", 5*7);
        debug!(
                   "Planet {name} thinks...";
             ->[2] "Planet {name} thinks...";
        );
        error!("{name}" => "{}!! Oh, yeaaaah!", 2*3*7);
        crit!(->[8] "{name}" => "Where is everyone?..");
        2 * 3 * 7
    }

    fn push(&mut self, creature: impl Creature + 'static) {
        let repr = creature.repr();
        let speech = creature.speech();
        let name = creature.name();
        let planet_name = self.name;
        info!(->[12] "New {name} on planet {planet_name}.");
        debug!(->[12] "{repr}" => "{name} says: {speech}");
        self.fauna.push(Box::new(creature));
    }
}

fn main() {

    let (server, ip) = ("localhost", "127.0.0.1");
    let server_check_result = "Ok";
    
    debug!(->[_,1] "The simulation server started successfully.");
    debug!(
        "{server}" =>       "{ip}";
        "Status"   => ->[3] "{server_check_result}";
    );
    
    {
        newline!();
        let universe = [0; 3];
        crit!("The Universe was created with a lifetime of {} days.", universe.len());
        let planet_name = "Novus";
        let mut planet = Planet::new(planet_name);
        info!(r#"The calculation of the "Answer to the Ultimate Question" begins."#);
        let answer = planet.get_answer();
        crit!(->[_,1,1] "The answer is {answer}.");
        warn!("Planet {planet_name} started watching TV.");
        error!("Uncontrolled evolutionary processes have begun on the planet {planet_name}.");
        for day in 1..(universe.len() + 1) {
            if day == universe.len() {
                warn!(->[6] "Today is day {day}, the last day of the Universe.");
                for _ in 0..2 {
                    planet.push(Rabbit::new());
                }
            } else {
                trace!(->[6] "Today is day {day}");
                planet.push(Fish::new());
            }
        }
        let mut population = vec![];
        for creature in &planet.fauna {
            population.push(creature.repr());
        }
        info!("The population of the planet {planet_name}: {population:?}.");
    }
    crit!("The End of the Universe.");
}
