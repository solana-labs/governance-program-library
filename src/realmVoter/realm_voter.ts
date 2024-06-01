/**
 * Program IDL in camelCase format in order to be used in JS/TS.
 *
 * Note that this is only a type helper and is not the actual IDL. The original
 * IDL can be found at `target/idl/realm_voter.json`.
 */
export type RealmVoter = {
  "address": "GRmVtfLq2BPeWs5EDoQoZc787VYkhdkA11k63QM1Xemz",
  "metadata": {
    "name": "realmVoter",
    "version": "0.0.1",
    "spec": "0.1.0",
    "description": "SPL Governance plugin granting governance power through Realms membership"
  },
  "instructions": [
    {
      "name": "configureGovernanceProgram",
      "discriminator": [
        249,
        113,
        78,
        90,
        175,
        252,
        38,
        185
      ],
      "accounts": [
        {
          "name": "registrar",
          "docs": [
            "Registrar which we configure the provided spl-governance instance for"
          ],
          "writable": true
        },
        {
          "name": "realm"
        },
        {
          "name": "realmAuthority",
          "docs": [
            "Authority of the Realm must sign the transaction and must match realm.authority"
          ],
          "signer": true
        },
        {
          "name": "governanceProgramId",
          "docs": [
            "The onus is entirely on the  caller side to ensure the provided instance is correct",
            "In future versions once we have the registry of spl-governance instances it could be validated against the registry"
          ]
        }
      ],
      "args": [
        {
          "name": "changeType",
          "type": {
            "defined": {
              "name": "collectionItemChangeType"
            }
          }
        }
      ]
    },
    {
      "name": "configureVoterWeights",
      "discriminator": [
        183,
        82,
        128,
        126,
        12,
        243,
        124,
        214
      ],
      "accounts": [
        {
          "name": "registrar",
          "docs": [
            "The Registrar for the given realm and governing_token_mint"
          ],
          "writable": true
        },
        {
          "name": "realm"
        },
        {
          "name": "realmAuthority",
          "docs": [
            "Authority of the Realm must sign and match realm.authority"
          ],
          "signer": true
        },
        {
          "name": "maxVoterWeightRecord",
          "docs": [
            "MaxVoterWeightRecord for the given registrar.realm and registrar.governing_token_mint"
          ],
          "writable": true
        }
      ],
      "args": [
        {
          "name": "realmMemberVoterWeight",
          "type": "u64"
        },
        {
          "name": "maxVoterWeight",
          "type": "u64"
        }
      ]
    },
    {
      "name": "createMaxVoterWeightRecord",
      "discriminator": [
        182,
        70,
        243,
        119,
        162,
        176,
        38,
        248
      ],
      "accounts": [
        {
          "name": "registrar"
        },
        {
          "name": "maxVoterWeightRecord",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  109,
                  97,
                  120,
                  45,
                  118,
                  111,
                  116,
                  101,
                  114,
                  45,
                  119,
                  101,
                  105,
                  103,
                  104,
                  116,
                  45,
                  114,
                  101,
                  99,
                  111,
                  114,
                  100
                ]
              },
              {
                "kind": "account",
                "path": "registrar.realm",
                "account": "registrar"
              },
              {
                "kind": "account",
                "path": "registrar.governing_token_mint",
                "account": "registrar"
              }
            ]
          }
        },
        {
          "name": "payer",
          "writable": true,
          "signer": true
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": []
    },
    {
      "name": "createRegistrar",
      "discriminator": [
        132,
        235,
        36,
        49,
        139,
        66,
        202,
        69
      ],
      "accounts": [
        {
          "name": "registrar",
          "docs": [
            "The Realm Voter Registrar",
            "There can only be a single registrar per governance Realm and governing mint of the Realm"
          ],
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  114,
                  101,
                  103,
                  105,
                  115,
                  116,
                  114,
                  97,
                  114
                ]
              },
              {
                "kind": "account",
                "path": "realm"
              },
              {
                "kind": "account",
                "path": "governingTokenMint"
              }
            ]
          }
        },
        {
          "name": "governanceProgramId",
          "docs": [
            "The program id of the spl-governance program the realm belongs to"
          ]
        },
        {
          "name": "realm",
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
          "docs": [
            "Either the realm community mint or the council mint.",
            "It must match Realm.community_mint or Realm.config.council_mint",
            "",
            "Note: Once the Realm voter plugin is enabled the governing_token_mint is used only as identity",
            "for the voting population and the tokens of that are no longer used"
          ]
        },
        {
          "name": "realmAuthority",
          "docs": [
            "realm_authority must sign and match Realm.authority"
          ],
          "signer": true
        },
        {
          "name": "payer",
          "writable": true,
          "signer": true
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "maxGovernancePrograms",
          "type": "u8"
        }
      ]
    },
    {
      "name": "createVoterWeightRecord",
      "discriminator": [
        184,
        249,
        133,
        178,
        88,
        152,
        250,
        186
      ],
      "accounts": [
        {
          "name": "registrar"
        },
        {
          "name": "voterWeightRecord",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  118,
                  111,
                  116,
                  101,
                  114,
                  45,
                  119,
                  101,
                  105,
                  103,
                  104,
                  116,
                  45,
                  114,
                  101,
                  99,
                  111,
                  114,
                  100
                ]
              },
              {
                "kind": "account",
                "path": "registrar.realm",
                "account": "registrar"
              },
              {
                "kind": "account",
                "path": "registrar.governing_token_mint",
                "account": "registrar"
              },
              {
                "kind": "arg",
                "path": "governingTokenOwner"
              }
            ]
          }
        },
        {
          "name": "payer",
          "writable": true,
          "signer": true
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "governingTokenOwner",
          "type": "pubkey"
        }
      ]
    },
    {
      "name": "updateVoterWeightRecord",
      "discriminator": [
        45,
        185,
        3,
        36,
        109,
        190,
        115,
        169
      ],
      "accounts": [
        {
          "name": "registrar",
          "docs": [
            "The RealmVoter voting Registrar"
          ]
        },
        {
          "name": "voterWeightRecord",
          "writable": true
        },
        {
          "name": "tokenOwnerRecord",
          "docs": [
            "TokenOwnerRecord for any of the configured spl-governance instances"
          ]
        }
      ],
      "args": []
    }
  ],
  "accounts": [
    {
      "name": "maxVoterWeightRecord",
      "discriminator": [
        157,
        95,
        242,
        151,
        16,
        98,
        26,
        118
      ]
    },
    {
      "name": "registrar",
      "discriminator": [
        193,
        202,
        205,
        51,
        78,
        168,
        150,
        128
      ]
    },
    {
      "name": "voterWeightRecord",
      "discriminator": [
        46,
        249,
        155,
        75,
        153,
        248,
        116,
        9
      ]
    }
  ],
  "types": [
    {
      "name": "collectionItemChangeType",
      "docs": [
        "Enum defining collection item change type"
      ],
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "upsert"
          },
          {
            "name": "remove"
          }
        ]
      }
    },
    {
      "name": "governanceProgramConfig",
      "docs": [
        "Configuration of an spl-governance instance used to grant governance power"
      ],
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "programId",
            "docs": [
              "The program id of the configured spl-governance instance"
            ],
            "type": "pubkey"
          },
          {
            "name": "reserved",
            "docs": [
              "Reserved for future upgrades"
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
      "name": "maxVoterWeightRecord",
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
            "type": "pubkey"
          },
          {
            "name": "governingTokenMint",
            "docs": [
              "Governing Token Mint the MaxVoterWeightRecord is associated with",
              "Note: The addin can take deposits of any tokens and is not restricted to the community or council tokens only"
            ],
            "type": "pubkey"
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
      "name": "registrar",
      "docs": [
        "Registrar which stores spl-governance configurations for the given Realm"
      ],
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "governanceProgramId",
            "docs": [
              "spl-governance program the Realm belongs to"
            ],
            "type": "pubkey"
          },
          {
            "name": "realm",
            "docs": [
              "Realm of the Registrar"
            ],
            "type": "pubkey"
          },
          {
            "name": "governingTokenMint",
            "docs": [
              "Governing token mint the Registrar is for",
              "It can either be the Community or the Council mint of the Realm",
              "When the plugin is enabled the mint is only used as the identity of the governing power (voting population)",
              "and the actual token of the mint is not used"
            ],
            "type": "pubkey"
          },
          {
            "name": "governanceProgramConfigs",
            "docs": [
              "spl-governance instances used for governance power",
              "Any DAO member of any DAO created using the configured spl-governances would be given 1 vote",
              "TODO: Once we have on-chain spl-governance registry this configuration won't be needed any longer"
            ],
            "type": {
              "vec": {
                "defined": {
                  "name": "governanceProgramConfig"
                }
              }
            }
          },
          {
            "name": "realmMemberVoterWeight",
            "docs": [
              "Vote weight assigned to a member of any of the Realms from the configured spl-governances"
            ],
            "type": "u64"
          },
          {
            "name": "maxVoterWeight",
            "docs": [
              "Max voter weight (expressed in governing_token_mint decimal units) is used to establish the theoretical Max Attendance Quorum which is then used to calculate Approval Quorum",
              "This manual configuration is a rough estimate because it's not practical to calculate on-chain the number of all DAO members for the given spl-governance instances",
              "",
              "Note: This is not a security vulnerability because the plugin is inherently not secure and used only to encourage DAO usage and registration of spl-governance instances"
            ],
            "type": "u64"
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
      "name": "voterWeightAction",
      "docs": [
        "VoterWeightAction enum as defined in spl-governance-addin-api",
        "It's redefined here for Anchor to export it to IDL"
      ],
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "castVote"
          },
          {
            "name": "commentProposal"
          },
          {
            "name": "createGovernance"
          },
          {
            "name": "createProposal"
          },
          {
            "name": "signOffProposal"
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
            "type": "pubkey"
          },
          {
            "name": "governingTokenMint",
            "docs": [
              "Governing Token Mint the VoterWeightRecord is associated with",
              "Note: The addin can take deposits of any tokens and is not restricted to the community or council tokens only"
            ],
            "type": "pubkey"
          },
          {
            "name": "governingTokenOwner",
            "docs": [
              "The owner of the governing token and voter",
              "This is the actual owner (voter) and corresponds to TokenOwnerRecord.governing_token_owner"
            ],
            "type": "pubkey"
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
                "defined": {
                  "name": "voterWeightAction"
                }
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
              "option": "pubkey"
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
  ]
};
