import { Idl } from '@project-serum/anchor';

export const IDL: Idl = {
  "version": "0.1.1",
  "name": "quadratic",
  "instructions": [
    {
      "name": "createRegistrar",
      "accounts": [
        {
          "name": "registrar",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "The quadratic Registrar",
            "There can only be a single registrar per governance Realm and governing mint of the Realm"
          ]
        },
        {
          "name": "governanceProgramId",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "The program id of the spl-governance program the realm belongs to"
          ]
        },
        {
          "name": "realm",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "An spl-governance Realm",
            "",
            "Realm is validated in the instruction:",
            "- Realm is owned by the governance_program_id",
            "- governing_token_mint must be the community or council mint",
            "- realm_authority is realm.authority"
          ]
        },
        {
          "name": "governingTokenMint",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "Either the realm community mint or the council mint.",
            "It must match Realm.community_mint or Realm.config.council_mint",
            "",
            "Note: Once the Civic Pass plugin is enabled the governing_token_mint is used only as identity",
            "for the voting population and the tokens of that are no longer used"
          ]
        },
        {
          "name": "realmAuthority",
          "isMut": false,
          "isSigner": true,
          "docs": [
            "realm_authority must sign and match Realm.authority"
          ]
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "usePreviousVoterWeightPlugin",
          "type": "bool"
        }
      ]
    },
    {
      "name": "configureRegistrar",
      "accounts": [
        {
          "name": "registrar",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "The quadratic Plugin Registrar to be updated"
          ]
        },
        {
          "name": "realm",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "An spl-governance Realm",
            "",
            "Realm is validated in the instruction:",
            "- Realm is owned by the governance_program_id",
            "- realm_authority is realm.authority"
          ]
        },
        {
          "name": "realmAuthority",
          "isMut": false,
          "isSigner": true,
          "docs": [
            "realm_authority must sign and match Realm.authority"
          ]
        }
      ],
      "args": [
        {
          "name": "usePreviousVoterWeightPlugin",
          "type": "bool"
        }
      ]
    },
    {
      "name": "createVoterWeightRecord",
      "accounts": [
        {
          "name": "registrar",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "voterWeightRecord",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "governingTokenOwner",
          "type": "publicKey"
        }
      ]
    },
    {
      "name": "updateVoterWeightRecord",
      "accounts": [
        {
          "name": "registrar",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "The quadratic plugin Registrar"
          ]
        },
        {
          "name": "inputVoterWeight",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "An account that is either of type TokenOwnerRecordV2 or VoterWeightRecord",
            "depending on whether the registrar includes a predecessor or not"
          ]
        },
        {
          "name": "voterWeightRecord",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "createMaxVoterWeightRecord",
      "accounts": [
        {
          "name": "maxVoterWeightRecord",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "governanceProgramId",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "The program id of the spl-governance program the realm belongs to"
          ]
        },
        {
          "name": "realm",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "realmGoverningTokenMint",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "Either the realm community mint or the council mint."
          ]
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "updateMaxVoterWeightRecord",
      "accounts": [
        {
          "name": "registrar",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "The quadratic plugin Registrar"
          ]
        },
        {
          "name": "inputMaxVoterWeight",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "An account that is either a governance token mint or a MaxVoterWeightRecord",
            "from a predecessor plugin.",
            "depending on whether the registrar includes a predecessor or not"
          ]
        },
        {
          "name": "maxVoterWeightRecord",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": []
    }
  ],
  "accounts": [
    {
      "name": "MaxVoterWeightRecord",
      "docs": [
        "MaxVoterWeightRecord account as defined in spl-governance-addin-api",
        "It's redefined here without account_discriminator for Anchor to treat it as native account",
        "",
        "The account is used as an api interface to provide max voting power to the governance program from external addin contracts"
      ],
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "realm",
            "docs": [
              "The Realm the MaxVoterWeightRecord belongs to"
            ],
            "type": "publicKey"
          },
          {
            "name": "governingTokenMint",
            "docs": [
              "Governing Token Mint the MaxVoterWeightRecord is associated with",
              "Note: The addin can take deposits of any tokens and is not restricted to the community or council tokens only",
              "The mint here is to link the record to either community or council mint of the realm"
            ],
            "type": "publicKey"
          },
          {
            "name": "maxVoterWeight",
            "docs": [
              "Max voter weight",
              "The max voter weight provided by the addin for the given realm and governing_token_mint"
            ],
            "type": "u64"
          },
          {
            "name": "maxVoterWeightExpiry",
            "docs": [
              "The slot when the max voting weight expires",
              "It should be set to None if the weight never expires",
              "If the max vote weight decays with time, for example for time locked based weights, then the expiry must be set",
              "As a pattern Revise instruction to update the max weight should be invoked before governance instruction within the same transaction",
              "and the expiry set to the current slot to provide up to date weight"
            ],
            "type": {
              "option": "u64"
            }
          },
          {
            "name": "reserved",
            "docs": [
              "Reserved space for future versions"
            ],
            "type": {
              "array": [
                "u8",
                8
              ]
            }
          }
        ]
      }
    },
    {
      "name": "Registrar",
      "docs": [
        "Registrar which stores Civic Pass voting configuration for the given Realm"
      ],
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "governanceProgramId",
            "docs": [
              "spl-governance program the Realm belongs to"
            ],
            "type": "publicKey"
          },
          {
            "name": "realm",
            "docs": [
              "Realm of the Registrar"
            ],
            "type": "publicKey"
          },
          {
            "name": "governingTokenMint",
            "docs": [
              "Governing token mint the Registrar is for",
              "It can either be the Community or the Council mint of the Realm",
              "When the plugin is used the mint is only used as identity of the governing power (voting population)",
              "and the actual token of the mint is not used"
            ],
            "type": "publicKey"
          },
          {
            "name": "previousVoterWeightPluginProgramId",
            "docs": [
              "If the plugin is one in a sequence, this is the previous plugin program ID",
              "If set, then update_voter_weight_record will expect a voter_weight_record owned by this program"
            ],
            "type": {
              "option": "publicKey"
            }
          },
          {
            "name": "reserved",
            "docs": [
              "Reserved for future upgrades"
            ],
            "type": {
              "array": [
                "u8",
                128
              ]
            }
          }
        ]
      }
    },
    {
      "name": "VoterWeightRecord",
      "docs": [
        "VoterWeightRecord account as defined in spl-governance-addin-api",
        "It's redefined here without account_discriminator for Anchor to treat it as native account",
        "",
        "The account is used as an api interface to provide voting power to the governance program from external addin contracts"
      ],
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "realm",
            "docs": [
              "The Realm the VoterWeightRecord belongs to"
            ],
            "type": "publicKey"
          },
          {
            "name": "governingTokenMint",
            "docs": [
              "Governing Token Mint the VoterWeightRecord is associated with",
              "Note: The addin can take deposits of any tokens and is not restricted to the community or council tokens only"
            ],
            "type": "publicKey"
          },
          {
            "name": "governingTokenOwner",
            "docs": [
              "The owner of the governing token and voter",
              "This is the actual owner (voter) and corresponds to TokenOwnerRecord.governing_token_owner"
            ],
            "type": "publicKey"
          },
          {
            "name": "voterWeight",
            "docs": [
              "Voter's weight",
              "The weight of the voter provided by the addin for the given realm, governing_token_mint and governing_token_owner (voter)"
            ],
            "type": "u64"
          },
          {
            "name": "voterWeightExpiry",
            "docs": [
              "The slot when the voting weight expires",
              "It should be set to None if the weight never expires",
              "If the voter weight decays with time, for example for time locked based weights, then the expiry must be set",
              "As a common pattern Revise instruction to update the weight should be invoked before governance instruction within the same transaction",
              "and the expiry set to the current slot to provide up to date weight"
            ],
            "type": {
              "option": "u64"
            }
          },
          {
            "name": "weightAction",
            "docs": [
              "The governance action the voter's weight pertains to",
              "It allows to provided voter's weight specific to the particular action the weight is evaluated for",
              "When the action is provided then the governance program asserts the executing action is the same as specified by the addin"
            ],
            "type": {
              "option": {
                "defined": "VoterWeightAction"
              }
            }
          },
          {
            "name": "weightActionTarget",
            "docs": [
              "The target the voter's weight  action pertains to",
              "It allows to provided voter's weight specific to the target the weight is evaluated for",
              "For example when addin supplies weight to vote on a particular proposal then it must specify the proposal as the action target",
              "When the target is provided then the governance program asserts the target is the same as specified by the addin"
            ],
            "type": {
              "option": "publicKey"
            }
          },
          {
            "name": "reserved",
            "docs": [
              "Reserved space for future versions"
            ],
            "type": {
              "array": [
                "u8",
                8
              ]
            }
          }
        ]
      }
    }
  ],
  "types": [
    {
      "name": "VoterWeightAction",
      "docs": [
        "VoterWeightAction enum as defined in spl-governance-addin-api",
        "It's redefined here for Anchor to export it to IDL"
      ],
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "CastVote"
          },
          {
            "name": "CommentProposal"
          },
          {
            "name": "CreateGovernance"
          },
          {
            "name": "CreateProposal"
          },
          {
            "name": "SignOffProposal"
          }
        ]
      }
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "InvalidRealmAuthority",
      "msg": "Invalid realm authority"
    },
    {
      "code": 6001,
      "name": "InvalidRealmForRegistrar",
      "msg": "Invalid realm for the provided registrar"
    },
    {
      "code": 6002,
      "name": "InvalidPredecessorTokenOwnerRecord",
      "msg": "Invalid TokenOwnerRecord as input voter weight (expecting TokenOwnerRecord V1 or V2)"
    },
    {
      "code": 6003,
      "name": "InvalidPredecessorVoterWeightRecord",
      "msg": "Invalid VoterWeightRecord as input voter weight (expecting VoterWeightRecord)"
    },
    {
      "code": 6004,
      "name": "InvalidPredecessorVoterWeightRecordRealm",
      "msg": "Invalid VoterWeightRecord realm for input voter weight"
    },
    {
      "code": 6005,
      "name": "InvalidPredecessorVoterWeightRecordGovTokenMint",
      "msg": "Invalid VoterWeightRecord governance token mint for input voter weight"
    },
    {
      "code": 6006,
      "name": "InvalidPredecessorVoterWeightRecordGovTokenOwner",
      "msg": "Invalid VoterWeightRecord governance token owner for input voter weight"
    },
    {
      "code": 6007,
      "name": "InvalidVoterWeightRecordRealm",
      "msg": "Invalid VoterWeightRecord realm"
    },
    {
      "code": 6008,
      "name": "InvalidVoterWeightRecordMint",
      "msg": "Invalid VoterWeightRecord mint"
    },
    {
      "code": 6009,
      "name": "MissingPreviousVoterWeightPlugin",
      "msg": "Previous voter weight plugin required but not provided"
    }
  ]
};

export type Quadratic = typeof IDL;