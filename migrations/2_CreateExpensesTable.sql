CREATE TABLE expenses (
    id INTEGER PRIMARY KEY,
    split_id TEXT NOT NULL,
    name TEXT NOT NULL,
    amount REAL NOT NULL,
    currency TEXT NOT NULL,
    original_amount REAL NOT NULL,
    original_currency TEXT NOT NULL,
    payed_by TEXT NOT NULL,
    payed_for TEXT NOT NULL,
    expense_date REAL NOT NULL,
    split_method TEXT NOT NULL
);
