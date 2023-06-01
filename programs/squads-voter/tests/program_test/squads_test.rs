use crate::program_test::program_test_bench::ProgramTestBench;
use anchor_lang::prelude::Pubkey;
use solana_program_test::ProgramTest;
use solana_sdk::transport::TransportError;
use std::{str::FromStr, sync::Arc};

pub struct SquadCookie {
    pub address: Pubkey,
}

pub struct SquadMemberCookie {
    pub address: Pubkey,
    pub squad_address: Pubkey,
}

pub struct SquadsTest {
    pub bench: Arc<ProgramTestBench>,
    pub program_id: Pubkey,
}

impl SquadsTest {
    pub fn program_id() -> Pubkey {
        Pubkey::from_str("Sqds1ufWkcv5z7K4RPXnrVTNq6Yw3zhEuajPkfhLpek").unwrap()
    }

    #[allow(dead_code)]
    pub fn add_program(program_test: &mut ProgramTest) {
        // TODO: Add squads_protocol program to fixtures and replace it's name
        program_test.add_program("spl_governance", Self::program_id(), None);
    }

    #[allow(dead_code)]
    pub fn new(bench: Arc<ProgramTestBench>) -> Self {
        SquadsTest {
            bench,
            program_id: Self::program_id(),
        }
    }

    #[allow(dead_code)]
    pub async fn with_squad(&mut self) -> Result<SquadCookie, TransportError> {
        // TODO: Create Squad

        let squad_address = Pubkey::new_unique();
        let squad_cookie = SquadCookie {
            address: squad_address,
        };

        Ok(squad_cookie)
    }

    #[allow(dead_code)]
    pub async fn with_squad_member(
        &mut self,
        squad_cookie: &SquadCookie,
    ) -> Result<SquadMemberCookie, TransportError> {
        // TODO: Create Squad Member

        let squad_member = Pubkey::new_unique();
        let squad_member_cookie = SquadMemberCookie {
            address: squad_member,
            squad_address: squad_cookie.address,
        };

        Ok(squad_member_cookie)
    }
}
