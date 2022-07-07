use crate::{
    id,
    tools::anchor::{DISCRIMINATOR_SIZE, PUBKEY_SIZE},
};
use anchor_lang::prelude::*;

/// Registrar which stores Civic Pass voting configuration for the given Realm
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

    /// The Gatekeeper Network represents the "Pass Type" that a
    /// user must present.
    pub gatekeeper_network: Pubkey,

    /// If the plugin is one in a sequence, this is the previous plugin program ID
    /// If set, then update_voter_weight_record will expect a voter_weight_record owned by this program
    pub previous_voter_weight_plugin_program_id: Option<Pubkey>,

    /// Reserved for future upgrades
    pub reserved: [u8; 128],
}

impl Registrar {
    pub fn get_space() -> usize {
        DISCRIMINATOR_SIZE + PUBKEY_SIZE * 4 + (PUBKEY_SIZE + 1) + 128
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

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_get_space() {
        // Arrange
        let expected_space = Registrar::get_space();

        let registrar = Registrar {
            governance_program_id: Pubkey::default(),
            previous_voter_weight_plugin_program_id: Pubkey::default().into(),
            realm: Pubkey::default(),
            governing_token_mint: Pubkey::default(),
            gatekeeper_network: Pubkey::default(),
            reserved: [0; 128],
        };

        // Act
        let actual_space = DISCRIMINATOR_SIZE + registrar.try_to_vec().unwrap().len();

        // Assert
        assert_eq!(expected_space, actual_space);
    }
}
