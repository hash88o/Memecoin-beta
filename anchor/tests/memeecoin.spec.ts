import * as anchor from '@coral-xyz/anchor'
import {Program} from '@coral-xyz/anchor'
import {Keypair} from '@solana/web3.js'
import {Memeecoin} from '../target/types/memeecoin'

describe('memeecoin', () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider)
  const payer = provider.wallet as anchor.Wallet

  const program = anchor.workspace.Memeecoin as Program<Memeecoin>

  const memeecoinKeypair = Keypair.generate()

  it('Initialize Memeecoin', async () => {
    await program.methods
      .initialize()
      .accounts({
        memeecoin: memeecoinKeypair.publicKey,
        payer: payer.publicKey,
      })
      .signers([memeecoinKeypair])
      .rpc()

    const currentCount = await program.account.memeecoin.fetch(memeecoinKeypair.publicKey)

    expect(currentCount.count).toEqual(0)
  })

  it('Increment Memeecoin', async () => {
    await program.methods.increment().accounts({ memeecoin: memeecoinKeypair.publicKey }).rpc()

    const currentCount = await program.account.memeecoin.fetch(memeecoinKeypair.publicKey)

    expect(currentCount.count).toEqual(1)
  })

  it('Increment Memeecoin Again', async () => {
    await program.methods.increment().accounts({ memeecoin: memeecoinKeypair.publicKey }).rpc()

    const currentCount = await program.account.memeecoin.fetch(memeecoinKeypair.publicKey)

    expect(currentCount.count).toEqual(2)
  })

  it('Decrement Memeecoin', async () => {
    await program.methods.decrement().accounts({ memeecoin: memeecoinKeypair.publicKey }).rpc()

    const currentCount = await program.account.memeecoin.fetch(memeecoinKeypair.publicKey)

    expect(currentCount.count).toEqual(1)
  })

  it('Set memeecoin value', async () => {
    await program.methods.set(42).accounts({ memeecoin: memeecoinKeypair.publicKey }).rpc()

    const currentCount = await program.account.memeecoin.fetch(memeecoinKeypair.publicKey)

    expect(currentCount.count).toEqual(42)
  })

  it('Set close the memeecoin account', async () => {
    await program.methods
      .close()
      .accounts({
        payer: payer.publicKey,
        memeecoin: memeecoinKeypair.publicKey,
      })
      .rpc()

    // The account should no longer exist, returning null.
    const userAccount = await program.account.memeecoin.fetchNullable(memeecoinKeypair.publicKey)
    expect(userAccount).toBeNull()
  })
})
