export type StakeIdl = {
  "metadata": {
    "version": "0.1.4",
    "name": "spl_token_staking",
    "spec": "none"
  },
  "instructions": [
    {
      "name": "initializeStakePool",
      "discriminator": [
        48, 189, 243, 73,
        19,  67,  36, 83
      ],      
      "docs": [
        "Create a [StakePool](state::StakePool) and initialize the Mint that will",
        "represent effective stake weight."
      ],
      "accounts": [
        {
          "name": "payer",
          "writable": true,
          "signer": true,
          "docs": [
            "Payer of rent"
          ]
        },
        {
          "name": "authority",
          "writable": false,
          "signer": false,
          "docs": [
            "Authority that can add rewards pools"
          ]
        },
        {
          "name": "mint",
          "writable": false,
          "signer": false,
          "docs": [
            "SPL Token Mint of the underlying token to be deposited for staking"
          ]
        },
        {
          "name": "stakePool",
          "writable": true,
          "signer": false
        },
        {
          "name": "stakeMint",
          "writable": true,
          "signer": false,
          "docs": [
            "An SPL token Mint for the effective stake weight token"
          ]
        },
        {
          "name": "vault",
          "writable": true,
          "signer": false,
          "docs": [
            "An SPL token Account for staging A tokens"
          ]
        },
        {
          "name": "tokenProgram",
          "writable": false,
          "signer": false
        },
        {
          "name": "rent",
          "writable": false,
          "signer": false
        },
        {
          "name": "systemProgram",
          "writable": false,
          "signer": false
        }
      ],
      "args": [
        {
          "name": "nonce",
          "type": "u8"
        },
        {
          "name": "maxWeight",
          "type": "u64"
        },
        {
          "name": "minDuration",
          "type": "u64"
        },
        {
          "name": "maxDuration",
          "type": "u64"
        }
      ]
    },
    {
      "name": "addRewardPool",
      "discriminator": [
        28,  53, 119,   0,
       114, 211, 196, 239
      ],
      "docs": [
        "Add a [RewardPool](state::RewardPool) to an existing [StakePool](state::StakePool).",
        "",
        "Can only be invoked by the StakePool's authority."
      ],
      "accounts": [
        {
          "name": "payer",
          "writable": true,
          "signer": true,
          "docs": [
            "Payer of rent"
          ]
        },
        {
          "name": "authority",
          "writable": false,
          "signer": true,
          "docs": [
            "Authority of the StakePool"
          ]
        },
        {
          "name": "rewardMint",
          "writable": false,
          "signer": false,
          "docs": [
            "SPL Token Mint of the token that will be distributed as rewards"
          ]
        },
        {
          "name": "stakePool",
          "writable": true,
          "signer": false,
          "docs": [
            "StakePool where the RewardPool will be added"
          ]
        },
        {
          "name": "rewardVault",
          "writable": true,
          "signer": false,
          "docs": [
            "An SPL token Account for holding rewards to be claimed"
          ]
        },
        {
          "name": "tokenProgram",
          "writable": false,
          "signer": false
        },
        {
          "name": "rent",
          "writable": false,
          "signer": false
        },
        {
          "name": "systemProgram",
          "writable": false,
          "signer": false
        }
      ],
      "args": [
        {
          "name": "index",
          "type": "u8"
        }
      ]
    },
    {
      "name": "deposit",
      "discriminator": [
        242,  35, 198, 137,
         82, 225, 242, 182
      ],
      "docs": [
        "Deposit (aka Stake) a wallet's tokens to the specified [StakePool](state::StakePool).",
        "Depending on the `lockup_duration` and the StakePool's weighting configuration, the",
        "wallet initiating the deposit will receive tokens representing their effective stake",
        "(i.e. deposited amount multiplied by the lockup weight).",
        "",
        "For each RewardPool, the latest amount per effective stake will be recalculated to ensure",
        "the latest accumulated rewards are attributed to all previous depositors and not the deposit",
        "resulting from this instruction.",
        "",
        "A [StakeDepositReceipt](state::StakeDepositReceipt) will be created to track the",
        "lockup duration, effective weight, and claimable rewards.",
        "",
        "Remaining accounts are required: pass the `reward_vault` of each reward pool. These must be",
        "passed in the same order as `StakePool.reward_pools`"
      ],
      "accounts": [
        {
          "name": "payer",
          "writable": true,
          "signer": true
        },
        {
          "name": "owner",
          "writable": false,
          "signer": false,
          "docs": [
            "Owner of the StakeDepositReceipt, which may differ",
            "from the account staking."
          ]
        },
        {
          "name": "from",
          "writable": true,
          "signer": false,
          "docs": [
            "Token Account to transfer stake_mint from, to be deposited into the vault"
          ]
        },
        {
          "name": "vault",
          "writable": true,
          "signer": false,
          "docs": [
            "Vault of the StakePool token will be transfer to"
          ]
        },
        {
          "name": "stakeMint",
          "writable": true,
          "signer": false
        },
        {
          "name": "destination",
          "writable": true,
          "signer": false,
          "docs": [
            "Token account the StakePool token will be transfered to"
          ]
        },
        {
          "name": "stakePool",
          "writable": true,
          "signer": false,
          "docs": [
            "StakePool owning the vault that will receive the deposit"
          ]
        },
        {
          "name": "stakeDepositReceipt",
          "writable": true,
          "signer": false
        },
        {
          "name": "tokenProgram",
          "writable": false,
          "signer": false
        },
        {
          "name": "rent",
          "writable": false,
          "signer": false
        },
        {
          "name": "systemProgram",
          "writable": false,
          "signer": false
        }
      ],
      "args": [
        {
          "name": "nonce",
          "type": "u32"
        },
        {
          "name": "amount",
          "type": "u64"
        },
        {
          "name": "lockupDuration",
          "type": "u64"
        }
      ]
    },
    {
      "name": "claimAll",
      "discriminator": [
        194, 194,  80, 194,
        234, 210, 217,  90
      ],
      "docs": [
        "Claim unclaimed rewards from all RewardPools for a specific StakeDepositReceipt.",
        "",
        "For each RewardPool, the latest amount per effective stake will be recalculated to ensure",
        "the latest accumulated rewards are accounted for in the claimable amount. The StakeDepositReceipt",
        "is also updated so that the latest claimed amount is equivalent, so that their claimable amount",
        "is 0 after invoking the claim instruction."
      ],
      "accounts": [
        {
          "name": "claimBase",
          "accounts": [
            {
              "name": "owner",
              "writable": true,
              "signer": true,
              "docs": [
                "Owner of the StakeDepositReceipt"
              ]
            },
            {
              "name": "stakePool",
              "writable": true,
              "signer": false
            },
            {
              "name": "stakeDepositReceipt",
              "writable": true,
              "signer": false,
              "docs": [
                "StakeDepositReceipt of the owner that will be used to claim respective rewards"
              ]
            },
            {
              "name": "tokenProgram",
              "writable": false,
              "signer": false
            }
          ]
        }
      ],
      "args": []
    },
    {
      "name": "withdraw",
      "discriminator": [
        183,  18,  70, 156,
        148, 109, 161,  34
      ],      
      "docs": [
        "Withdraw (aka Unstake) a wallet's tokens for a specific StakeDepositReceipt. The StakePool's",
        "total weighted stake will be decreased by the effective stake amount of the StakeDepositReceipt",
        "and the original amount deposited will be transferred out of the vault.",
        "",
        "All rewards will be claimed. So, for each RewardPool, the latest amount per effective stake will",
        "be recalculated to ensure the latest accumulated rewards are accounted for in the claimable amount.",
        "The StakeDepositReceipt is also updated so that the latest claimed amount is equivalent, so that",
        "their claimable amount is 0 after invoking the withdraw instruction.",
        "",
        "StakeDepositReceipt account is closed after this instruction.",
        "",
        "Remaining accounts are required: pass the `reward_vault` of each reward pool. These must be",
        "passed in the same order as `StakePool.reward_pools`. The owner (the token account which",
        "gains the withdrawn funds) must also be passed be, in pairs like so:",
        "* `<reward_vault[0]><owner[0]>`",
        "* `<reward_vault[1]><owner[1]>",
        "* ...etc"
      ],
      "accounts": [
        {
          "name": "claimBase",
          "accounts": [
            {
              "name": "owner",
              "writable": true,
              "signer": true,
              "docs": [
                "Owner of the StakeDepositReceipt"
              ]
            },
            {
              "name": "stakePool",
              "writable": true,
              "signer": false
            },
            {
              "name": "stakeDepositReceipt",
              "writable": true,
              "signer": false,
              "docs": [
                "StakeDepositReceipt of the owner that will be used to claim respective rewards"
              ]
            },
            {
              "name": "tokenProgram",
              "writable": false,
              "signer": false
            }
          ]
        },
        {
          "name": "vault",
          "writable": true,
          "signer": false,
          "docs": [
            "Vault of the StakePool token will be transferred from"
          ]
        },
        {
          "name": "stakeMint",
          "writable": true,
          "signer": false,
          "docs": [
            "stake_mint of StakePool that will be burned"
          ]
        },
        {
          "name": "from",
          "writable": true,
          "signer": false,
          "docs": [
            "Token Account holding weighted stake representation token to burn"
          ]
        },
        {
          "name": "destination",
          "writable": true,
          "signer": false,
          "docs": [
            "Token account to transfer the previously staked token to"
          ]
        }
      ],
      "args": []
    },
    {
      "name": "updateTokenMeta",
      "discriminator": [
        138,  54,  34,   1,
        233, 180, 193, 240
      ],
      "accounts": [
        {
          "name": "authority",
          "writable": false,
          "signer": true
        },
        {
          "name": "metadataAccount",
          "writable": true,
          "signer": false
        },
        {
          "name": "stakePool",
          "writable": false,
          "signer": false
        },
        {
          "name": "stakeMint",
          "writable": false,
          "signer": false
        },
        {
          "name": "metadataProgram",
          "writable": false,
          "signer": false
        },
        {
          "name": "rent",
          "writable": false,
          "signer": false
        },
        {
          "name": "systemProgram",
          "writable": false,
          "signer": false
        }
      ],
      "args": [
        {
          "name": "name",
          "type": "string"
        },
        {
          "name": "symbol",
          "type": "string"
        },
        {
          "name": "uri",
          "type": "string"
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "lendingMarket",
      "discriminator": [
        246, 114, 50,  98,
        72, 157, 28, 120
      ]
    },
    {
      "name": "stakePool",
      "discriminator": [
        121,  34, 206, 21,
         79, 127, 255, 28
      ]
    },
    {
      "name": "stakeDepositReceipt",
      "discriminator": [
        210, 98, 254, 196,
        151, 68, 235,   0
      ]
          
    },
    {
      "name": "Registrar",
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
    }
  ],
  "types": [
    {
      "name": "lendingMarket",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "version",
            "type": "u8"
          },
          {
            "name": "bump_seed",
            "type": "u8"
          },
          {
            "name": "owner",
            "type": "pubkey"
          },
          {
            "name": "quote_currency",
            "type": {
              "array": [
                "u8",
                32
              ]
            }
          },
          {
            "name": "token_program",
            "type": "pubkey"
          },
          {
            "name": "oracle_id",
            "type": "pubkey"
          }
        ]
      }
    },
    {
      "name": "stakePool",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "authority",
            "docs": [
              "Pubkey that can make updates to StakePool"
            ],
            "type": "pubkey"
          },
          {
            "name": "totalWeightedStake",
            "docs": [
              "Total amount staked that accounts for the lock up period weighting.\n    Note, this is not equal to the amount of SPL Tokens staked."
            ],
            "type": "u128"
          },
          {
            "name": "vault",
            "docs": [
              "Token Account to store the staked SPL Token"
            ],
            "type": "pubkey"
          },
          {
            "name": "mint",
            "docs": [
              "Mint of the token being staked"
            ],
            "type": "pubkey"
          },
          {
            "name": "stakeMint",
            "docs": [
              "Mint of the token representing effective stake"
            ],
            "type": "pubkey"
          },
          {
            "name": "arr",
            "type": {
              "array": [
                {
                  "defined": {
                    "name": "RewardPool"
                  }
                },
                10
              ]
            }
          },
          {
            "name": "baseWeight",
            "docs": [
              "The minimum weight received for staking. In terms of 1 / SCALE_FACTOR_BASE.",
              "Examples:",
              "* `min_weight = 1 x SCALE_FACTOR_BASE` = minmum of 1x multiplier for > min_duration staking",
              "* `min_weight = 2 x SCALE_FACTOR_BASE` = minmum of 2x multiplier for > min_duration staking"
            ],
            "type": "u64"
          },
          {
            "name": "maxWeight",
            "docs": [
              "Maximum weight for staking lockup (i.e. weight multiplier when locked",
              "up for max duration). In terms of 1 / SCALE_FACTOR_BASE. Examples:",
              "* A `max_weight = 1 x SCALE_FACTOR_BASE` = 1x multiplier for max staking duration",
              "* A `max_weight = 2 x SCALE_FACTOR_BASE` = 2x multiplier for max staking duration"
            ],
            "type": "u64"
          },
          {
            "name": "minDuration",
            "docs": [
              "Minimum duration for lockup. At this point, the staker would receive the base weight. In seconds."
            ],
            "type": "u64"
          },
          {
            "name": "maxDuration",
            "docs": [
              "Maximum duration for lockup. At this point, the staker would receive the max weight. In seconds."
            ],
            "type": "u64"
          },
          {
            "name": "nonce",
            "docs": [
              "Nonce to derive multiple stake pools from same mint"
            ],
            "type": "u8"
          },
          {
            "name": "bumpSeed",
            "docs": [
              "Bump seed for stake_mint"
            ],
            "type": "u8"
          },
          {
            "name": "padding0",
            "type": {
              "array": [
                "u8",
                6
              ]
            }
          },
          {
            "name": "reserved",
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
      "name": "stakeDepositReceipt",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "owner",
            "docs": [
              "Pubkey that owns the staked assets"
            ],
            "type": "pubkey"
          },
          {
            "name": "payer",
            "docs": [
              "Pubkey that paid for the deposit"
            ],
            "type": "pubkey"
          },
          {
            "name": "stakePool",
            "docs": [
              "StakePool the deposit is for"
            ],
            "type": "pubkey"
          },
          {
            "name": "lockupDuration",
            "docs": [
              "Duration of the lockup period in seconds"
            ],
            "type": "u64"
          },
          {
            "name": "depositTimestamp",
            "docs": [
              "Timestamp in seconds of when the stake lockup began"
            ],
            "type": "i64"
          },
          {
            "name": "depositAmount",
            "docs": [
              "Amount of SPL token deposited"
            ],
            "type": "u64"
          },
          {
            "name": "effectiveStake",
            "docs": [
              "Amount of stake weighted by lockup duration."
            ],
            "type": "u128"
          },
          {
            "name": "claimedAmount",
            "type": {
              "array": [
                "u128",
                10
              ]
            }
          }
        ]
      }
    },
    {
      "name": "RewardPool",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "rewardVault",
            "docs": [
              "Token Account to store the reward SPL Token"
            ],
            "type": "pubkey"
          },
          {
            "name": "rewardsPerEffectiveStake",
            "docs": [
              "Ever increasing accumulator of the amount of rewards per effective stake.\n    Said another way, if a user deposited before any rewards were added to the\n    `vault`, then this would be the token amount per effective stake they could\n    claim."
            ],
            "type": "u128"
          },
          {
            "name": "lastAmount",
            "docs": [
              "latest amount of tokens in the vault"
            ],
            "type": "u64"
          },
          {
            "name": "padding0",
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
      "name": "Mam",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "field",
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "Registrar",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "address",
            "type": "pubkey"
          },
          {
            "name": "amount",
            "type": "u8"
          },
          {
            "name": "arr",
            "type": {
              "array": [
                {
                  "defined": {
                    "name": "Mam"
                  }
                },
                8
              ]
            }
          }
        ]
      }
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "InvalidAuthority",
      "msg": "Invalid StakePool authority"
    },
    {
      "code": 6001,
      "name": "RewardPoolIndexOccupied",
      "msg": "RewardPool index is already occupied"
    },
    {
      "code": 6002,
      "name": "InvalidStakePoolVault",
      "msg": "StakePool vault is invalid"
    },
    {
      "code": 6003,
      "name": "InvalidRewardPoolVault",
      "msg": "RewardPool vault is invalid"
    },
    {
      "code": 6004,
      "name": "InvalidRewardPoolVaultIndex",
      "msg": "Invalid RewardPool vault remaining account index"
    },
    {
      "code": 6005,
      "name": "InvalidOwner",
      "msg": "Invalid StakeDepositReceiptOwner"
    },
    {
      "code": 6006,
      "name": "InvalidStakePool",
      "msg": "Invalid StakePool"
    },
    {
      "code": 6007,
      "name": "PrecisionMath",
      "msg": "Math precision error"
    },
    {
      "code": 6008,
      "name": "InvalidStakeMint",
      "msg": "Invalid stake mint"
    },
    {
      "code": 6009,
      "name": "StakeStillLocked",
      "msg": "Stake is still locked"
    },
    {
      "code": 6010,
      "name": "InvalidStakePoolDuration",
      "msg": "Max duration must be great than min"
    },
    {
      "code": 6011,
      "name": "InvalidStakePoolWeight",
      "msg": "Max weight must be great than min"
    },
    {
      "code": 6012,
      "name": "DurationTooShort",
      "msg": "Duration too short"
    }
  ],
  "address": "STAKEkKzbdeKkqzKpLkNQD3SUuLgshDKCD7U8duxAbB"
}

export const stakeIdl: StakeIdl = {
  "metadata": {
    "version": "0.1.4",
    "name": "spl_token_staking",
    "spec": "none"
  },
  "instructions": [
    {
      "name": "initializeStakePool",
      "discriminator": [
        48, 189, 243, 73,
        19,  67,  36, 83
      ],      
      "docs": [
        "Create a [StakePool](state::StakePool) and initialize the Mint that will",
        "represent effective stake weight."
      ],
      "accounts": [
        {
          "name": "payer",
          "writable": true,
          "signer": true,
          "docs": [
            "Payer of rent"
          ]
        },
        {
          "name": "authority",
          "writable": false,
          "signer": false,
          "docs": [
            "Authority that can add rewards pools"
          ]
        },
        {
          "name": "mint",
          "writable": false,
          "signer": false,
          "docs": [
            "SPL Token Mint of the underlying token to be deposited for staking"
          ]
        },
        {
          "name": "stakePool",
          "writable": true,
          "signer": false
        },
        {
          "name": "stakeMint",
          "writable": true,
          "signer": false,
          "docs": [
            "An SPL token Mint for the effective stake weight token"
          ]
        },
        {
          "name": "vault",
          "writable": true,
          "signer": false,
          "docs": [
            "An SPL token Account for staging A tokens"
          ]
        },
        {
          "name": "tokenProgram",
          "writable": false,
          "signer": false
        },
        {
          "name": "rent",
          "writable": false,
          "signer": false
        },
        {
          "name": "systemProgram",
          "writable": false,
          "signer": false
        }
      ],
      "args": [
        {
          "name": "nonce",
          "type": "u8"
        },
        {
          "name": "maxWeight",
          "type": "u64"
        },
        {
          "name": "minDuration",
          "type": "u64"
        },
        {
          "name": "maxDuration",
          "type": "u64"
        }
      ]
    },
    {
      "name": "addRewardPool",
      "discriminator": [
        28,  53, 119,   0,
       114, 211, 196, 239
      ],
      "docs": [
        "Add a [RewardPool](state::RewardPool) to an existing [StakePool](state::StakePool).",
        "",
        "Can only be invoked by the StakePool's authority."
      ],
      "accounts": [
        {
          "name": "payer",
          "writable": true,
          "signer": true,
          "docs": [
            "Payer of rent"
          ]
        },
        {
          "name": "authority",
          "writable": false,
          "signer": true,
          "docs": [
            "Authority of the StakePool"
          ]
        },
        {
          "name": "rewardMint",
          "writable": false,
          "signer": false,
          "docs": [
            "SPL Token Mint of the token that will be distributed as rewards"
          ]
        },
        {
          "name": "stakePool",
          "writable": true,
          "signer": false,
          "docs": [
            "StakePool where the RewardPool will be added"
          ]
        },
        {
          "name": "rewardVault",
          "writable": true,
          "signer": false,
          "docs": [
            "An SPL token Account for holding rewards to be claimed"
          ]
        },
        {
          "name": "tokenProgram",
          "writable": false,
          "signer": false
        },
        {
          "name": "rent",
          "writable": false,
          "signer": false
        },
        {
          "name": "systemProgram",
          "writable": false,
          "signer": false
        }
      ],
      "args": [
        {
          "name": "index",
          "type": "u8"
        }
      ]
    },
    {
      "name": "deposit",
      "discriminator": [
        242,  35, 198, 137,
         82, 225, 242, 182
      ],
      "docs": [
        "Deposit (aka Stake) a wallet's tokens to the specified [StakePool](state::StakePool).",
        "Depending on the `lockup_duration` and the StakePool's weighting configuration, the",
        "wallet initiating the deposit will receive tokens representing their effective stake",
        "(i.e. deposited amount multiplied by the lockup weight).",
        "",
        "For each RewardPool, the latest amount per effective stake will be recalculated to ensure",
        "the latest accumulated rewards are attributed to all previous depositors and not the deposit",
        "resulting from this instruction.",
        "",
        "A [StakeDepositReceipt](state::StakeDepositReceipt) will be created to track the",
        "lockup duration, effective weight, and claimable rewards.",
        "",
        "Remaining accounts are required: pass the `reward_vault` of each reward pool. These must be",
        "passed in the same order as `StakePool.reward_pools`"
      ],
      "accounts": [
        {
          "name": "payer",
          "writable": true,
          "signer": true
        },
        {
          "name": "owner",
          "writable": false,
          "signer": false,
          "docs": [
            "Owner of the StakeDepositReceipt, which may differ",
            "from the account staking."
          ]
        },
        {
          "name": "from",
          "writable": true,
          "signer": false,
          "docs": [
            "Token Account to transfer stake_mint from, to be deposited into the vault"
          ]
        },
        {
          "name": "vault",
          "writable": true,
          "signer": false,
          "docs": [
            "Vault of the StakePool token will be transfer to"
          ]
        },
        {
          "name": "stakeMint",
          "writable": true,
          "signer": false
        },
        {
          "name": "destination",
          "writable": true,
          "signer": false,
          "docs": [
            "Token account the StakePool token will be transfered to"
          ]
        },
        {
          "name": "stakePool",
          "writable": true,
          "signer": false,
          "docs": [
            "StakePool owning the vault that will receive the deposit"
          ]
        },
        {
          "name": "stakeDepositReceipt",
          "writable": true,
          "signer": false
        },
        {
          "name": "tokenProgram",
          "writable": false,
          "signer": false
        },
        {
          "name": "rent",
          "writable": false,
          "signer": false
        },
        {
          "name": "systemProgram",
          "writable": false,
          "signer": false
        }
      ],
      "args": [
        {
          "name": "nonce",
          "type": "u32"
        },
        {
          "name": "amount",
          "type": "u64"
        },
        {
          "name": "lockupDuration",
          "type": "u64"
        }
      ]
    },
    {
      "name": "claimAll",
      "discriminator": [
        194, 194,  80, 194,
        234, 210, 217,  90
      ],
      "docs": [
        "Claim unclaimed rewards from all RewardPools for a specific StakeDepositReceipt.",
        "",
        "For each RewardPool, the latest amount per effective stake will be recalculated to ensure",
        "the latest accumulated rewards are accounted for in the claimable amount. The StakeDepositReceipt",
        "is also updated so that the latest claimed amount is equivalent, so that their claimable amount",
        "is 0 after invoking the claim instruction."
      ],
      "accounts": [
        {
          "name": "claimBase",
          "accounts": [
            {
              "name": "owner",
              "writable": true,
              "signer": true,
              "docs": [
                "Owner of the StakeDepositReceipt"
              ]
            },
            {
              "name": "stakePool",
              "writable": true,
              "signer": false
            },
            {
              "name": "stakeDepositReceipt",
              "writable": true,
              "signer": false,
              "docs": [
                "StakeDepositReceipt of the owner that will be used to claim respective rewards"
              ]
            },
            {
              "name": "tokenProgram",
              "writable": false,
              "signer": false
            }
          ]
        }
      ],
      "args": []
    },
    {
      "name": "withdraw",
      "discriminator": [
        183,  18,  70, 156,
        148, 109, 161,  34
      ],      
      "docs": [
        "Withdraw (aka Unstake) a wallet's tokens for a specific StakeDepositReceipt. The StakePool's",
        "total weighted stake will be decreased by the effective stake amount of the StakeDepositReceipt",
        "and the original amount deposited will be transferred out of the vault.",
        "",
        "All rewards will be claimed. So, for each RewardPool, the latest amount per effective stake will",
        "be recalculated to ensure the latest accumulated rewards are accounted for in the claimable amount.",
        "The StakeDepositReceipt is also updated so that the latest claimed amount is equivalent, so that",
        "their claimable amount is 0 after invoking the withdraw instruction.",
        "",
        "StakeDepositReceipt account is closed after this instruction.",
        "",
        "Remaining accounts are required: pass the `reward_vault` of each reward pool. These must be",
        "passed in the same order as `StakePool.reward_pools`. The owner (the token account which",
        "gains the withdrawn funds) must also be passed be, in pairs like so:",
        "* `<reward_vault[0]><owner[0]>`",
        "* `<reward_vault[1]><owner[1]>",
        "* ...etc"
      ],
      "accounts": [
        {
          "name": "claimBase",
          "accounts": [
            {
              "name": "owner",
              "writable": true,
              "signer": true,
              "docs": [
                "Owner of the StakeDepositReceipt"
              ]
            },
            {
              "name": "stakePool",
              "writable": true,
              "signer": false
            },
            {
              "name": "stakeDepositReceipt",
              "writable": true,
              "signer": false,
              "docs": [
                "StakeDepositReceipt of the owner that will be used to claim respective rewards"
              ]
            },
            {
              "name": "tokenProgram",
              "writable": false,
              "signer": false
            }
          ]
        },
        {
          "name": "vault",
          "writable": true,
          "signer": false,
          "docs": [
            "Vault of the StakePool token will be transferred from"
          ]
        },
        {
          "name": "stakeMint",
          "writable": true,
          "signer": false,
          "docs": [
            "stake_mint of StakePool that will be burned"
          ]
        },
        {
          "name": "from",
          "writable": true,
          "signer": false,
          "docs": [
            "Token Account holding weighted stake representation token to burn"
          ]
        },
        {
          "name": "destination",
          "writable": true,
          "signer": false,
          "docs": [
            "Token account to transfer the previously staked token to"
          ]
        }
      ],
      "args": []
    },
    {
      "name": "updateTokenMeta",
      "discriminator": [
        138,  54,  34,   1,
        233, 180, 193, 240
      ],
      "accounts": [
        {
          "name": "authority",
          "writable": false,
          "signer": true
        },
        {
          "name": "metadataAccount",
          "writable": true,
          "signer": false
        },
        {
          "name": "stakePool",
          "writable": false,
          "signer": false
        },
        {
          "name": "stakeMint",
          "writable": false,
          "signer": false
        },
        {
          "name": "metadataProgram",
          "writable": false,
          "signer": false
        },
        {
          "name": "rent",
          "writable": false,
          "signer": false
        },
        {
          "name": "systemProgram",
          "writable": false,
          "signer": false
        }
      ],
      "args": [
        {
          "name": "name",
          "type": "string"
        },
        {
          "name": "symbol",
          "type": "string"
        },
        {
          "name": "uri",
          "type": "string"
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "lendingMarket",
      "discriminator": [
        246, 114, 50,  98,
        72, 157, 28, 120
      ]
    },
    {
      "name": "stakePool",
      "discriminator": [
        121,  34, 206, 21,
         79, 127, 255, 28
      ]
    },
    {
      "name": "stakeDepositReceipt",
      "discriminator": [
        210, 98, 254, 196,
        151, 68, 235,   0
      ]
          
    },
    {
      "name": "Registrar",
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
    }
  ],
  "types": [
    {
      "name": "lendingMarket",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "version",
            "type": "u8"
          },
          {
            "name": "bump_seed",
            "type": "u8"
          },
          {
            "name": "owner",
            "type": "pubkey"
          },
          {
            "name": "quote_currency",
            "type": {
              "array": [
                "u8",
                32
              ]
            }
          },
          {
            "name": "token_program",
            "type": "pubkey"
          },
          {
            "name": "oracle_id",
            "type": "pubkey"
          }
        ]
      }
    },
    {
      "name": "stakePool",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "authority",
            "docs": [
              "Pubkey that can make updates to StakePool"
            ],
            "type": "pubkey"
          },
          {
            "name": "totalWeightedStake",
            "docs": [
              "Total amount staked that accounts for the lock up period weighting.\n    Note, this is not equal to the amount of SPL Tokens staked."
            ],
            "type": "u128"
          },
          {
            "name": "vault",
            "docs": [
              "Token Account to store the staked SPL Token"
            ],
            "type": "pubkey"
          },
          {
            "name": "mint",
            "docs": [
              "Mint of the token being staked"
            ],
            "type": "pubkey"
          },
          {
            "name": "stakeMint",
            "docs": [
              "Mint of the token representing effective stake"
            ],
            "type": "pubkey"
          },
          {
            "name": "arr",
            "type": {
              "array": [
                {
                  "defined": {
                    "name": "RewardPool"
                  }
                },
                10
              ]
            }
          },
          {
            "name": "baseWeight",
            "docs": [
              "The minimum weight received for staking. In terms of 1 / SCALE_FACTOR_BASE.",
              "Examples:",
              "* `min_weight = 1 x SCALE_FACTOR_BASE` = minmum of 1x multiplier for > min_duration staking",
              "* `min_weight = 2 x SCALE_FACTOR_BASE` = minmum of 2x multiplier for > min_duration staking"
            ],
            "type": "u64"
          },
          {
            "name": "maxWeight",
            "docs": [
              "Maximum weight for staking lockup (i.e. weight multiplier when locked",
              "up for max duration). In terms of 1 / SCALE_FACTOR_BASE. Examples:",
              "* A `max_weight = 1 x SCALE_FACTOR_BASE` = 1x multiplier for max staking duration",
              "* A `max_weight = 2 x SCALE_FACTOR_BASE` = 2x multiplier for max staking duration"
            ],
            "type": "u64"
          },
          {
            "name": "minDuration",
            "docs": [
              "Minimum duration for lockup. At this point, the staker would receive the base weight. In seconds."
            ],
            "type": "u64"
          },
          {
            "name": "maxDuration",
            "docs": [
              "Maximum duration for lockup. At this point, the staker would receive the max weight. In seconds."
            ],
            "type": "u64"
          },
          {
            "name": "nonce",
            "docs": [
              "Nonce to derive multiple stake pools from same mint"
            ],
            "type": "u8"
          },
          {
            "name": "bumpSeed",
            "docs": [
              "Bump seed for stake_mint"
            ],
            "type": "u8"
          },
          {
            "name": "padding0",
            "type": {
              "array": [
                "u8",
                6
              ]
            }
          },
          {
            "name": "reserved",
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
      "name": "stakeDepositReceipt",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "owner",
            "docs": [
              "Pubkey that owns the staked assets"
            ],
            "type": "pubkey"
          },
          {
            "name": "payer",
            "docs": [
              "Pubkey that paid for the deposit"
            ],
            "type": "pubkey"
          },
          {
            "name": "stakePool",
            "docs": [
              "StakePool the deposit is for"
            ],
            "type": "pubkey"
          },
          {
            "name": "lockupDuration",
            "docs": [
              "Duration of the lockup period in seconds"
            ],
            "type": "u64"
          },
          {
            "name": "depositTimestamp",
            "docs": [
              "Timestamp in seconds of when the stake lockup began"
            ],
            "type": "i64"
          },
          {
            "name": "depositAmount",
            "docs": [
              "Amount of SPL token deposited"
            ],
            "type": "u64"
          },
          {
            "name": "effectiveStake",
            "docs": [
              "Amount of stake weighted by lockup duration."
            ],
            "type": "u128"
          },
          {
            "name": "claimedAmount",
            "type": {
              "array": [
                "u128",
                10
              ]
            }
          }
        ]
      }
    },
    {
      "name": "RewardPool",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "rewardVault",
            "docs": [
              "Token Account to store the reward SPL Token"
            ],
            "type": "pubkey"
          },
          {
            "name": "rewardsPerEffectiveStake",
            "docs": [
              "Ever increasing accumulator of the amount of rewards per effective stake.\n    Said another way, if a user deposited before any rewards were added to the\n    `vault`, then this would be the token amount per effective stake they could\n    claim."
            ],
            "type": "u128"
          },
          {
            "name": "lastAmount",
            "docs": [
              "latest amount of tokens in the vault"
            ],
            "type": "u64"
          },
          {
            "name": "padding0",
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
      "name": "Mam",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "field",
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "Registrar",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "address",
            "type": "pubkey"
          },
          {
            "name": "amount",
            "type": "u8"
          },
          {
            "name": "arr",
            "type": {
              "array": [
                {
                  "defined": {
                    "name": "Mam"
                  }
                },
                8
              ]
            }
          }
        ]
      }
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "InvalidAuthority",
      "msg": "Invalid StakePool authority"
    },
    {
      "code": 6001,
      "name": "RewardPoolIndexOccupied",
      "msg": "RewardPool index is already occupied"
    },
    {
      "code": 6002,
      "name": "InvalidStakePoolVault",
      "msg": "StakePool vault is invalid"
    },
    {
      "code": 6003,
      "name": "InvalidRewardPoolVault",
      "msg": "RewardPool vault is invalid"
    },
    {
      "code": 6004,
      "name": "InvalidRewardPoolVaultIndex",
      "msg": "Invalid RewardPool vault remaining account index"
    },
    {
      "code": 6005,
      "name": "InvalidOwner",
      "msg": "Invalid StakeDepositReceiptOwner"
    },
    {
      "code": 6006,
      "name": "InvalidStakePool",
      "msg": "Invalid StakePool"
    },
    {
      "code": 6007,
      "name": "PrecisionMath",
      "msg": "Math precision error"
    },
    {
      "code": 6008,
      "name": "InvalidStakeMint",
      "msg": "Invalid stake mint"
    },
    {
      "code": 6009,
      "name": "StakeStillLocked",
      "msg": "Stake is still locked"
    },
    {
      "code": 6010,
      "name": "InvalidStakePoolDuration",
      "msg": "Max duration must be great than min"
    },
    {
      "code": 6011,
      "name": "InvalidStakePoolWeight",
      "msg": "Max weight must be great than min"
    },
    {
      "code": 6012,
      "name": "DurationTooShort",
      "msg": "Duration too short"
    }
  ],
  "address": "STAKEkKzbdeKkqzKpLkNQD3SUuLgshDKCD7U8duxAbB"
}