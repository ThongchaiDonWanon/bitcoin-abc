// Copyright (c) 2023 The Bitcoin developers
// Distributed under the MIT software license, see the accompanying
// file COPYING or http://www.opensource.org/licenses/mit-license.php.

//! Module for [`Group`] and [`GroupQuery`].

use bitcoinsuite_core::tx::Tx;

use crate::io::{GroupHistoryConf, GroupUtxoConf};

/// Struct giving impls of [`Group`] all the necessary data to determine the
/// member of the group.
#[derive(Clone, Copy, Debug)]
pub struct GroupQuery<'a> {
    /// Whether the tx is a coinbase tx.
    pub is_coinbase: bool,
    /// The transaction that should be grouped.
    pub tx: &'a Tx,
}

/// Item returned by `members_tx`.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MemberItem<M> {
    /// Index of the item in the list of inputs/outputs.
    pub idx: usize,
    /// Member of this item.
    pub member: M,
}

/// Groups txs and determines which members they are.
///
/// A member is one instance in a group and can be anything in a tx, e.g. the
/// input and output scripts, the SLP token ID, a SWaP signal, a smart contract
/// etc.
pub trait Group {
    /// Iterator over the members found for a given [`GroupQuery`].
    type Iter<'a>: IntoIterator<Item = MemberItem<Self::Member<'a>>> + 'a;

    /// Member of a group, this is what txs will be grouped by.
    ///
    /// We use a HashMap and a BTreeMap to group txs, so it must implement
    /// [`std::hash::Hash`] and [`Ord`].
    type Member<'a>: std::hash::Hash + Eq + Ord;

    /// Serialized member, this is what will be used as key in the DB.
    /// Normally, this will be a [`Vec<u8>`] or an [`u8`] array or slice.
    ///
    /// We use this to allow members to separate between the code that groups
    /// them from the serialization of members. For example, we can group by raw
    /// input/output script bytecode, but then use compressed scripts when
    /// adding to the DB. That way, we don't have to run the compression every
    /// time we compare/hash elements for grouping.
    ///
    /// Note: For group history, this will be suffixed by a 4-byte page number.
    type MemberSer<'a>: AsRef<[u8]> + 'a;

    /// Find the group's members in the given query's tx's inputs.
    ///
    /// Note: This is allowed to return a member multiple times per query.
    ///
    /// Note: The returned iterator is allowed to borrow from the query.
    fn input_members<'a>(&self, query: GroupQuery<'a>) -> Self::Iter<'a>;

    /// Find the group's members in the given query's tx's outputs.
    ///
    /// Note: This is allowed to return a member multiple times per query.
    ///
    /// Note: The returned iterator is allowed to borrow from the query.
    fn output_members<'a>(&self, query: GroupQuery<'a>) -> Self::Iter<'a>;

    /// Serialize the given member.
    fn ser_member<'a>(&self, member: &Self::Member<'a>) -> Self::MemberSer<'a>;

    /// The [`GroupHistoryConf`] for this group.
    fn tx_history_conf() -> GroupHistoryConf;

    /// The [`GroupUtxoConf`] for this group.
    fn utxo_conf() -> GroupUtxoConf;
}

/// Helper which returns the `G::Member`s of both inputs and outputs of the
/// group for the tx.
pub fn tx_members_for_group<'a, G: Group>(
    group: &G,
    query: GroupQuery<'a>,
) -> impl Iterator<Item = G::Member<'a>> {
    group
        .input_members(query)
        .into_iter()
        .chain(group.output_members(query))
        .map(|item| item.member)
}
