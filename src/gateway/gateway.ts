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
          "name": "gatekeeperNetwork",
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
          "name": "gatekeeperNetwork",
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
      "name": "updateVoterWeightRecord",
      "accounts": [
        {
          "name": "registrar",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "inputVoterWeight",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "gatewayToken",
          "isMut": false,
          "isSigner": false
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
            "name": "gatekeeperNetwork",
            "type": "publicKey"
          },
          {
            "name": "previousVoterWeightPluginProgramId",
            "type": {
              "option": "publicKey"
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
            "name": "TokenOwnerRecordV2",
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
      "name": "InvalidTokenOwnerForVoterWeightRecord",
      "msg": "Invalid TokenOwner for VoterWeightRecord"
    },
    {
      "code": 6010,
      "name": "InvalidGatewayToken",
      "msg": "Invalid gateway token"
    },
    {
      "code": 6011,
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
          "name": "gatekeeperNetwork",
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
          "name": "gatekeeperNetwork",
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
      "name": "updateVoterWeightRecord",
      "accounts": [
        {
          "name": "registrar",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "inputVoterWeight",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "gatewayToken",
          "isMut": false,
          "isSigner": false
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
            "name": "gatekeeperNetwork",
            "type": "publicKey"
          },
          {
            "name": "previousVoterWeightPluginProgramId",
            "type": {
              "option": "publicKey"
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
            "name": "TokenOwnerRecordV2",
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
      "name": "InvalidTokenOwnerForVoterWeightRecord",
      "msg": "Invalid TokenOwner for VoterWeightRecord"
    },
    {
      "code": 6010,
      "name": "InvalidGatewayToken",
      "msg": "Invalid gateway token"
    },
    {
      "code": 6011,
      "name": "MissingPreviousVoterWeightPlugin",
      "msg": "Previous voter weight plugin required but not provided"
    }
  ]
}