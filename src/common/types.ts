import { AccountClient, Idl, Program } from '@coral-xyz/anchor';

type RegistrarAccountTemplate = {
  name: 'registrar'
  type: {
    fields: [
      {
        "name": "previousVoterWeightPluginProgramId",
        "type": {
          "option": "publicKey"
        }
      }
    ]
  }
}

// IdlAccountDef is not exported by anchor, so this allows us to reference it
type NonNullableAccounts = NonNullable<Idl['accounts']>;
type IdlAccountDef = NonNullableAccounts[number];
type IdlTypeDefTyStruct = IdlAccountDef['type'];

// A type function that checks if one of the fields of an account type has the name 'previousVoterWeightPluginProgramId'
type HasPreviousVotingWeightPlugin<T extends IdlTypeDefTyStruct['fields']> =
  T extends [infer First extends IdlTypeDefTyStruct['fields'][number], ...infer Rest]
    ? First['name'] extends RegistrarAccountTemplate['type']['fields'][0]['name']
      ? T
      : Rest extends IdlTypeDefTyStruct['fields']
        ? (Rest extends HasPreviousVotingWeightPlugin<Rest> ? T : never)
        : never
    : never;

// A type function that checks if an account is a registrar account
type MatchesRegistrarAccountType<T extends IdlAccountDef> = T['name'] extends 'registrar' ? (
  T['type']['fields'] extends HasPreviousVotingWeightPlugin<T['type']['fields']> ? T : never
  ) : never;

// A type function that checks that an IDLAccountDef array has a registrar account
type HasRegistrar<T extends IdlAccountDef[]> =
  T extends [infer First extends IdlAccountDef, ...infer Rest]
    ? First extends MatchesRegistrarAccountType<First>
      ? T
      : Rest extends IdlAccountDef[]
        ? (Rest extends HasRegistrar<Rest> ? T : never)
        : never
    : never;


// A type function that defines a program that uses a plugin IDL
export type PluginProgramAccounts<T extends Idl> = Program<T>['account'] extends {
  registrar: AccountClient<T,'registrar'>,
  voterWeightRecord: AccountClient<T,'voterWeightRecord'>,
  // may be undefined if the plugin does not support maxVoterWeightRecord - should be inferrable from T
  maxVoterWeightRecord: AccountClient<T,'maxVoterWeightRecord'>,
} ? Program<T>['account'] : never;
