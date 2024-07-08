use crate::config::Quote;
use anyhow::{ensure, Context, Result};
use rand::Rng;
use rand_distr::WeightedIndex;

struct NormalizedQuote<'a> {
    weight: f64,
    content: &'a str,
}

pub struct Selector;

impl Selector {
    pub fn select(quotes: &Vec<Quote>) -> Result<&str> {
        let normalized = Self::normalize(quotes)?;
        let weights = normalized.iter().map(|quote| quote.weight);
        let distribution = WeightedIndex::new(weights).context("unexpected error")?;
        let index = rand::thread_rng().sample(distribution);
        Ok(normalized[index].content)
    }

    fn normalize(quotes: &Vec<Quote>) -> Result<Vec<NormalizedQuote<'_>>> {
        let mut normalized = Vec::with_capacity(quotes.len());
        let mut unweighted = Vec::new();
        let mut total_weight = 0.0;

        for quote in quotes {
            match quote.weight {
                None => {
                    unweighted.push(quote);
                }
                Some(weight) => {
                    if weight == 0.0
                        || weight.is_nan()
                        || weight.is_infinite()
                        || weight.is_sign_negative()
                    {
                        continue;
                    }
                    total_weight += weight;
                    normalized.push(NormalizedQuote {
                        weight,
                        content: &quote.content,
                    });
                }
            }
        }

        ensure!(!normalized.is_empty(), "no valid quotes found");

        if !unweighted.is_empty() {
            let average_weight = total_weight / normalized.len() as f64;
            for quote in unweighted {
                total_weight += average_weight;
                normalized.push(NormalizedQuote {
                    weight: average_weight,
                    content: &quote.content,
                });
            }
        }

        ensure!(total_weight.is_finite(), "total weight overflows");

        Ok(normalized)
    }
}
