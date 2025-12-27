use std::{
    io::{Error, Stdin, Write, stdin},
    ops::RangeInclusive,
};

use whist::game::hand::{InputError, Requester};

pub struct CliRequest {
    reader: Stdin,
}

impl CliRequest {
    pub fn new() -> Self {
        let reader = stdin();
        Self { reader }
    }

    pub fn ask(&self, text: &str) -> Result<String, Error> {
        let mut buffer = String::new();
        print!("{text}");
        std::io::stdout().flush()?;
        self.reader.read_line(&mut buffer)?;
        Ok(buffer)
    }
}

#[async_trait::async_trait]
impl Requester for CliRequest {
    async fn ask_bid(&self, range: RangeInclusive<i16>) -> Result<i16, InputError> {
        let bid = self
            .ask(&format!("Bid inside {}-{}: ", range.start(), range.end()))
            .unwrap();
        Ok(bid
            .trim()
            .parse::<i16>()
            .map_err(|e| InputError::InvalidInput(e.to_string()))?)
    }

    async fn ask_name(&self) -> Result<String, InputError> {
        Ok(self.ask("New player name: ").unwrap().trim().to_string())
    }

    async fn pick_names(
        &self,
        names_list: Vec<String>,
        names_number: usize,
    ) -> Result<Vec<String>, InputError> {
        let name_and_idx: Vec<String> = names_list
            .iter()
            .enumerate()
            .map(|(i, n)| format!("{n}:{i}"))
            .collect();
        let mut out = Vec::with_capacity(names_number);
        for _ in 0..names_number {
            let idx = self
                .ask(&format!("Pick names using their index: {name_and_idx:?}"))
                .unwrap();
            let idx: usize = idx.trim().parse().unwrap();
            out.push(names_list[idx].clone());
        }
        Ok(out)
    }
}
