use crate::{
    error::SquadsVoterError,
    id,
    state::SquadConfig,
    tools::anchor::{DISCRIMINATOR_SIZE, PUBKEY_SIZE},
};
use anchor_lang::prelude::*;

/// Registrar which stores Squads voting configuration for the given Realm
#[account]
#[derive(Debug, PartialEq)]
pub struct Registrar {
    /// spl-governance program the Realm belongs to
    pub governance_program_id: Pubkey,

    /// Realm of the Registrar
    pub realm: Pubkey,

    /// Governing token mint the Registrar is for
    /// It can either be the Community or the Council mint of the Realm
    /// When the plugin is used the mint is only used as identity of the governing power (voting population)
    /// and the actual token of the mint is not used
    pub governing_token_mint: Pubkey,

    /// Squads used for governance
    pub squads_configs: Vec<SquadConfig>,

    /// Reserved for future upgrades
    pub reserved: [u8; 128],
}

impl Registrar {
    pub fn get_space(max_squads: u8) -> usize {
        DISCRIMINATOR_SIZE + PUBKEY_SIZE * 3 + 4 + max_squads as usize * (PUBKEY_SIZE + 8 + 8) + 128
    }
}

/// Returns Registrar PDA seeds
pub fn get_registrar_seeds<'a>(
    realm: &'a Pubkey,
    governing_token_mint: &'a Pubkey,
) -> [&'a [u8]; 3] {
    [b"registrar", realm.as_ref(), governing_token_mint.as_ref()]
}

/// Returns Registrar PDA address
pub fn get_registrar_address(realm: &Pubkey, governing_token_mint: &Pubkey) -> Pubkey {
    Pubkey::find_program_address(&get_registrar_seeds(realm, governing_token_mint), &id()).0
}

impl Registrar {
    pub fn get_squad_config(&self, squad: &Pubkey) -> Result<&SquadConfig> {
        return self
            .squads_configs
            .iter()
            .find(|sc| sc.squad == *squad)
            .ok_or_else(|| SquadsVoterError::SquadNotFound.into());
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_get_space() {
        // Arrange
        let expected_space = Registrar::get_space(3);

        let registrar = Registrar {
            governance_program_id: Pubkey::default(),
            realm: Pubkey::default(),
            governing_token_mint: Pubkey::default(),
            squads_configs: vec![
                SquadConfig::default(),
                SquadConfig::default(),
                SquadConfig::default(),
            ],
            reserved: [0; 128],
        };

        // Act
        let actual_space = DISCRIMINATOR_SIZE + registrar.try_to_vec().unwrap().len();

        // Assert
        assert_eq!(expected_space, actual_space);
    }
}
