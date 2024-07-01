use std::collections::HashMap;

use crate::{positions::CrossMarginActivePosition, CrossMarginAccount, CrossMarginPositionSide};

pub struct MarginCalculationDto {
    pub leverage: f64,
    pub lots_amount: f64,
    pub contract_size: f64,
    pub margin_rate: f64,
    pub side: CrossMarginPositionSide,
}

pub fn calculate_margin(
    account: &impl CrossMarginAccount,
    positions: &Vec<&impl CrossMarginActivePosition>,
) -> f64 {
    let mut grouped_positions = HashMap::new();
    let mut margin = 0.0;

    for position in positions {
        grouped_positions
            .entry(position.get_instrument_id())
            .or_insert(Vec::new())
            .push(position);
    }

    for (instrument, positions) in grouped_positions {
        margin += calculate_specific_instrument_margin(
            &positions,
            account.get_leverage(),
            account.get_instruments_leverages().get(instrument),
        )
        ;
    }

    return margin;
}

fn calculate_specific_instrument_margin(
    positions: &Vec<&&impl CrossMarginActivePosition>,
    account_leverage: f64,
    instrument_leverage: Option<&f64>,
) -> f64 {
    let mut buy_lots_amount = 0.0;
    let mut sell_lots_amount = 0.0;

    for position in positions {
        match position.get_side() {
            &CrossMarginPositionSide::Buy => buy_lots_amount += position.get_lots_amount(),
            &CrossMarginPositionSide::Sell => sell_lots_amount += position.get_lots_amount(),
        }
    }

    let leverage = match instrument_leverage {
        Some(x) => x.min(account_leverage),
        None => account_leverage,
    };

    let is_hedge = buy_lots_amount > 0.0 && sell_lots_amount > 0.0;

    if !is_hedge {
        return positions
            .iter()
            .map(|x| x.get_lots_size() * x.get_lots_amount() / leverage * x.get_margin_price())
            .sum();
    }

    let mut buy_hedge_amount = buy_lots_amount.min(sell_lots_amount);
    let mut sell_hedge_amount = buy_hedge_amount.clone();

    let mut hedged_positions = vec![];
    let mut not_hedged_positions = vec![];

    for position in positions {
        match position.get_side() {
            &CrossMarginPositionSide::Buy => {
                if buy_hedge_amount < position.get_lots_amount() {
                    hedged_positions.push(MarginCalculationDto {
                        leverage: leverage,
                        lots_amount: buy_hedge_amount,
                        contract_size: position.get_lots_size(),
                        margin_rate: position.get_margin_price(),
                        side: position.get_side().clone(),
                    });
                    not_hedged_positions.push(MarginCalculationDto {
                        leverage: leverage,
                        lots_amount: position.get_lots_amount() - buy_hedge_amount,
                        contract_size: position.get_lots_size(),
                        margin_rate: position.get_margin_price(),
                        side: position.get_side().clone(),
                    });
                    buy_hedge_amount = 0.0;
                    continue;
                }

                buy_hedge_amount = buy_hedge_amount - position.get_lots_amount();
                hedged_positions.push(MarginCalculationDto {
                    leverage: leverage,
                    lots_amount: position.get_lots_amount(),
                    contract_size: position.get_lots_size(),
                    margin_rate: position.get_margin_price(),
                    side: position.get_side().clone(),
                });
            }
            &CrossMarginPositionSide::Sell => {
                if sell_hedge_amount < position.get_lots_amount() {
                    hedged_positions.push(MarginCalculationDto {
                        leverage,
                        lots_amount: sell_hedge_amount,
                        contract_size: position.get_lots_size(),
                        margin_rate: position.get_margin_price(),
                        side: position.get_side().clone(),
                    });
                    not_hedged_positions.push(MarginCalculationDto {
                        leverage,
                        lots_amount: position.get_lots_amount() - sell_hedge_amount,
                        contract_size: position.get_lots_size(),
                        margin_rate: position.get_margin_price(),
                        side: position.get_side().clone(),
                    });
                    sell_hedge_amount = 0.0;
                    continue;
                }

                sell_hedge_amount = sell_hedge_amount - position.get_lots_amount();
                hedged_positions.push(MarginCalculationDto {
                    leverage: leverage,
                    lots_amount: position.get_lots_amount(),
                    contract_size: position.get_lots_size(),
                    margin_rate: position.get_margin_price(),
                    side: position.get_side().clone(),
                });
            }
        }
    }

    let hedged_margin = calculate_hedged_margin(&hedged_positions);
    let not_hedged_margin = not_hedged_positions
        .iter()
        .map(|x| x.contract_size * x.lots_amount / x.leverage * x.margin_rate)
        .sum::<f64>();

    return hedged_margin + not_hedged_margin;
}

fn calculate_hedged_margin(positions: &Vec<MarginCalculationDto>) -> f64 {
    let avg_margin_rate =
        positions.iter().map(|x| x.margin_rate).sum::<f64>() / positions.len() as f64;

    return positions
        .iter()
        .map(|x| x.contract_size * x.lots_amount / x.leverage * avg_margin_rate)
        .sum::<f64>()
        / positions.len() as f64;
}
