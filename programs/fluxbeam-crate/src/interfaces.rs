use anchor_lang::prelude::*;
use anchor_spl::metadata::MetadataAccount;

// #[derive(Clone)]
// pub struct MetadataInterface;
//
// const IDS: [Pubkey; 3] = [Metadata2022::id(), Metadata2022Devnet::id(), Metadata::id()];
//
// impl anchor_lang::Ids for MetadataInterface {
//     fn ids() -> &'static [Pubkey] {
//         &IDS
//     }
// }

#[derive(Clone)]
pub struct Metadata2022;

impl anchor_lang::Id for Metadata2022 {
    fn id() -> Pubkey {
        Pubkey::try_from("META4s4fSmpkTbZoUsgC1oBnWB31vQcmnN8giPw51Zu").unwrap()
    }
}

pub struct Metadata2022Devnet;

impl anchor_lang::Id for Metadata2022Devnet {
    fn id() -> Pubkey {
        Pubkey::try_from("M1tgEZCz7fHqRAR3G5RLxU6c6ceQiZyFK7tzzy4Rof4").unwrap()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Metadata2022Account(MetadataAccount);

impl anchor_lang::AccountDeserialize for Metadata2022Account {
    fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
        let md = MetadataAccount::try_deserialize(buf)?;
        Ok(Self(md))
    }

    fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
        let md = MetadataAccount::try_deserialize_unchecked(buf)?;
        Ok(Self(md))
    }
}

impl anchor_lang::AccountSerialize for Metadata2022Account {}


impl anchor_lang::Owner for Metadata2022Account {
    fn owner() -> Pubkey {
        Metadata2022Devnet::id()
    }
}