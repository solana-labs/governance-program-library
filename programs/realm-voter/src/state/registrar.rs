use crate::{
    error::SquadsVoterError,
    id,
    state::GovernanceProgramConfig,
    tools::anchor::{DISCRIMINATOR_SIZE, PUBKEY_SIZE},
};
use anchor_lang::prelude::*;

/// Registrar which stores spl-governance configurations for the given Realm
#[account]
#[derive(Debug, PartialEq)]
pub struct Registrar {
    /// spl-governance program the Realm belongs to
    pub governance_program_id: Pubkey,

    /// Realm of the Registrar
    pub realm: Pubkey,

    /// Governing token mint the Registrar is for
    /// It can either be the Community or the Council mint of the Realm
    /// When the plugin is enabled the mint is only used as the identity of the governing power (voting population)
    /// and the actual token of the mint is not used
    pub governing_token_mint: Pubkey,

    /// spl-governance instances used for governance power
    /// Any DAO member of any DAO created using the configured spl-governances would be given 1 vote
    /// TODO: Once we have on-chain spl-governance registry this configuration won't be needed any longer
    pub governance_program_configs: Vec<GovernanceProgramConfig>,

    /// Max voter weight (expressed in governing_token_mint decimal units) is used to establish the theoretical Max Attendance Quorum which is then used to calculate Approval Quorum
    /// This manual configuration is a rough estimate because it's not practical to calculate on-chain the number of all DAO members for the given spl-governance instances
    ///
    /// Note: This is not a security vulnerability because the plugin is inherently not secure and used only to encourage DAO usage and registration of spl-governance instances
    pub max_voter_weight: u64,

    /// Reserved for future upgrades
    pub reserved: [u8; 128],
}

impl Registrar {
    pub fn get_space(max_governance_programs: u8) -> usize {
        DISCRIMINATOR_SIZE
            + PUBKEY_SIZE * 3
            + 4
            + max_governance_programs as usize * (PUBKEY_SIZE + 8)
            + 8
            + 128
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
    pub fn get_squad_config(&self, squad: &Pubkey) -> Result<&GovernanceProgramConfig> {
        return self
            .governance_program_configs
            .iter()
            .find(|sc| sc.program_id == *squad)
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
            governance_program_configs: vec![
                GovernanceProgramConfig::default(),
                GovernanceProgramConfig::default(),
                GovernanceProgramConfig::default(),
            ],
            reserved: [0; 128],
            max_voter_weight: 100,
        };

        // Act
        let actual_space = DISCRIMINATOR_SIZE + registrar.try_to_vec().unwrap().len();

        // Assert
        assert_eq!(expected_space, actual_space);
    }
}
