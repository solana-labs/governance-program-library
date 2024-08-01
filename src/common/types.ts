import { AccountClient, Idl, Program } from '@coral-xyz/anchor';
import { IdlDefinedFieldsNamed, IdlField, IdlTypeDefTyStruct } from '@coral-xyz/anchor/dist/cjs/idl';

type RegistrarAccountTemplate = {
  name: 'registrar'
  type: {
    fields: [
      {
        "name": "previousVoterWeightPluginProgramId",
        "type": {
          "option": "pubkey"
        }
      }
    ]
  }
}

// A type function that checks if one of the fields of an account type has the name 'previousVoterWeightPluginProgramId'
type HasPreviousVotingWeightPlugin<T extends IdlDefinedFieldsNamed[number]> =
  T extends [infer First extends IdlDefinedFieldsNamed[number], ...infer Rest]
    ? First['name'] extends RegistrarAccountTemplate['type']['fields'][0]['name']
      ? T
      : Rest extends IdlDefinedFieldsNamed[number]
        ? (Rest extends HasPreviousVotingWeightPlugin<Rest> ? T : never)
        : never
    : never;

// A type function that checks if an account is a registrar account
type MatchesRegistrarAccountType<T extends IdlDefinedFieldsNamed[number]> = T['name'] extends 'registrar' ? (
  T extends HasPreviousVotingWeightPlugin<T> ? T : never
  ) : never;

// A type function that checks that an IDLAccountDef array has a registrar account
type HasRegistrar<T extends IdlDefinedFieldsNamed[number]> =
  T extends [infer First extends IdlDefinedFieldsNamed[number], ...infer Rest]
    ? First extends MatchesRegistrarAccountType<First>
      ? T
      : Rest extends IdlDefinedFieldsNamed[number]
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
