export type NftVoter = {
  "version": "0.1.1",
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
      "name": "relinquishNftVote",
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
          "name": "governance",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "proposal",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "governingTokenOwner",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "voteRecord",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "beneficiary",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": []
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
          "type": "u64"
        },
        {
          "name": "size",
          "type": "u32"
        }
      ]
    },
    {
      "name": "castNftVote",
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
          "name": "governingTokenOwner",
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
          "name": "proposal",
          "type": "publicKey"
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "nftVoteRecord",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "proposal",
            "type": "publicKey"
          },
          {
            "name": "nftMint",
            "type": "publicKey"
          },
          {
            "name": "governingTokenOwner",
            "type": "publicKey"
          }
        ]
      }
    },
    {
      "name": "maxVoterWeightRecord",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "realm",
            "type": "publicKey"
          },
          {
            "name": "governingTokenMint",
            "type": "publicKey"
          },
          {
            "name": "maxVoterWeight",
            "type": "u64"
          },
          {
            "name": "maxVoterWeightExpiry",
            "type": {
              "option": "u64"
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
    },
    {
      "name": "voterWeightRecord",
      "type": {
        "kind": "struct",
        "fields": [
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
          },
          {
            "name": "voterWeight",
            "type": "u64"
          },
          {
            "name": "voterWeightExpiry",
            "type": {
              "option": "u64"
            }
          },
          {
            "name": "weightAction",
            "type": {
              "option": {
                "defined": "VoterWeightAction"
              }
            }
          },
          {
            "name": "weightActionTarget",
            "type": {
              "option": "publicKey"
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
    }
  ],
  "types": [
    {
      "name": "CollectionConfig",
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
            "type": "u64"
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
      "name": "VoterWeightAction",
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
      "msg": "Invalid Realm Authority"
    },
    {
      "code": 6001,
      "name": "InvalidRealmForRegistrar",
      "msg": "Invalid Realm for Registrar"
    },
    {
      "code": 6002,
      "name": "InvalidCollectionSize",
      "msg": "Invalid Collection Size"
    },
    {
      "code": 6003,
      "name": "InvalidMaxVoterWeightRecordRealm",
      "msg": "Invalid MaxVoterWeightRecord Realm"
    },
    {
      "code": 6004,
      "name": "InvalidMaxVoterWeightRecordMint",
      "msg": "Invalid MaxVoterWeightRecord Mint"
    },
    {
      "code": 6005,
      "name": "CastVoteIsNotAllowed",
      "msg": "CastVote Is Not Allowed"
    },
    {
      "code": 6006,
      "name": "InvalidVoterWeightRecordRealm",
      "msg": "Invalid VoterWeightRecord Realm"
    },
    {
      "code": 6007,
      "name": "InvalidVoterWeightRecordMint",
      "msg": "Invalid VoterWeightRecord Mint"
    },
    {
      "code": 6008,
      "name": "InvalidTokenOwnerForVoterWeightRecord",
      "msg": "Invalid TokenOwner for VoterWeightRecord"
    },
    {
      "code": 6009,
      "name": "CollectionMustBeVerified",
      "msg": "Collection must be verified"
    },
    {
      "code": 6010,
      "name": "VoterDoesNotOwnNft",
      "msg": "Voter does not own NFT"
    },
    {
      "code": 6011,
      "name": "CollectionNotFound",
      "msg": "Collection not found"
    },
    {
      "code": 6012,
      "name": "MissingMetadataCollection",
      "msg": "Missing Metadata collection"
    },
    {
      "code": 6013,
      "name": "TokenMetadataDoesNotMatch",
      "msg": "Token Metadata doesn't match"
    },
    {
      "code": 6014,
      "name": "InvalidAccountOwner",
      "msg": "Invalid account owner"
    },
    {
      "code": 6015,
      "name": "InvalidTokenMetadataAccount",
      "msg": "Invalid token metadata account"
    },
    {
      "code": 6016,
      "name": "DuplicatedNftDetected",
      "msg": "Duplicated NFT detected"
    },
    {
      "code": 6017,
      "name": "InvalidNftAmount",
      "msg": "Invalid NFT amount"
    },
    {
      "code": 6018,
      "name": "NftAlreadyVoted",
      "msg": "NFT already voted"
    },
    {
      "code": 6019,
      "name": "InvalidProposalForNftVoteRecord",
      "msg": "Invalid Proposal for NftVoteRecord"
    },
    {
      "code": 6020,
      "name": "InvalidTokenOwnerForNftVoteRecord",
      "msg": "Invalid TokenOwner for NftVoteRecord"
    },
    {
      "code": 6021,
      "name": "VoteRecordMustBeWithdrawn",
      "msg": "VoteRecord must be withdrawn"
    },
    {
      "code": 6022,
      "name": "InvalidVoteRecordForNftVoteRecord",
      "msg": "Invalid VoteRecord for NftVoteRecord"
    },
    {
      "code": 6023,
      "name": "VoterWeightRecordMustBeExpired",
      "msg": "VoterWeightRecord must be expired"
    },
    {
      "code": 6024,
      "name": "CannotConfigureCollectionWithVotingProposals",
      "msg": "Cannot configure collection with voting proposals"
    }
  ]
};

export const IDL: NftVoter = {
  "version": "0.1.1",
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
      "name": "relinquishNftVote",
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
          "name": "governance",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "proposal",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "governingTokenOwner",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "voteRecord",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "beneficiary",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": []
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
          "type": "u64"
        },
        {
          "name": "size",
          "type": "u32"
        }
      ]
    },
    {
      "name": "castNftVote",
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
          "name": "governingTokenOwner",
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
          "name": "proposal",
          "type": "publicKey"
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "nftVoteRecord",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "proposal",
            "type": "publicKey"
          },
          {
            "name": "nftMint",
            "type": "publicKey"
          },
          {
            "name": "governingTokenOwner",
            "type": "publicKey"
          }
        ]
      }
    },
    {
      "name": "maxVoterWeightRecord",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "realm",
            "type": "publicKey"
          },
          {
            "name": "governingTokenMint",
            "type": "publicKey"
          },
          {
            "name": "maxVoterWeight",
            "type": "u64"
          },
          {
            "name": "maxVoterWeightExpiry",
            "type": {
              "option": "u64"
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
    },
    {
      "name": "voterWeightRecord",
      "type": {
        "kind": "struct",
        "fields": [
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
          },
          {
            "name": "voterWeight",
            "type": "u64"
          },
          {
            "name": "voterWeightExpiry",
            "type": {
              "option": "u64"
            }
          },
          {
            "name": "weightAction",
            "type": {
              "option": {
                "defined": "VoterWeightAction"
              }
            }
          },
          {
            "name": "weightActionTarget",
            "type": {
              "option": "publicKey"
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
    }
  ],
  "types": [
    {
      "name": "CollectionConfig",
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
            "type": "u64"
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
      "name": "VoterWeightAction",
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
      "msg": "Invalid Realm Authority"
    },
    {
      "code": 6001,
      "name": "InvalidRealmForRegistrar",
      "msg": "Invalid Realm for Registrar"
    },
    {
      "code": 6002,
      "name": "InvalidCollectionSize",
      "msg": "Invalid Collection Size"
    },
    {
      "code": 6003,
      "name": "InvalidMaxVoterWeightRecordRealm",
      "msg": "Invalid MaxVoterWeightRecord Realm"
    },
    {
      "code": 6004,
      "name": "InvalidMaxVoterWeightRecordMint",
      "msg": "Invalid MaxVoterWeightRecord Mint"
    },
    {
      "code": 6005,
      "name": "CastVoteIsNotAllowed",
      "msg": "CastVote Is Not Allowed"
    },
    {
      "code": 6006,
      "name": "InvalidVoterWeightRecordRealm",
      "msg": "Invalid VoterWeightRecord Realm"
    },
    {
      "code": 6007,
      "name": "InvalidVoterWeightRecordMint",
      "msg": "Invalid VoterWeightRecord Mint"
    },
    {
      "code": 6008,
      "name": "InvalidTokenOwnerForVoterWeightRecord",
      "msg": "Invalid TokenOwner for VoterWeightRecord"
    },
    {
      "code": 6009,
      "name": "CollectionMustBeVerified",
      "msg": "Collection must be verified"
    },
    {
      "code": 6010,
      "name": "VoterDoesNotOwnNft",
      "msg": "Voter does not own NFT"
    },
    {
      "code": 6011,
      "name": "CollectionNotFound",
      "msg": "Collection not found"
    },
    {
      "code": 6012,
      "name": "MissingMetadataCollection",
      "msg": "Missing Metadata collection"
    },
    {
      "code": 6013,
      "name": "TokenMetadataDoesNotMatch",
      "msg": "Token Metadata doesn't match"
    },
    {
      "code": 6014,
      "name": "InvalidAccountOwner",
      "msg": "Invalid account owner"
    },
    {
      "code": 6015,
      "name": "InvalidTokenMetadataAccount",
      "msg": "Invalid token metadata account"
    },
    {
      "code": 6016,
      "name": "DuplicatedNftDetected",
      "msg": "Duplicated NFT detected"
    },
    {
      "code": 6017,
      "name": "InvalidNftAmount",
      "msg": "Invalid NFT amount"
    },
    {
      "code": 6018,
      "name": "NftAlreadyVoted",
      "msg": "NFT already voted"
    },
    {
      "code": 6019,
      "name": "InvalidProposalForNftVoteRecord",
      "msg": "Invalid Proposal for NftVoteRecord"
    },
    {
      "code": 6020,
      "name": "InvalidTokenOwnerForNftVoteRecord",
      "msg": "Invalid TokenOwner for NftVoteRecord"
    },
    {
      "code": 6021,
      "name": "VoteRecordMustBeWithdrawn",
      "msg": "VoteRecord must be withdrawn"
    },
    {
      "code": 6022,
      "name": "InvalidVoteRecordForNftVoteRecord",
      "msg": "Invalid VoteRecord for NftVoteRecord"
    },
    {
      "code": 6023,
      "name": "VoterWeightRecordMustBeExpired",
      "msg": "VoterWeightRecord must be expired"
    },
    {
      "code": 6024,
      "name": "CannotConfigureCollectionWithVotingProposals",
      "msg": "Cannot configure collection with voting proposals"
    }
  ]
};
