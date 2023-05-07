export type Gateway = {
  "version": "0.1.1",
  "name": "gateway",
  "instructions": [
    {
      "name": "createRegistrar",
      "accounts": [
        {
          "name": "registrar",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "The Gateway Registrar",
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
          "name": "gatekeeperNetwork",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "The Identity.com Gateway gatekeeper network that this realm uses",
            "(See the registry struct docs for details).",
            "Gateway Token belongs to this gatekeeper network, so passing a particular key here is",
            "essentially saying \"We trust this gatekeeper network\"."
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
            "The Gateway Plugin Registrar to be updated"
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
        },
        {
          "name": "gatekeeperNetwork",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "The new Identity.com Gateway gatekeeper network",
            "(See the registry struct docs for details).",
            "Gateway Token belongs to this gatekeeper network, so passing a particular key here is",
            "essentially saying \"We trust this gatekeeper network\"."
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
            "The Gateway Registrar"
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
          "name": "gatewayToken",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "A gateway token from the gatekeeper network in the registrar.",
            "Proves that the holder is permitted to take an action."
          ]
        },
        {
          "name": "voterWeightRecord",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": []
    }
  ],
  "accounts": [
    {
      "name": "registrar",
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
            "name": "gatekeeperNetwork",
            "docs": [
              "The Gatekeeper Network represents the \"Pass Type\" that a",
              "user must present."
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
      "name": "voterWeightRecord",
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
      "name": "GenericVoterWeightEnum",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "VoterWeightRecord",
            "fields": [
              {
                "defined": "spl_governance_addin_api::voter_weight::VoterWeightRecord"
              }
            ]
          },
          {
            "name": "TokenOwnerRecord",
            "fields": [
              {
                "defined": "TokenOwnerRecordV2"
              }
            ]
          }
        ]
      }
    },
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
      "name": "InvalidGatewayToken",
      "msg": "Invalid gateway token"
    },
    {
      "code": 6010,
      "name": "MissingPreviousVoterWeightPlugin",
      "msg": "Previous voter weight plugin required but not provided"
    }
  ]
};

export const IDL: Gateway = {
  "version": "0.1.1",
  "name": "gateway",
  "instructions": [
    {
      "name": "createRegistrar",
      "accounts": [
        {
          "name": "registrar",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "The Gateway Registrar",
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
          "name": "gatekeeperNetwork",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "The Identity.com Gateway gatekeeper network that this realm uses",
            "(See the registry struct docs for details).",
            "Gateway Token belongs to this gatekeeper network, so passing a particular key here is",
            "essentially saying \"We trust this gatekeeper network\"."
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
            "The Gateway Plugin Registrar to be updated"
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
        },
        {
          "name": "gatekeeperNetwork",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "The new Identity.com Gateway gatekeeper network",
            "(See the registry struct docs for details).",
            "Gateway Token belongs to this gatekeeper network, so passing a particular key here is",
            "essentially saying \"We trust this gatekeeper network\"."
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
            "The Gateway Registrar"
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
          "name": "gatewayToken",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "A gateway token from the gatekeeper network in the registrar.",
            "Proves that the holder is permitted to take an action."
          ]
        },
        {
          "name": "voterWeightRecord",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": []
    }
  ],
  "accounts": [
    {
      "name": "registrar",
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
            "name": "gatekeeperNetwork",
            "docs": [
              "The Gatekeeper Network represents the \"Pass Type\" that a",
              "user must present."
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
      "name": "voterWeightRecord",
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
      "name": "GenericVoterWeightEnum",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "VoterWeightRecord",
            "fields": [
              {
                "defined": "spl_governance_addin_api::voter_weight::VoterWeightRecord"
              }
            ]
          },
          {
            "name": "TokenOwnerRecord",
            "fields": [
              {
                "defined": "TokenOwnerRecordV2"
              }
            ]
          }
        ]
      }
    },
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
      "name": "InvalidGatewayToken",
      "msg": "Invalid gateway token"
    },
    {
      "code": 6010,
      "name": "MissingPreviousVoterWeightPlugin",
      "msg": "Previous voter weight plugin required but not provided"
    }
  ]
};
