#!/usr/bin/env bash

set -euo pipefail

# if [[ -z "${PROVIDER_WALLET}" ]]; then
#   echo "Please provide path to a provider wallet keypair."
#   exit -1
# fi

# if [[ -z "${VERSION_MANUALLY_BUMPED}" ]]; then
#   echo "Please bump versions in package.json and in cargo.toml."
#   exit -1
# fi

# build program
anchor build

# update on chain program and IDL, atm used for testing/developing
# anchor deploy --provider.cluster devnet --provider.wallet ${PROVIDER_WALLET}
# anchor idl upgrade --provider.cluster devnet --provider.wallet ${PROVIDER_WALLET}\
#  --filepath target/idl/nft_voter.json FDfF7jzJDCEkFWNi3is487k8rFPJxFkU821t2pQ1vDr1

# update types in npm package and publish the npm package
cp ./target/types/nft_voter.ts src/nft_voter.ts
yarn clean && yarn build && cp package.json ./dist/ && yarn publish dist

echo
echo Remember to commit and push the version update as well as the changes
echo to src/nft_voter.ts .
echo
