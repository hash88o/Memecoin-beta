'use client'

import { getMemeecoinProgram, getMemeecoinProgramId } from '@project/anchor'
import { useConnection } from '@solana/wallet-adapter-react'
import { Cluster, Keypair, PublicKey } from '@solana/web3.js'
import { useMutation, useQuery } from '@tanstack/react-query'
import { useMemo } from 'react'
import toast from 'react-hot-toast'
import { useCluster } from '../cluster/cluster-data-access'
import { useAnchorProvider } from '../solana/solana-provider'
import { useTransactionToast } from '../ui/ui-layout'

export function useMemeecoinProgram() {
  const { connection } = useConnection()
  const { cluster } = useCluster()
  const transactionToast = useTransactionToast()
  const provider = useAnchorProvider()
  const programId = useMemo(() => getMemeecoinProgramId(cluster.network as Cluster), [cluster])
  const program = useMemo(() => getMemeecoinProgram(provider, programId), [provider, programId])

  const accounts = useQuery({
    queryKey: ['memeecoin', 'all', { cluster }],
    queryFn: () => program.account.memeecoin.all(),
  })

  const getProgramAccount = useQuery({
    queryKey: ['get-program-account', { cluster }],
    queryFn: () => connection.getParsedAccountInfo(programId),
  })

  const initialize = useMutation({
    mutationKey: ['memeecoin', 'initialize', { cluster }],
    mutationFn: (keypair: Keypair) =>
      program.methods.initialize().accounts({ memeecoin: keypair.publicKey }).signers([keypair]).rpc(),
    onSuccess: (signature) => {
      transactionToast(signature)
      return accounts.refetch()
    },
    onError: () => toast.error('Failed to initialize account'),
  })

  return {
    program,
    programId,
    accounts,
    getProgramAccount,
    initialize,
  }
}

export function useMemeecoinProgramAccount({ account }: { account: PublicKey }) {
  const { cluster } = useCluster()
  const transactionToast = useTransactionToast()
  const { program, accounts } = useMemeecoinProgram()

  const accountQuery = useQuery({
    queryKey: ['memeecoin', 'fetch', { cluster, account }],
    queryFn: () => program.account.memeecoin.fetch(account),
  })

  const closeMutation = useMutation({
    mutationKey: ['memeecoin', 'close', { cluster, account }],
    mutationFn: () => program.methods.close().accounts({ memeecoin: account }).rpc(),
    onSuccess: (tx) => {
      transactionToast(tx)
      return accounts.refetch()
    },
  })

  const decrementMutation = useMutation({
    mutationKey: ['memeecoin', 'decrement', { cluster, account }],
    mutationFn: () => program.methods.decrement().accounts({ memeecoin: account }).rpc(),
    onSuccess: (tx) => {
      transactionToast(tx)
      return accountQuery.refetch()
    },
  })

  const incrementMutation = useMutation({
    mutationKey: ['memeecoin', 'increment', { cluster, account }],
    mutationFn: () => program.methods.increment().accounts({ memeecoin: account }).rpc(),
    onSuccess: (tx) => {
      transactionToast(tx)
      return accountQuery.refetch()
    },
  })

  const setMutation = useMutation({
    mutationKey: ['memeecoin', 'set', { cluster, account }],
    mutationFn: (value: number) => program.methods.set(value).accounts({ memeecoin: account }).rpc(),
    onSuccess: (tx) => {
      transactionToast(tx)
      return accountQuery.refetch()
    },
  })

  return {
    accountQuery,
    closeMutation,
    decrementMutation,
    incrementMutation,
    setMutation,
  }
}
