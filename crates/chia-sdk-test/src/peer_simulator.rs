use std::{net::SocketAddr, sync::Arc};

use chia_protocol::{Bytes32, Coin, CoinState, Message};
use chia_sdk_client::{Peer, PeerOptions};
use error::PeerSimulatorError;
use peer_map::PeerMap;
use simulator_config::SimulatorConfig;
use subscriptions::Subscriptions;
use tokio::{
    net::TcpListener,
    sync::{mpsc, Mutex},
    task::JoinHandle,
};
use tokio_tungstenite::connect_async;
use ws_connection::ws_connection;

use crate::Simulator;

mod error;
mod peer_map;
mod simulator_config;
mod subscriptions;
mod ws_connection;

#[derive(Debug)]
pub struct PeerSimulator {
    config: Arc<SimulatorConfig>,
    addr: SocketAddr,
    simulator: Arc<Mutex<Simulator>>,
    subscriptions: Arc<Mutex<Subscriptions>>,
    join_handle: JoinHandle<()>,
}

impl PeerSimulator {
    pub async fn new() -> Result<Self, PeerSimulatorError> {
        Self::with_config(SimulatorConfig::default()).await
    }

    pub async fn with_config(config: SimulatorConfig) -> Result<Self, PeerSimulatorError> {
        log::info!("starting simulator");

        let addr = "127.0.0.1:0";
        let peer_map = PeerMap::default();
        let listener = TcpListener::bind(addr).await?;
        let addr = listener.local_addr()?;
        let simulator = Arc::new(Mutex::new(Simulator::default()));
        let subscriptions = Arc::new(Mutex::new(Subscriptions::default()));
        let config = Arc::new(config);

        let simulator_clone = simulator.clone();
        let subscriptions_clone = subscriptions.clone();
        let config_clone = config.clone();

        let join_handle = tokio::spawn(async move {
            let simulator = simulator_clone;
            let subscriptions = subscriptions_clone;
            let config = config_clone;

            while let Ok((stream, addr)) = listener.accept().await {
                let stream = match tokio_tungstenite::accept_async(stream).await {
                    Ok(stream) => stream,
                    Err(error) => {
                        log::error!("error accepting websocket connection: {}", error);
                        continue;
                    }
                };
                tokio::spawn(ws_connection(
                    peer_map.clone(),
                    stream,
                    addr,
                    config.clone(),
                    simulator.clone(),
                    subscriptions.clone(),
                ));
            }
        });

        Ok(Self {
            config,
            addr,
            simulator,
            subscriptions,
            join_handle,
        })
    }

    pub fn config(&self) -> &SimulatorConfig {
        &self.config
    }

    pub async fn connect_split(
        &self,
    ) -> Result<(Peer, mpsc::Receiver<Message>), PeerSimulatorError> {
        log::info!("connecting new peer to simulator");
        let (ws, _) = connect_async(format!("ws://{}", self.addr)).await?;
        Ok(Peer::from_websocket(
            ws,
            PeerOptions {
                rate_limit_factor: 0.6,
            },
        )?)
    }

    pub async fn connect(&self) -> Result<Peer, PeerSimulatorError> {
        let (peer, mut receiver) = self.connect_split().await?;

        tokio::spawn(async move {
            while let Some(message) = receiver.recv().await {
                log::debug!("received message: {message:?}");
            }
        });

        Ok(peer)
    }

    pub async fn reset(&self) -> Result<(), PeerSimulatorError> {
        *self.simulator.lock().await = Simulator::default();
        *self.subscriptions.lock().await = Subscriptions::default();
        Ok(())
    }

    pub async fn mint_coin(&self, puzzle_hash: Bytes32, amount: u64) -> Coin {
        self.simulator.lock().await.new_coin(puzzle_hash, amount)
    }

    pub async fn add_hint(&self, coin_id: Bytes32, hint: Bytes32) {
        self.simulator.lock().await.hint_coin(coin_id, hint);
    }

    pub async fn coin_state(&self, coin_id: Bytes32) -> Option<CoinState> {
        self.simulator.lock().await.coin_state(coin_id)
    }

    pub async fn height(&self) -> u32 {
        self.simulator.lock().await.height()
    }

    pub async fn header_hash(&self, height: u32) -> Bytes32 {
        self.simulator.lock().await.header_hash_of(height).unwrap()
    }

    pub async fn peak_hash(&self) -> Bytes32 {
        self.simulator.lock().await.header_hash()
    }
}

impl Drop for PeerSimulator {
    fn drop(&mut self) {
        self.join_handle.abort();
    }
}

#[cfg(test)]
mod tests {
    use chia_bls::{DerivableKey, PublicKey, Signature};
    use chia_protocol::{
        Bytes, CoinSpend, CoinStateFilters, CoinStateUpdate, RespondCoinState, RespondPuzzleState,
        SpendBundle,
    };
    use chia_sdk_types::{AggSigMe, CreateCoin, Remark};

    use crate::{coin_state_updates, test_secret_key, test_transaction, to_program, to_puzzle};

    use super::*;

    #[tokio::test]
    async fn test_coin_state() -> anyhow::Result<()> {
        let sim = PeerSimulator::new().await?;

        let coin = sim.mint_coin(Bytes32::default(), 1000).await;
        let coin_state = sim
            .coin_state(coin.coin_id())
            .await
            .expect("missing coin state");

        assert_eq!(coin_state.coin, coin);
        assert_eq!(coin_state.created_height, Some(0));
        assert_eq!(coin_state.spent_height, None);

        Ok(())
    }

    #[tokio::test]
    async fn test_empty_transaction() -> anyhow::Result<()> {
        let sim = PeerSimulator::new().await?;
        let peer = sim.connect().await?;

        let empty_bundle = SpendBundle::new(Vec::new(), Signature::default());
        let transaction_id = empty_bundle.name();

        let ack = peer.send_transaction(empty_bundle).await?;
        assert_eq!(ack.status, 3);
        assert_eq!(ack.txid, transaction_id);

        Ok(())
    }

    #[tokio::test]
    async fn test_simple_transaction() -> anyhow::Result<()> {
        let sim = PeerSimulator::new().await?;
        let peer = sim.connect().await?;

        let (puzzle_hash, puzzle_reveal) = to_puzzle(1)?;

        let coin = sim.mint_coin(puzzle_hash, 0).await;

        let spend_bundle = SpendBundle::new(
            vec![CoinSpend::new(coin, puzzle_reveal, to_program(())?)],
            Signature::default(),
        );

        let ack = peer.send_transaction(spend_bundle).await?;
        assert_eq!(ack.status, 1);

        Ok(())
    }

    #[tokio::test]
    async fn test_unknown_coin() -> anyhow::Result<()> {
        let sim = PeerSimulator::new().await?;
        let peer = sim.connect().await?;

        let (puzzle_hash, puzzle_reveal) = to_puzzle(1)?;

        let coin = Coin::new(Bytes32::default(), puzzle_hash, 0);

        let spend_bundle = SpendBundle::new(
            vec![CoinSpend::new(coin, puzzle_reveal, to_program(())?)],
            Signature::default(),
        );

        let ack = peer.send_transaction(spend_bundle).await?;
        assert_eq!(ack.status, 3);

        Ok(())
    }

    #[tokio::test]
    async fn test_bad_signature() -> anyhow::Result<()> {
        let sim = PeerSimulator::new().await?;
        let peer = sim.connect().await?;
        let public_key = test_secret_key()?.public_key();

        let (puzzle_hash, puzzle_reveal) = to_puzzle(1)?;

        let coin = sim.mint_coin(puzzle_hash, 0).await;

        let spend_bundle = SpendBundle::new(
            vec![CoinSpend::new(
                coin,
                puzzle_reveal,
                to_program([AggSigMe::new(public_key, Bytes::default())])?,
            )],
            Signature::default(),
        );

        let ack = peer.send_transaction(spend_bundle).await?;
        assert_eq!(ack.status, 3);

        Ok(())
    }

    #[tokio::test]
    async fn test_infinity_signature() -> anyhow::Result<()> {
        let sim = PeerSimulator::new().await?;
        let peer = sim.connect().await?;

        let (puzzle_hash, puzzle_reveal) = to_puzzle(1)?;

        let coin = sim.mint_coin(puzzle_hash, 0).await;

        let spend_bundle = SpendBundle::new(
            vec![CoinSpend::new(
                coin,
                puzzle_reveal,
                to_program([AggSigMe::new(PublicKey::default(), Bytes::default())])?,
            )],
            Signature::default(),
        );

        let ack = peer.send_transaction(spend_bundle).await?;
        assert_eq!(ack.status, 3);

        Ok(())
    }

    #[tokio::test]
    async fn test_valid_signature() -> anyhow::Result<()> {
        let sim = PeerSimulator::new().await?;
        let peer = sim.connect().await?;
        let sk = test_secret_key()?;
        let pk = sk.public_key();

        let (puzzle_hash, puzzle_reveal) = to_puzzle(1)?;

        let coin = sim.mint_coin(puzzle_hash, 0).await;

        test_transaction(
            &peer,
            vec![CoinSpend::new(
                coin,
                puzzle_reveal,
                to_program([AggSigMe::new(pk, b"Hello, world!".to_vec().into())])?,
            )],
            &[sk],
        )
        .await;

        Ok(())
    }

    #[tokio::test]
    async fn test_aggregated_signature() -> anyhow::Result<()> {
        let sim = PeerSimulator::new().await?;
        let peer = sim.connect().await?;

        let sk1 = test_secret_key()?.derive_unhardened(0);
        let pk1 = sk1.public_key();

        let sk2 = test_secret_key()?.derive_unhardened(1);
        let pk2 = sk2.public_key();

        let (puzzle_hash, puzzle_reveal) = to_puzzle(1)?;

        let coin = sim.mint_coin(puzzle_hash, 0).await;

        test_transaction(
            &peer,
            vec![CoinSpend::new(
                coin,
                puzzle_reveal,
                to_program([
                    AggSigMe::new(pk1, b"Hello, world!".to_vec().into()),
                    AggSigMe::new(pk2, b"Goodbye, world!".to_vec().into()),
                ])?,
            )],
            &[sk1, sk2],
        )
        .await;

        Ok(())
    }

    #[tokio::test]
    async fn test_excessive_output() -> anyhow::Result<()> {
        let sim = PeerSimulator::new().await?;
        let peer = sim.connect().await?;

        let (puzzle_hash, puzzle_reveal) = to_puzzle(1)?;

        let coin = sim.mint_coin(puzzle_hash, 0).await;

        let spend_bundle = SpendBundle::new(
            vec![CoinSpend::new(
                coin,
                puzzle_reveal,
                to_program([CreateCoin::new(puzzle_hash, 1, Vec::new())])?,
            )],
            Signature::default(),
        );

        let ack = peer.send_transaction(spend_bundle).await?;
        assert_eq!(ack.status, 3);

        Ok(())
    }

    #[tokio::test]
    async fn test_lineage() -> anyhow::Result<()> {
        let sim = PeerSimulator::new().await?;
        let peer = sim.connect().await?;

        let (puzzle_hash, puzzle_reveal) = to_puzzle(1)?;

        let mut coin = sim.mint_coin(puzzle_hash, 1000).await;

        for _ in 0..1000 {
            let spend_bundle = SpendBundle::new(
                vec![CoinSpend::new(
                    coin,
                    puzzle_reveal.clone(),
                    to_program([CreateCoin::new(puzzle_hash, coin.amount - 1, Vec::new())])?,
                )],
                Signature::default(),
            );

            let ack = peer.send_transaction(spend_bundle).await?;
            assert_eq!(ack.status, 1);

            coin = Coin::new(coin.coin_id(), puzzle_hash, coin.amount - 1);
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_request_children_unknown() -> anyhow::Result<()> {
        let sim = PeerSimulator::new().await?;
        let peer = sim.connect().await?;

        let children = peer.request_children(Bytes32::default()).await?;
        assert!(children.coin_states.is_empty());

        Ok(())
    }

    #[tokio::test]
    async fn test_request_empty_children() -> anyhow::Result<()> {
        let sim = PeerSimulator::new().await?;
        let peer = sim.connect().await?;

        let (puzzle_hash, puzzle_reveal) = to_puzzle(1)?;

        let coin = sim.mint_coin(puzzle_hash, 0).await;

        let spend_bundle = SpendBundle::new(
            vec![CoinSpend::new(coin, puzzle_reveal, to_program(())?)],
            Signature::default(),
        );

        let ack = peer.send_transaction(spend_bundle).await?;
        assert_eq!(ack.status, 1);

        let children = peer.request_children(coin.coin_id()).await?;
        assert!(children.coin_states.is_empty());

        Ok(())
    }

    #[tokio::test]
    async fn test_request_children() -> anyhow::Result<()> {
        let sim = PeerSimulator::new().await?;
        let peer = sim.connect().await?;

        let (puzzle_hash, puzzle_reveal) = to_puzzle(1)?;

        let coin = sim.mint_coin(puzzle_hash, 3).await;

        let spend_bundle = SpendBundle::new(
            vec![CoinSpend::new(
                coin,
                puzzle_reveal,
                to_program([
                    CreateCoin::new(puzzle_hash, 1, Vec::new()),
                    CreateCoin::new(puzzle_hash, 2, Vec::new()),
                ])?,
            )],
            Signature::default(),
        );

        let ack = peer.send_transaction(spend_bundle).await?;
        assert_eq!(ack.status, 1);

        let children = peer.request_children(coin.coin_id()).await?;
        assert_eq!(children.coin_states.len(), 2);

        let found_1 = children
            .coin_states
            .iter()
            .find(|cs| cs.coin.amount == 1)
            .copied();
        let found_2 = children
            .coin_states
            .iter()
            .find(|cs| cs.coin.amount == 2)
            .copied();

        let expected_1 = CoinState::new(Coin::new(coin.coin_id(), puzzle_hash, 1), None, Some(0));
        let expected_2 = CoinState::new(Coin::new(coin.coin_id(), puzzle_hash, 2), None, Some(0));

        assert_eq!(found_1, Some(expected_1));
        assert_eq!(found_2, Some(expected_2));

        Ok(())
    }

    #[tokio::test]
    async fn test_puzzle_solution() -> anyhow::Result<()> {
        let sim = PeerSimulator::new().await?;
        let peer = sim.connect().await?;

        let (puzzle_hash, puzzle_reveal) = to_puzzle(1)?;
        let solution = to_program([Remark::new(())])?;

        let coin = sim.mint_coin(puzzle_hash, 0).await;

        let spend_bundle = SpendBundle::new(
            vec![CoinSpend::new(
                coin,
                puzzle_reveal.clone(),
                solution.clone(),
            )],
            Signature::default(),
        );

        let ack = peer.send_transaction(spend_bundle).await?;
        assert_eq!(ack.status, 1);

        let response = peer
            .request_puzzle_and_solution(coin.coin_id(), 0)
            .await?
            .unwrap();
        assert_eq!(response.coin_name, coin.coin_id());
        assert_eq!(response.puzzle, puzzle_reveal);
        assert_eq!(response.solution, solution);
        assert_eq!(response.height, 0);

        Ok(())
    }

    #[tokio::test]
    async fn test_spent_coin_subscription() -> anyhow::Result<()> {
        let sim = PeerSimulator::new().await?;
        let (peer, mut receiver) = sim.connect_split().await?;

        let (puzzle_hash, puzzle_reveal) = to_puzzle(1)?;

        let coin = sim.mint_coin(puzzle_hash, 0).await;
        let mut coin_state = sim
            .coin_state(coin.coin_id())
            .await
            .expect("missing coin state");

        let coin_states = peer
            .register_for_coin_updates(vec![coin.coin_id()], 0)
            .await?
            .coin_states;
        assert_eq!(coin_states.len(), 1);
        assert_eq!(coin_states[0], coin_state);

        let spend_bundle = SpendBundle::new(
            vec![CoinSpend::new(coin, puzzle_reveal, to_program(())?)],
            Signature::default(),
        );

        let ack = peer.send_transaction(spend_bundle).await?;
        assert_eq!(ack.status, 1);

        coin_state.spent_height = Some(0);

        let updates = coin_state_updates(&mut receiver);
        assert_eq!(updates.len(), 1);

        assert_eq!(
            updates[0],
            CoinStateUpdate::new(1, 1, sim.peak_hash().await, vec![coin_state])
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_created_coin_subscription() -> anyhow::Result<()> {
        let sim = PeerSimulator::new().await?;
        let (peer, mut receiver) = sim.connect_split().await?;

        let (puzzle_hash, puzzle_reveal) = to_puzzle(1)?;

        let coin = sim.mint_coin(puzzle_hash, 1).await;
        let child_coin = Coin::new(coin.coin_id(), puzzle_hash, 1);

        let coin_states = peer
            .register_for_coin_updates(vec![child_coin.coin_id()], 0)
            .await?
            .coin_states;
        assert_eq!(coin_states.len(), 0);

        let spend_bundle = SpendBundle::new(
            vec![CoinSpend::new(
                coin,
                puzzle_reveal,
                to_program([CreateCoin::new(puzzle_hash, 1, Vec::new())])?,
            )],
            Signature::default(),
        );

        let ack = peer.send_transaction(spend_bundle).await?;
        assert_eq!(ack.status, 1);

        let updates = coin_state_updates(&mut receiver);
        assert_eq!(updates.len(), 1);

        let coin_state = CoinState::new(child_coin, None, Some(0));

        assert_eq!(
            updates[0],
            CoinStateUpdate::new(1, 1, sim.peak_hash().await, vec![coin_state])
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_spent_puzzle_subscription() -> anyhow::Result<()> {
        let sim = PeerSimulator::new().await?;
        let (peer, mut receiver) = sim.connect_split().await?;

        let (puzzle_hash, puzzle_reveal) = to_puzzle(1)?;

        let coin = sim.mint_coin(puzzle_hash, 0).await;
        let mut coin_state = sim
            .coin_state(coin.coin_id())
            .await
            .expect("missing coin state");

        let coin_states = peer
            .register_for_ph_updates(vec![coin.puzzle_hash], 0)
            .await?
            .coin_states;
        assert_eq!(coin_states.len(), 1);
        assert_eq!(coin_states[0], coin_state);

        let spend_bundle = SpendBundle::new(
            vec![CoinSpend::new(coin, puzzle_reveal, to_program(())?)],
            Signature::default(),
        );

        let ack = peer.send_transaction(spend_bundle).await?;
        assert_eq!(ack.status, 1);

        coin_state.spent_height = Some(0);

        let updates = coin_state_updates(&mut receiver);
        assert_eq!(updates.len(), 1);

        assert_eq!(
            updates[0],
            CoinStateUpdate::new(1, 1, sim.peak_hash().await, vec![coin_state])
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_created_puzzle_subscription() -> anyhow::Result<()> {
        let sim = PeerSimulator::new().await?;
        let (peer, mut receiver) = sim.connect_split().await?;

        let (puzzle_hash, puzzle_reveal) = to_puzzle(1)?;

        let coin = sim.mint_coin(puzzle_hash, 1).await;
        let child_coin = Coin::new(coin.coin_id(), Bytes32::default(), 1);

        let coin_states = peer
            .register_for_ph_updates(vec![child_coin.puzzle_hash], 0)
            .await?
            .coin_states;
        assert_eq!(coin_states.len(), 0);

        let spend_bundle = SpendBundle::new(
            vec![CoinSpend::new(
                coin,
                puzzle_reveal,
                to_program([CreateCoin::new(child_coin.puzzle_hash, 1, Vec::new())])?,
            )],
            Signature::default(),
        );

        let ack = peer.send_transaction(spend_bundle).await?;
        assert_eq!(ack.status, 1);

        let updates = coin_state_updates(&mut receiver);
        assert_eq!(updates.len(), 1);

        let coin_state = CoinState::new(child_coin, None, Some(0));

        assert_eq!(
            updates[0],
            CoinStateUpdate::new(1, 1, sim.peak_hash().await, vec![coin_state])
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_spent_hint_subscription() -> anyhow::Result<()> {
        let sim = PeerSimulator::new().await?;
        let (peer, mut receiver) = sim.connect_split().await?;

        let hint = Bytes32::new([42; 32]);
        let (puzzle_hash, puzzle_reveal) = to_puzzle(1)?;

        let coin = sim.mint_coin(puzzle_hash, 0).await;
        sim.add_hint(coin.coin_id(), hint).await;

        let mut coin_state = sim
            .coin_state(coin.coin_id())
            .await
            .expect("missing coin state");

        let coin_states = peer
            .register_for_ph_updates(vec![hint], 0)
            .await?
            .coin_states;
        assert_eq!(coin_states.len(), 1);
        assert_eq!(coin_states[0], coin_state);

        let spend_bundle = SpendBundle::new(
            vec![CoinSpend::new(coin, puzzle_reveal, to_program(())?)],
            Signature::default(),
        );

        let ack = peer.send_transaction(spend_bundle).await?;
        assert_eq!(ack.status, 1);

        coin_state.spent_height = Some(0);

        let updates = coin_state_updates(&mut receiver);
        assert_eq!(updates.len(), 1);

        assert_eq!(
            updates[0],
            CoinStateUpdate::new(1, 1, sim.peak_hash().await, vec![coin_state])
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_created_hint_subscription() -> anyhow::Result<()> {
        let sim = PeerSimulator::new().await?;
        let (peer, mut receiver) = sim.connect_split().await?;

        let hint = Bytes32::new([42; 32]);
        let (puzzle_hash, puzzle_reveal) = to_puzzle(1)?;

        let coin = sim.mint_coin(puzzle_hash, 0).await;

        let coin_states = peer
            .register_for_ph_updates(vec![hint], 0)
            .await?
            .coin_states;
        assert_eq!(coin_states.len(), 0);

        let spend_bundle = SpendBundle::new(
            vec![CoinSpend::new(
                coin,
                puzzle_reveal,
                to_program([CreateCoin::new(puzzle_hash, 0, vec![hint.into()])])?,
            )],
            Signature::default(),
        );

        let ack = peer.send_transaction(spend_bundle).await?;
        assert_eq!(ack.status, 1);

        let updates = coin_state_updates(&mut receiver);
        assert_eq!(updates.len(), 1);

        assert_eq!(
            updates[0],
            CoinStateUpdate::new(
                1,
                1,
                sim.peak_hash().await,
                vec![CoinState::new(
                    Coin::new(coin.coin_id(), puzzle_hash, 0),
                    None,
                    Some(0)
                )]
            )
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_request_coin_state() -> anyhow::Result<()> {
        let sim = PeerSimulator::new().await?;
        let peer = sim.connect().await?;

        let (puzzle_hash, puzzle_reveal) = to_puzzle(1)?;

        let coin = sim.mint_coin(puzzle_hash, 0).await;
        let mut coin_state = sim
            .coin_state(coin.coin_id())
            .await
            .expect("missing coin state");

        let response = peer
            .request_coin_state(
                vec![coin.coin_id()],
                None,
                sim.config().constants.genesis_challenge,
                false,
            )
            .await?
            .unwrap();
        assert_eq!(
            response,
            RespondCoinState::new(vec![coin.coin_id()], vec![coin_state])
        );

        let spend_bundle = SpendBundle::new(
            vec![CoinSpend::new(coin, puzzle_reveal, to_program(())?)],
            Signature::default(),
        );

        let ack = peer.send_transaction(spend_bundle).await?;
        assert_eq!(ack.status, 1);

        coin_state.spent_height = Some(0);

        let response = peer
            .request_coin_state(
                vec![coin.coin_id()],
                None,
                sim.config().constants.genesis_challenge,
                false,
            )
            .await?
            .unwrap();
        assert_eq!(
            response,
            RespondCoinState::new(vec![coin.coin_id()], vec![coin_state])
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_request_puzzle_state() -> anyhow::Result<()> {
        let sim = PeerSimulator::new().await?;
        let peer = sim.connect().await?;

        let (puzzle_hash, puzzle_reveal) = to_puzzle(1)?;

        let coin = sim.mint_coin(puzzle_hash, 0).await;
        let mut coin_state = sim
            .coin_state(coin.coin_id())
            .await
            .expect("missing coin state");

        let response = peer
            .request_puzzle_state(
                vec![puzzle_hash],
                None,
                sim.config().constants.genesis_challenge,
                CoinStateFilters::new(true, true, true, 0),
                false,
            )
            .await?
            .unwrap();
        assert_eq!(
            response,
            RespondPuzzleState::new(
                vec![puzzle_hash],
                0,
                sim.header_hash(0).await,
                true,
                vec![coin_state]
            )
        );

        let spend_bundle = SpendBundle::new(
            vec![CoinSpend::new(coin, puzzle_reveal, to_program(())?)],
            Signature::default(),
        );

        let ack = peer.send_transaction(spend_bundle).await?;
        assert_eq!(ack.status, 1);

        coin_state.spent_height = Some(0);

        let response = peer
            .request_puzzle_state(
                vec![puzzle_hash],
                None,
                sim.config().constants.genesis_challenge,
                CoinStateFilters::new(true, true, true, 0),
                false,
            )
            .await?
            .unwrap();
        assert_eq!(
            response,
            RespondPuzzleState::new(
                vec![puzzle_hash],
                1,
                sim.header_hash(1).await,
                true,
                vec![coin_state]
            )
        );

        Ok(())
    }
}
