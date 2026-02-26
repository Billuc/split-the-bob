use crate::expenses::expense::{self, Expense};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Balance {
    pub debtor: String,
    pub amount: f32,
    pub currency: String,
    pub creditor: String,
}

pub fn balances_from_expenses(expenses: &Vec<Expense>, default_currency: String) -> (Vec<Balance>, HashMap<String, f32>) {
    let individual_balances = get_individual_balances(expenses);
    let (debts, credits) = get_credits_and_debts(&individual_balances);
    let balances = calculate_balances(debts, credits, default_currency);

    (balances, individual_balances)
}

fn get_individual_balances(expenses: &Vec<Expense>) -> HashMap<String, f32> {
    let mut balance_per_participant: HashMap<String, f32> = HashMap::new();

    for expense in expenses {
        for (participant, amount) in split_expense(&expense) {
            let participant_balance = balance_per_participant.entry(participant).or_insert(0.0);
            *participant_balance += amount;
        }
    }

    balance_per_participant
}

fn get_credits_and_debts(balance_per_participant: &HashMap<String, f32>) -> (Vec<(String, f32)>, Vec<(String, f32)>) {
    let mut debts: Vec<(String, f32)> = Vec::new();
    let mut credits: Vec<(String, f32)> = Vec::new();

    for (participant, balance) in balance_per_participant {
        if *balance < 0.0 {
            credits.push((participant.clone(), -(*balance)));
        } else if *balance > 0.0 {
            debts.push((participant.clone(), *balance));
        }
    }

    debts.sort_by(|a, b| b.1.total_cmp(&a.1));
    credits.sort_by(|a, b| b.1.total_cmp(&a.1));

    (debts, credits)
}

fn split_expense(expense: &Expense) -> impl Iterator<Item = (String, f32)> {
    match expense.split_method {
        expense::SplitMethod::Evenly => split_evenly(
            expense.amount,
            expense.payed_by.clone(),
            expense.payed_for.clone(),
        ),
    }
}

fn split_evenly(
    amount: f32,
    payer: String,
    participants: Vec<String>,
) -> impl Iterator<Item = (String, f32)> {
    let amount_per_person = amount / participants.len() as f32;

    participants.into_iter().map(move |p| {
        (p.clone(), amount_per_person)
    }).chain(vec![(payer.clone(), -amount)])
}

fn calculate_balances(
    debts: Vec<(String, f32)>,
    credits: Vec<(String, f32)>,
    currency: String,
) -> Vec<Balance> {
    let mut balances: Vec<Balance> = Vec::new();
    let mut credits = credits.clone();

    for (debtor, debt_amount) in debts {
        let mut remaining_debt = debt_amount;

        for (creditor, credit_amount) in &mut credits {
            if remaining_debt <= 0.0 {
                break;
            }

            let amount_to_pay = remaining_debt.min(*credit_amount);
            if amount_to_pay > 0.0 {
                balances.push(Balance {
                    debtor: debtor.clone(),
                    amount: amount_to_pay,
                    currency: currency.clone(),
                    creditor: creditor.clone(),
                });

                remaining_debt -= amount_to_pay;
                *credit_amount -= amount_to_pay;
            }
        }
    }

    return balances;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expenses::expense::{Expense, SplitMethod};
    use std::time::SystemTime;

    fn create_expense(
        id: i64,
        split_id: &str,
        name: &str,
        amount: f32,
        currency: &str,
        payed_by: &str,
        payed_for: Vec<&str>,
    ) -> Expense {
        Expense {
            id,
            split_id: split_id.to_string(),
            name: name.to_string(),
            amount,
            currency: currency.to_string(),
            original_amount: amount,
            original_currency: currency.to_string(),
            payed_by: payed_by.to_string(),
            payed_for: payed_for.iter().map(|s| s.to_string()).collect(),
            expense_date: SystemTime::now(),
            split_method: SplitMethod::Evenly,
        }
    }

    fn find_balance<'a>(
        balances: &'a [Balance],
        debtor: &str,
        creditor: &str,
    ) -> Option<&'a Balance> {
        balances
            .iter()
            .find(|b| b.debtor == debtor && b.creditor == creditor)
    }

    #[test]
    fn test_single_expense_two_people() {
        let expenses = vec![create_expense(
            1,
            "split1",
            "Dinner",
            100.0,
            "EUR",
            "Alice",
            vec!["Alice", "Bob"],
        )];

        let balances = balances_from_expenses(&expenses, "EUR".to_string());

        assert_eq!(balances.len(), 1);
        let balance = &balances[0];
        assert_eq!(balance.debtor, "Bob");
        assert_eq!(balance.creditor, "Alice");
        assert_eq!(balance.amount, 50.0);
        assert_eq!(balance.currency, "EUR");
    }

    #[test]
    fn test_single_expense_three_people() {
        let expenses = vec![create_expense(
            1,
            "split1",
            "Lunch",
            90.0,
            "USD",
            "Alice",
            vec!["Alice", "Bob", "Charlie"],
        )];

        let (balances, _) = balances_from_expenses(&expenses, "USD".to_string());

        assert_eq!(balances.len(), 2);

        // Bob and Charlie each owe Alice 30.0
        let bob_balance = find_balance(&balances, "Bob", "Alice");
        assert!(bob_balance.is_some());
        assert_eq!(bob_balance.unwrap().amount, 30.0);

        let charlie_balance = find_balance(&balances, "Charlie", "Alice");
        assert!(charlie_balance.is_some());
        assert_eq!(charlie_balance.unwrap().amount, 30.0);
    }

    #[test]
    fn test_two_expenses_same_payer() {
        let expenses = vec![
            create_expense(
                1,
                "split1",
                "Dinner",
                60.0,
                "EUR",
                "Alice",
                vec!["Alice", "Bob"],
            ),
            create_expense(
                2,
                "split1",
                "Drinks",
                40.0,
                "EUR",
                "Alice",
                vec!["Alice", "Bob"],
            ),
        ];

        let balances = balances_from_expenses(&expenses, "EUR".to_string());

        assert_eq!(balances.len(), 1);
        let balance = &balances[0];
        assert_eq!(balance.debtor, "Bob");
        assert_eq!(balance.creditor, "Alice");
        assert_eq!(balance.amount, 50.0); // (60/2 + 40/2)
    }

    #[test]
    fn test_two_expenses_different_payers() {
        let expenses = vec![
            create_expense(
                1,
                "split1",
                "Dinner",
                100.0,
                "EUR",
                "Alice",
                vec!["Alice", "Bob"],
            ),
            create_expense(
                2,
                "split1",
                "Drinks",
                60.0,
                "EUR",
                "Bob",
                vec!["Alice", "Bob"],
            ),
        ];

        let balances = balances_from_expenses(&expenses, "EUR".to_string());

        // Alice paid 100, owes 50 (her share of 100) + 30 (her share of 60) = 80 total owed to group
        // Bob paid 60, owes 50 (his share of 100) + 30 (his share of 60) = 80 total owed to group
        // Actually: Alice net = -50 + 30 = -20 (creditor)
        // Bob net = 50 - 30 = 20 (debtor)
        assert_eq!(balances.len(), 1);
        let balance = &balances[0];
        assert_eq!(balance.debtor, "Bob");
        assert_eq!(balance.creditor, "Alice");
        assert_eq!(balance.amount, 20.0);
    }

    #[test]
    fn test_someone_pays_only_for_themselves() {
        let expenses = vec![
            create_expense(
                1,
                "split1",
                "Group Dinner",
                90.0,
                "EUR",
                "Alice",
                vec!["Alice", "Bob", "Charlie"],
            ),
            create_expense(
                2,
                "split1",
                "Personal Coffee",
                5.0,
                "EUR",
                "Bob",
                vec!["Bob"],
            ),
        ];

        let (balances, _) = balances_from_expenses(&expenses, "EUR".to_string());

        // Alice paid 90, owes 30 (her share) -> net = -60 (creditor)
        // Bob paid 5, owes 30 (from group dinner) + 5 (coffee) = 35 -> net = 30 (debtor)
        // Charlie paid 0, owes 30 -> net = 30 (debtor)
        assert_eq!(balances.len(), 2);

        let bob_balance = find_balance(&balances, "Bob", "Alice");
        assert!(bob_balance.is_some());
        assert_eq!(bob_balance.unwrap().amount, 30.0);

        let charlie_balance = find_balance(&balances, "Charlie", "Alice");
        assert!(charlie_balance.is_some());
        assert_eq!(charlie_balance.unwrap().amount, 30.0);
    }

    #[test]
    fn test_no_debt_when_balanced() {
        let expenses = vec![
            create_expense(
                1,
                "split1",
                "Dinner",
                100.0,
                "EUR",
                "Alice",
                vec!["Alice", "Bob"],
            ),
            create_expense(
                2,
                "split1",
                "Lunch",
                100.0,
                "EUR",
                "Bob",
                vec!["Alice", "Bob"],
            ),
        ];

        let balances = balances_from_expenses(&expenses, "EUR".to_string());

        // Alice paid 100, owes 100 -> net = 0
        // Bob paid 100, owes 100 -> net = 0
        assert_eq!(balances.len(), 0);
    }

    #[test]
    fn test_complex_four_people_multiple_expenses() {
        let expenses = vec![
            create_expense(
                1,
                "split1",
                "Hotel",
                400.0,
                "EUR",
                "Alice",
                vec!["Alice", "Bob", "Charlie", "Diana"],
            ),
            create_expense(
                2,
                "split1",
                "Dinner",
                120.0,
                "EUR",
                "Bob",
                vec!["Alice", "Bob", "Charlie", "Diana"],
            ),
            create_expense(
                3,
                "split1",
                "Breakfast",
                80.0,
                "EUR",
                "Charlie",
                vec!["Alice", "Bob", "Charlie", "Diana"],
            ),
            create_expense(
                4,
                "split1",
                "Taxi",
                60.0,
                "EUR",
                "Diana",
                vec!["Alice", "Bob", "Charlie"],
            ),
        ];

        let (balances, _) = balances_from_expenses(&expenses, "EUR".to_string());

        // Alice: paid 400, owes (100 + 30 + 20 + 20) = 170 -> net = -230 (creditor)
        // Bob: paid 120, owes (100 + 30 + 20 + 20) = 170 -> net = 50 (debtor)
        // Charlie: paid 80, owes (100 + 30 + 20 + 20) = 170 -> net = 90 (debtor)
        // Diana: paid 60, owes (100 + 30 + 20 + 0) = 150 -> net = 90 (debtor)
        
        // Total debts: 50 + 90 + 90 = 230 (matches Alice's credit)
        assert!(balances.len() == 3);
        
        let bob_balance = find_balance(&balances, "Bob", "Alice");
        assert!(bob_balance.is_some());
        assert_eq!(bob_balance.unwrap().amount, 50.0);

        let charlie_balance = find_balance(&balances, "Charlie", "Alice");
        assert!(charlie_balance.is_some());
        assert_eq!(charlie_balance.unwrap().amount, 90.0);

        let diana_balance = find_balance(&balances, "Diana", "Alice");
        assert!(diana_balance.is_some());
        assert_eq!(diana_balance.unwrap().amount, 90.0);
    }

    #[test]
    fn test_complex_five_people_six_expenses() {
        let expenses = vec![
            create_expense(
                1,
                "trip",
                "Flight",
                500.0,
                "USD",
                "Alice",
                vec!["Alice", "Bob", "Charlie", "Diana", "Eve"],
            ),
            create_expense(
                2,
                "trip",
                "Hotel",
                325.0,
                "USD",
                "Charlie",
                vec!["Alice", "Bob", "Charlie", "Diana", "Eve"],
            ),
            create_expense(
                3,
                "trip",
                "Dinner",
                100.0,
                "USD",
                "Bob",
                vec!["Alice", "Bob", "Charlie", "Diana", "Eve"],
            ),
            create_expense(
                4,
                "trip",
                "Breakfast",
                75.0,
                "USD",
                "Diana",
                vec!["Alice", "Bob", "Charlie", "Diana", "Eve"],
            ),
            create_expense(
                5,
                "trip",
                "Taxi",
                50.0,
                "USD",
                "Eve",
                vec!["Alice", "Bob", "Charlie", "Diana", "Eve"],
            ),
        ];

        let (balances, _) = balances_from_expenses(&expenses, "USD".to_string());

        // Total expenses: 500 + 325 + 100 + 75 + 50 = 1050
        // Each person owes: 1050 / 5 = 210
        // Alice: paid 500, owes 210 -> net = -290 (creditor)
        // Charlie: paid 325, owes 210 -> net = -115 (creditor)
        // Bob: paid 100, owes 210 -> net = 110 (debtor)
        // Diana: paid 75, owes 210 -> net = 135 (debtor)
        // Eve: paid 50, owes 210 -> net = 160 (debtor)
        
        // Expected balances (sorted by largest debts first):
        // Eve (160) owes Alice (290): pays 160 to Alice
        // Diana (135) owes Alice (130 remaining): pays 130 to Alice, 5 remains
        // Diana (5 remaining) owes Charlie (115): pays 5 to Charlie
        // Bob (110) owes Charlie (110 remaining): pays 110 to Charlie
        
        assert_eq!(balances.len(), 4);
        
        let eve_balance = find_balance(&balances, "Eve", "Alice");
        assert!(eve_balance.is_some(), "Eve should owe Alice");
        assert_eq!(eve_balance.unwrap().amount, 160.0);
        assert_eq!(eve_balance.unwrap().currency, "USD");

        let diana_alice_balance = find_balance(&balances, "Diana", "Alice");
        assert!(diana_alice_balance.is_some(), "Diana should owe Alice");
        assert_eq!(diana_alice_balance.unwrap().amount, 130.0);
        assert_eq!(diana_alice_balance.unwrap().currency, "USD");

        let diana_charlie_balance = find_balance(&balances, "Diana", "Charlie");
        assert!(diana_charlie_balance.is_some(), "Diana should owe Charlie");
        assert_eq!(diana_charlie_balance.unwrap().amount, 5.0);
        assert_eq!(diana_charlie_balance.unwrap().currency, "USD");

        let bob_balance = find_balance(&balances, "Bob", "Charlie");
        assert!(bob_balance.is_some(), "Bob should owe Charlie");
        assert_eq!(bob_balance.unwrap().amount, 110.0);
        assert_eq!(bob_balance.unwrap().currency, "USD");
    }

    #[test]
    fn test_edge_case_empty_expenses() {
        let expenses: Vec<Expense> = vec![];
        let balances = balances_from_expenses(&expenses, "EUR".to_string());
        assert_eq!(balances.len(), 0);
    }

    #[test]
    fn test_edge_case_single_person_expense() {
        let expenses = vec![create_expense(
            1,
            "split1",
            "Solo Lunch",
            20.0,
            "EUR",
            "Alice",
            vec!["Alice"],
        )];

        let balances = balances_from_expenses(&expenses, "EUR".to_string());

        // Alice pays for herself, no debt
        assert_eq!(balances.len(), 0);
    }
}
