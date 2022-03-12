export type NftVoter = {
  "version": "0.0.0",
  "name": "nft_voter",
  "instructions": [
    {
      "name": "createRegistrar",
      "accounts": [
        {
          "name": "registrar",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "governanceProgramId",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "realm",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "governingTokenMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "realmAuthority",
          "isMut": false,
          "isSigner": true
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
          "name": "maxCollections",
          "type": "u8"
        }
      ]
    },
    {
      "name": "createVoterWeightRecord",
      "accounts": [
        {
          "name": "voterWeightRecord",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "governanceProgramId",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "realm",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "realmGoverningTokenMint",
          "isMut": false,
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
          "isSigner": false
        },
        {
          "name": "realm",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "realmGoverningTokenMint",
          "isMut": false,
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
      "args": []
    },
    {
      "name": "updateVoterWeightRecord",
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
        }
      ],
      "args": [
        {
          "name": "voterWeightAction",
          "type": {
            "defined": "VoterWeightAction"
          }
        }
      ]
    },
    {
      "name": "relinquishVote",
      "accounts": [
        {
          "name": "registrar",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "realm",
          "type": "publicKey"
        },
        {
          "name": "governingTokenMint",
          "type": "publicKey"
        },
        {
          "name": "governingTokenOwner",
          "type": "publicKey"
        }
      ]
    },
    {
      "name": "configureCollection",
      "accounts": [
        {
          "name": "registrar",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "realm",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "realmAuthority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "collection",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "maxVoterWeightRecord",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "weight",
          "type": "u16"
        },
        {
          "name": "size",
          "type": "u32"
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "collectionConfig",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "collection",
            "type": "publicKey"
          },
          {
            "name": "size",
            "type": "u32"
          },
          {
            "name": "weight",
            "type": "u16"
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
      "name": "proposalNftVote",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "proposal",
            "type": "publicKey"
          },
          {
            "name": "nft",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "registrar",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "governanceProgramId",
            "type": "publicKey"
          },
          {
            "name": "realm",
            "type": "publicKey"
          },
          {
            "name": "governingTokenMint",
            "type": "publicKey"
          },
          {
            "name": "collectionConfigs",
            "type": {
              "vec": {
                "defined": "CollectionConfig"
              }
            }
          },
          {
            "name": "reserved",
            "type": {
              "array": [
                "u8",
                128
              ]
            }
          }
        ]
      }
    }
  ],
  "types": [
    {
      "name": "NftVoterError",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "InvalidRealmAuthority"
          },
          {
            "name": "InvalidRegistrarRealm"
          },
          {
            "name": "InvalidCollectionSize"
          },
          {
            "name": "InvalidMaxVoterWeightRecordRealm"
          },
          {
            "name": "InvalidMaxVoterWeightRecordMint"
          },
          {
            "name": "CastVoteIsNotAllowed"
          },
          {
            "name": "InvalidVoterWeightRecordRealm"
          },
          {
            "name": "InvalidVoterWeightRecordMint"
          },
          {
            "name": "CollectionMustBeVerified"
          },
          {
            "name": "VoterDoesNotOwnNft"
          },
          {
            "name": "CollectionNotFound"
          },
          {
            "name": "TokenMetadataDoesNotMatch"
          },
          {
            "name": "InvalidAccountOwner"
          }
        ]
      }
    }
  ]
};

export const IDL: NftVoter = {
  "version": "0.0.0",
  "name": "nft_voter",
  "instructions": [
    {
      "name": "createRegistrar",
      "accounts": [
        {
          "name": "registrar",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "governanceProgramId",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "realm",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "governingTokenMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "realmAuthority",
          "isMut": false,
          "isSigner": true
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
          "name": "maxCollections",
          "type": "u8"
        }
      ]
    },
    {
      "name": "createVoterWeightRecord",
      "accounts": [
        {
          "name": "voterWeightRecord",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "governanceProgramId",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "realm",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "realmGoverningTokenMint",
          "isMut": false,
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
          "isSigner": false
        },
        {
          "name": "realm",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "realmGoverningTokenMint",
          "isMut": false,
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
      "args": []
    },
    {
      "name": "updateVoterWeightRecord",
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
        }
      ],
      "args": [
        {
          "name": "voterWeightAction",
          "type": {
            "defined": "VoterWeightAction"
          }
        }
      ]
    },
    {
      "name": "relinquishVote",
      "accounts": [
        {
          "name": "registrar",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "realm",
          "type": "publicKey"
        },
        {
          "name": "governingTokenMint",
          "type": "publicKey"
        },
        {
          "name": "governingTokenOwner",
          "type": "publicKey"
        }
      ]
    },
    {
      "name": "configureCollection",
      "accounts": [
        {
          "name": "registrar",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "realm",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "realmAuthority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "collection",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "maxVoterWeightRecord",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "weight",
          "type": "u16"
        },
        {
          "name": "size",
          "type": "u32"
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "collectionConfig",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "collection",
            "type": "publicKey"
          },
          {
            "name": "size",
            "type": "u32"
          },
          {
            "name": "weight",
            "type": "u16"
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
      "name": "proposalNftVote",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "proposal",
            "type": "publicKey"
          },
          {
            "name": "nft",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "registrar",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "governanceProgramId",
            "type": "publicKey"
          },
          {
            "name": "realm",
            "type": "publicKey"
          },
          {
            "name": "governingTokenMint",
            "type": "publicKey"
          },
          {
            "name": "collectionConfigs",
            "type": {
              "vec": {
                "defined": "CollectionConfig"
              }
            }
          },
          {
            "name": "reserved",
            "type": {
              "array": [
                "u8",
                128
              ]
            }
          }
        ]
      }
    }
  ],
  "types": [
    {
      "name": "NftVoterError",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "InvalidRealmAuthority"
          },
          {
            "name": "InvalidRegistrarRealm"
          },
          {
            "name": "InvalidCollectionSize"
          },
          {
            "name": "InvalidMaxVoterWeightRecordRealm"
          },
          {
            "name": "InvalidMaxVoterWeightRecordMint"
          },
          {
            "name": "CastVoteIsNotAllowed"
          },
          {
            "name": "InvalidVoterWeightRecordRealm"
          },
          {
            "name": "InvalidVoterWeightRecordMint"
          },
          {
            "name": "CollectionMustBeVerified"
          },
          {
            "name": "VoterDoesNotOwnNft"
          },
          {
            "name": "CollectionNotFound"
          },
          {
            "name": "TokenMetadataDoesNotMatch"
          },
          {
            "name": "InvalidAccountOwner"
          }
        ]
      }
    }
  ]
};
