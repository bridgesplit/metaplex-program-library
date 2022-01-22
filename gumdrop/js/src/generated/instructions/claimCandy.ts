import * as splToken from '@solana/spl-token';
import * as beet from '@metaplex-foundation/beet';
import * as web3 from '@solana/web3.js';
import * as beetSolana from '@metaplex-foundation/beet-solana';

export type ClaimCandyInstructionArgs = {
  walletBump: number;
  claimBump: number;
  index: beet.bignum;
  amount: beet.bignum;
  claimantSecret: web3.PublicKey;
  proof: number[] /* size: 32 */[];
};
const claimCandyStruct = new beet.FixableBeetArgsStruct<
  ClaimCandyInstructionArgs & {
    instructionDiscriminator: number[];
  }
>(
  [
    ['instructionDiscriminator', beet.uniformFixedSizeArray(beet.u8, 8)],
    ['walletBump', beet.u8],
    ['claimBump', beet.u8],
    ['index', beet.u64],
    ['amount', beet.u64],
    ['claimantSecret', beetSolana.publicKey],
    ['proof', beet.array(beet.uniformFixedSizeArray(beet.u8, 32))],
  ],
  'ClaimCandyInstructionArgs',
);
export type ClaimCandyInstructionAccounts = {
  distributor: web3.PublicKey;
  distributorWallet: web3.PublicKey;
  claimCount: web3.PublicKey;
  temporal: web3.PublicKey;
  payer: web3.PublicKey;
  candyMachineConfig: web3.PublicKey;
  candyMachine: web3.PublicKey;
  candyMachineWallet: web3.PublicKey;
  candyMachineMint: web3.PublicKey;
  candyMachineMetadata: web3.PublicKey;
  candyMachineMasterEdition: web3.PublicKey;
  tokenMetadataProgram: web3.PublicKey;
  candyMachineProgram: web3.PublicKey;
  clock: web3.PublicKey;
};

const claimCandyInstructionDiscriminator = [87, 176, 177, 90, 136, 95, 83, 242];

/**
 * Creates a _ClaimCandy_ instruction.
 *
 * @param accounts that will be accessed while the instruction is processed
 * @param args to provide as instruction data to the program
 */
export function createClaimCandyInstruction(
  accounts: ClaimCandyInstructionAccounts,
  args: ClaimCandyInstructionArgs,
) {
  const {
    distributor,
    distributorWallet,
    claimCount,
    temporal,
    payer,
    candyMachineConfig,
    candyMachine,
    candyMachineWallet,
    candyMachineMint,
    candyMachineMetadata,
    candyMachineMasterEdition,
    tokenMetadataProgram,
    candyMachineProgram,
    clock,
  } = accounts;

  const [data] = claimCandyStruct.serialize({
    instructionDiscriminator: claimCandyInstructionDiscriminator,
    ...args,
  });
  const keys: web3.AccountMeta[] = [
    {
      pubkey: distributor,
      isWritable: true,
      isSigner: false,
    },
    {
      pubkey: distributorWallet,
      isWritable: true,
      isSigner: false,
    },
    {
      pubkey: claimCount,
      isWritable: true,
      isSigner: false,
    },
    {
      pubkey: temporal,
      isWritable: false,
      isSigner: true,
    },
    {
      pubkey: payer,
      isWritable: false,
      isSigner: true,
    },
    {
      pubkey: candyMachineConfig,
      isWritable: false,
      isSigner: false,
    },
    {
      pubkey: candyMachine,
      isWritable: true,
      isSigner: false,
    },
    {
      pubkey: candyMachineWallet,
      isWritable: true,
      isSigner: false,
    },
    {
      pubkey: candyMachineMint,
      isWritable: true,
      isSigner: false,
    },
    {
      pubkey: candyMachineMetadata,
      isWritable: true,
      isSigner: false,
    },
    {
      pubkey: candyMachineMasterEdition,
      isWritable: true,
      isSigner: false,
    },
    {
      pubkey: web3.SystemProgram.programId,
      isWritable: false,
      isSigner: false,
    },
    {
      pubkey: splToken.TOKEN_PROGRAM_ID,
      isWritable: false,
      isSigner: false,
    },
    {
      pubkey: tokenMetadataProgram,
      isWritable: false,
      isSigner: false,
    },
    {
      pubkey: candyMachineProgram,
      isWritable: false,
      isSigner: false,
    },
    {
      pubkey: web3.SYSVAR_RENT_PUBKEY,
      isWritable: false,
      isSigner: false,
    },
    {
      pubkey: clock,
      isWritable: false,
      isSigner: false,
    },
  ];

  const ix = new web3.TransactionInstruction({
    programId: new web3.PublicKey('gdrpGjVffourzkdDRrQmySw4aTHr8a3xmQzzxSwFD1a'),
    keys,
    data,
  });
  return ix;
}
