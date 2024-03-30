use chia_bls::Signature;
use chia_protocol::{Bytes32, Coin, CoinSpend, Program, SpendBundle};
use sqlx::{Acquire, Result, Sqlite};

/// A SQLite implementation of a transaction store. Uses the tables `transactions` and `coin_spends`.
#[derive(Debug, Clone)]
pub struct SqliteTransactionStore<T> {
    db: T,
}

impl<'a, T> SqliteTransactionStore<T>
where
    for<'b> &'b T: Acquire<'a, Database = Sqlite>,
{
    /// Create a new `TransactionStore` from a connection pool.
    pub fn new(db: T) -> Self {
        Self { db }
    }

    /// Get all spent coins from the store.
    pub async fn spent_coins(&self) -> Result<Vec<Coin>> {
        let mut conn = self.db.acquire().await?;

        Ok(sqlx::query!(
            "
            SELECT `parent_coin_id`, `puzzle_hash`, `amount`
            FROM `coin_spends` ORDER BY `coin_id` ASC
            "
        )
        .fetch_all(&mut *conn)
        .await?
        .into_iter()
        .map(|record| {
            let parent_coin_id: [u8; 32] = record.parent_coin_id.try_into().unwrap();
            let puzzle_hash: [u8; 32] = record.puzzle_hash.try_into().unwrap();

            Coin {
                parent_coin_info: parent_coin_id.into(),
                puzzle_hash: puzzle_hash.into(),
                amount: record.amount as u64,
            }
        })
        .collect())
    }

    /// Get a list of all transactions in the store.
    pub async fn transactions(&self) -> Result<Vec<Bytes32>> {
        let mut conn = self.db.acquire().await?;

        Ok(
            sqlx::query!(
                "SELECT `transaction_id` AS `transaction_id: Vec<u8>` FROM `transactions`"
            )
            .fetch_all(&mut *conn)
            .await?
            .into_iter()
            .map(|row| row.transaction_id.try_into().unwrap())
            .collect(),
        )
    }

    /// Get a transaction by its id.
    pub async fn transaction(&self, transaction_id: Bytes32) -> Result<Option<SpendBundle>> {
        let mut tx = self.db.begin().await?;
        let transaction_id = transaction_id.to_vec();
        let spend_transaction_id = transaction_id.clone();

        let Some(transaction) = sqlx::query!(
            "
            SELECT
                `aggregated_signature` AS `aggregated_signature: Vec<u8>`
            FROM `transactions` WHERE `transaction_id` = ?
            ",
            transaction_id
        )
        .fetch_optional(&mut *tx)
        .await?
        else {
            return Ok(None);
        };

        let coin_spends = sqlx::query!(
            "
            SELECT `coin_id`, `parent_coin_id`, `puzzle_hash`, `amount`,
                   `puzzle_reveal`, `solution`, `transaction_id`
            FROM `coin_spends` WHERE `transaction_id` = ?
            ",
            spend_transaction_id
        )
        .fetch_all(&mut *tx)
        .await?
        .into_iter()
        .map(|row| {
            let parent_coin_id: [u8; 32] = row.parent_coin_id.try_into().unwrap();
            let puzzle_hash: [u8; 32] = row.puzzle_hash.try_into().unwrap();

            CoinSpend {
                coin: Coin {
                    parent_coin_info: parent_coin_id.into(),
                    puzzle_hash: puzzle_hash.into(),
                    amount: row.amount as u64,
                },
                puzzle_reveal: Program::from(row.puzzle_reveal),
                solution: Program::from(row.solution),
            }
        })
        .collect();

        tx.commit().await?;

        let signature: [u8; 96] = transaction.aggregated_signature.try_into().unwrap();
        Ok(Some(SpendBundle::new(
            coin_spends,
            Signature::from_bytes(&signature).unwrap(),
        )))
    }

    /// Get the coins spent by a transaction.
    pub async fn removals(&self, transaction_id: Bytes32) -> Result<Vec<Coin>> {
        let mut conn = self.db.acquire().await?;
        let transaction_id = transaction_id.to_vec();

        Ok(sqlx::query!(
            "
            SELECT `parent_coin_id`, `puzzle_hash`, `amount`
            FROM `coin_spends` WHERE `transaction_id` = ?
            ORDER BY `coin_id` ASC
            ",
            transaction_id
        )
        .fetch_all(&mut *conn)
        .await?
        .into_iter()
        .map(|record| {
            let parent_coin_id: [u8; 32] = record.parent_coin_id.try_into().unwrap();
            let puzzle_hash: [u8; 32] = record.puzzle_hash.try_into().unwrap();

            Coin {
                parent_coin_info: parent_coin_id.into(),
                puzzle_hash: puzzle_hash.into(),
                amount: record.amount as u64,
            }
        })
        .collect())
    }

    /// Add a transaction to the store.
    pub async fn add_transaction(&self, spend_bundle: SpendBundle) -> Result<()> {
        let mut tx = self.db.begin().await?;
        let transaction_id = spend_bundle.name().to_vec();
        let add_transaction_id = transaction_id.clone();
        let aggregated_signature = spend_bundle.aggregated_signature.to_bytes().to_vec();

        sqlx::query!(
            "
            REPLACE INTO `transactions` (
                `transaction_id`,
                `aggregated_signature`
            )
            VALUES (?, ?)
            ",
            add_transaction_id,
            aggregated_signature
        )
        .execute(&mut *tx)
        .await?;

        for coin_spend in spend_bundle.coin_spends {
            let coin_id = coin_spend.coin.coin_id().to_vec();
            let parent_coin_id = coin_spend.coin.parent_coin_info.to_vec();
            let puzzle_hash = coin_spend.coin.puzzle_hash.to_vec();
            let amount = coin_spend.coin.amount as i64;
            let puzzle_reveal = coin_spend.puzzle_reveal.as_ref().to_vec();
            let solution = coin_spend.solution.as_ref().to_vec();
            let transaction_id = transaction_id.clone();

            sqlx::query!(
                "
                REPLACE INTO `coin_spends` (
                    `coin_id`,
                    `parent_coin_id`,
                    `puzzle_hash`,
                    `amount`,
                    `puzzle_reveal`,
                    `solution`,
                    `transaction_id`
                )
                VALUES (?, ?, ?, ?, ?, ?, ?)
                ",
                coin_id,
                parent_coin_id,
                puzzle_hash,
                amount,
                puzzle_reveal,
                solution,
                transaction_id
            )
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await
    }

    /// Remove a transaction from the store.
    pub async fn remove_transaction(&self, transaction_id: Bytes32) -> Result<()> {
        let mut conn = self.db.acquire().await?;
        let transaction_id = transaction_id.to_vec();

        sqlx::query!(
            "DELETE FROM `transactions` WHERE `transaction_id` = ?",
            transaction_id
        )
        .execute(&mut *conn)
        .await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use sqlx::SqlitePool;

    use super::*;

    #[sqlx::test]
    async fn test_transaction_store(pool: SqlitePool) {
        let store = SqliteTransactionStore::new(pool.clone());

        // Add a transaction.
        let coin = Coin {
            parent_coin_info: Bytes32::default(),
            puzzle_hash: Bytes32::default(),
            amount: 100,
        };

        let puzzle_reveal = Program::default();
        let solution = Program::default();

        let coin_spend = CoinSpend {
            coin: coin.clone(),
            puzzle_reveal,
            solution,
        };

        let spend_bundle = SpendBundle::new(vec![coin_spend], Signature::default());

        let transaction_id = spend_bundle.name();

        store.add_transaction(spend_bundle.clone()).await.unwrap();

        // Get the transaction and compare.
        let transaction = store
            .transaction(transaction_id)
            .await
            .unwrap()
            .expect("no spend bundle");
        assert_eq!(transaction, spend_bundle);

        // Get the removals and compare.
        let removals = store.removals(transaction_id).await.unwrap();
        assert_eq!(removals, vec![coin.clone()]);

        // Get all spent coins and make sure the coin is there.
        let spent_coins = store.spent_coins().await.unwrap();
        assert_eq!(spent_coins, vec![coin]);

        // Remove the transaction.
        store.remove_transaction(transaction_id).await.unwrap();

        // Make sure the transaction is gone.
        assert!(store.transaction(transaction_id).await.unwrap().is_none());
    }
}
