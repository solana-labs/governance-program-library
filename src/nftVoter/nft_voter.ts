/**
 * Program IDL in camelCase format in order to be used in JS/TS.
 *
 * Note that this is only a type helper and is not the actual IDL. The original
 * IDL can be found at `target/idl/nft_voter.json`.
 */
export type NftVoter = {
  "address": "GnftV5kLjd67tvHpNGyodwWveEKivz3ZWvvE3Z4xi2iw",
  "metadata": {
    "name": "nftVoter",
    "version": "0.2.2",
    "spec": "0.1.0",
    "description": "SPL Governance addin implementing NFT based governance"
  },
  "instructions": [
    {
      "name": "castNftVote",
      "discriminator": [
        150,
        177,
        73,
        223,
        30,
        12,
        172,
        125
      ],
      "accounts": [
        {
          "name": "registrar",
          "docs": [
            "The NFT voting registrar"
          ]
        },
        {
          "name": "voterWeightRecord",
          "writable": true
        },
        {
          "name": "voterTokenOwnerRecord",
          "docs": [
            "TokenOwnerRecord of the voter who casts the vote"
          ]
        },
        {
          "name": "voterAuthority",
          "docs": [
            "Authority of the voter who casts the vote",
            "It can be either governing_token_owner or its delegate and must sign this instruction"
          ],
          "signer": true
        },
        {
          "name": "payer",
          "docs": [
            "The account which pays for the transaction"
          ],
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
          "name": "proposal",
          "type": "pubkey"
        }
      ]
    },
    {
      "name": "configureCollection",
      "discriminator": [
        71,
        128,
        33,
        233,
        71,
        167,
        155,
        164
      ],
      "accounts": [
        {
          "name": "registrar",
          "docs": [
            "Registrar for which we configure this Collection"
          ],
          "writable": true
        },
        {
          "name": "realm"
        },
        {
          "name": "realmAuthority",
          "docs": [
            "Authority of the Realm must sign and match Realm.authority"
          ],
          "signer": true
        },
        {
          "name": "collection"
        },
        {
          "name": "maxVoterWeightRecord",
          "writable": true
        }
      ],
      "args": [
        {
          "name": "weight",
          "type": "u64"
        },
        {
          "name": "size",
          "type": "u32"
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
                "path": "realm"
              },
              {
                "kind": "account",
                "path": "realmGoverningTokenMint"
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
          "name": "realm"
        },
        {
          "name": "realmGoverningTokenMint",
          "docs": [
            "Either the realm community mint or the council mint."
          ]
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
            "The NFT voting Registrar",
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
            "Note: Once the NFT plugin is enabled the governing_token_mint is used only as identity",
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
          "name": "maxCollections",
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
                "path": "realm"
              },
              {
                "kind": "account",
                "path": "realmGoverningTokenMint"
              },
              {
                "kind": "arg",
                "path": "governingTokenOwner"
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
          "name": "realm"
        },
        {
          "name": "realmGoverningTokenMint",
          "docs": [
            "Either the realm community mint or the council mint."
          ]
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
      "name": "relinquishNftVote",
      "discriminator": [
        180,
        111,
        224,
        230,
        204,
        199,
        66,
        66
      ],
      "accounts": [
        {
          "name": "registrar",
          "docs": [
            "The NFT voting Registrar"
          ]
        },
        {
          "name": "voterWeightRecord",
          "writable": true
        },
        {
          "name": "governance",
          "docs": [
            "Governance account the Proposal is for"
          ]
        },
        {
          "name": "proposal"
        },
        {
          "name": "voterTokenOwnerRecord",
          "docs": [
            "TokenOwnerRecord of the voter who cast the original vote"
          ]
        },
        {
          "name": "voterAuthority",
          "docs": [
            "Authority of the voter who cast the original vote",
            "It can be either governing_token_owner or its delegate and must sign this instruction"
          ],
          "signer": true
        },
        {
          "name": "voteRecord",
          "docs": [
            "The account is used to validate that it doesn't exist and if it doesn't then Anchor owner check throws error",
            "The check is disabled here and performed inside the instruction",
            "#[account(owner = registrar.governance_program_id)]"
          ]
        },
        {
          "name": "beneficiary",
          "writable": true
        }
      ],
      "args": []
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
            "The NFT voting Registrar"
          ]
        },
        {
          "name": "voterWeightRecord",
          "writable": true
        }
      ],
      "args": [
        {
          "name": "voterWeightAction",
          "type": {
            "defined": {
              "name": "voterWeightAction"
            }
          }
        }
      ]
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
      "name": "collectionConfig",
      "docs": [
        "Configuration of an NFT collection used for governance power"
      ],
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "collection",
            "docs": [
              "The NFT collection used for governance"
            ],
            "type": "pubkey"
          },
          {
            "name": "size",
            "docs": [
              "The size of the NFT collection used to calculate max voter weight",
              "Note: At the moment the size is not captured on Metaplex accounts",
              "and it has to be manually updated on the Registrar"
            ],
            "type": "u32"
          },
          {
            "name": "weight",
            "docs": [
              "Governance power weight of the collection",
              "Each NFT in the collection has governance power = 1 * weight",
              "Note: The weight is scaled accordingly to the governing_token_mint decimals",
              "Ex: if the the mint has 2 decimal places then weight of 1 should be stored as 100"
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
        "Registrar which stores NFT voting configuration for the given Realm"
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
              "When the plugin is used the mint is only used as identity of the governing power (voting population)",
              "and the actual token of the mint is not used"
            ],
            "type": "pubkey"
          },
          {
            "name": "collectionConfigs",
            "docs": [
              "MPL Collection used for voting"
            ],
            "type": {
              "vec": {
                "defined": {
                  "name": "collectionConfig"
                }
              }
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
