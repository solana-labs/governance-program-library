export type RealmVoter = {
  "version": "0.0.1",
  "name": "realm_voter",
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
          "name": "maxGovernancePrograms",
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
        },
        {
          "name": "tokenOwnerRecord",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "configureVoterWeights",
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
          "name": "maxVoterWeightRecord",
          "isMut": true,
          "isSigner": false
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
      "name": "configureGovernanceProgram",
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
          "name": "governanceProgramId",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    }
  ],
  "accounts": [
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
            "name": "governanceProgramConfigs",
            "type": {
              "vec": {
                "defined": "GovernanceProgramConfig"
              }
            }
          },
          {
            "name": "realmMemberVoterWeight",
            "type": "u64"
          },
          {
            "name": "maxVoterWeight",
            "type": "u64"
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
      "name": "GovernanceProgramConfig",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "programId",
            "type": "publicKey"
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
      "name": "InvalidVoterWeightRecordRealm",
      "msg": "Invalid VoterWeightRecord Realm"
    },
    {
      "code": 6003,
      "name": "InvalidVoterWeightRecordMint",
      "msg": "Invalid VoterWeightRecord Mint"
    },
    {
      "code": 6004,
      "name": "TokenOwnerRecordFromOwnRealmNotAllowed",
      "msg": "TokenOwnerRecord from own realm is not allowed"
    },
    {
      "code": 6005,
      "name": "GovernanceProgramNotConfigured",
      "msg": "Governance program not configured"
    },
    {
      "code": 6006,
      "name": "GoverningTokenOwnerMustMatch",
      "msg": "Governing TokenOwner must match"
    }
  ]
};

export const IDL: RealmVoter = {
  "version": "0.0.1",
  "name": "realm_voter",
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
          "name": "maxGovernancePrograms",
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
        },
        {
          "name": "tokenOwnerRecord",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "configureVoterWeights",
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
          "name": "maxVoterWeightRecord",
          "isMut": true,
          "isSigner": false
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
      "name": "configureGovernanceProgram",
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
          "name": "governanceProgramId",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    }
  ],
  "accounts": [
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
            "name": "governanceProgramConfigs",
            "type": {
              "vec": {
                "defined": "GovernanceProgramConfig"
              }
            }
          },
          {
            "name": "realmMemberVoterWeight",
            "type": "u64"
          },
          {
            "name": "maxVoterWeight",
            "type": "u64"
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
      "name": "GovernanceProgramConfig",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "programId",
            "type": "publicKey"
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
      "name": "InvalidVoterWeightRecordRealm",
      "msg": "Invalid VoterWeightRecord Realm"
    },
    {
      "code": 6003,
      "name": "InvalidVoterWeightRecordMint",
      "msg": "Invalid VoterWeightRecord Mint"
    },
    {
      "code": 6004,
      "name": "TokenOwnerRecordFromOwnRealmNotAllowed",
      "msg": "TokenOwnerRecord from own realm is not allowed"
    },
    {
      "code": 6005,
      "name": "GovernanceProgramNotConfigured",
      "msg": "Governance program not configured"
    },
    {
      "code": 6006,
      "name": "GoverningTokenOwnerMustMatch",
      "msg": "Governing TokenOwner must match"
    }
  ]
};
