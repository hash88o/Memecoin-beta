// Here we export some useful types and functions for interacting with the Anchor program.
import { AnchorProvider, Program } from '@coral-xyz/anchor'
import { Cluster, PublicKey } from '@solana/web3.js'
import MemeecoinIDL from '../target/idl/memeecoin.json'
import type { Memeecoin } from '../target/types/memeecoin'

// Re-export the generated IDL and type
export { Memeecoin, MemeecoinIDL }

// The programId is imported from the program IDL.
export const MEMEECOIN_PROGRAM_ID = new PublicKey(MemeecoinIDL.address)

// This is a helper function to get the Memeecoin Anchor program.
export function getMemeecoinProgram(provider: AnchorProvider, address?: PublicKey) {
  return new Program({ ...MemeecoinIDL, address: address ? address.toBase58() : MemeecoinIDL.address } as Memeecoin, provider)
}

// This is a helper function to get the program ID for the Memeecoin program depending on the cluster.
export function getMemeecoinProgramId(cluster: Cluster) {
  switch (cluster) {
    case 'devnet':
    case 'testnet':
      // This is the program ID for the Memeecoin program on devnet and testnet.
      return new PublicKey('coUnmi3oBUtwtd9fjeAvSsJssXh5A5xyPbhpewyzRVF')
    case 'mainnet-beta':
    default:
      return MEMEECOIN_PROGRAM_ID
  }
}
