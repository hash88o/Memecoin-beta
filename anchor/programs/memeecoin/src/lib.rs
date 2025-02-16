use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{self, Mint, Token, TokenAccount};

declare_id!("2ELnZEunuAXPW966HS57chzPXCCHHhyVkksR6qmEpwy2");

#[program]
pub mod meme_token {
    use super::*;

    // Tokenomics constants
    const INITIAL_SUPPLY: u64 = 1_000_000_000; // 1 billion tokens
    const LIQUIDITY_LOCK_PERIOD: i64 = 180 * 24 * 60 * 60; // 180 days
    const MIN_HOLDING_PERIOD: i64 = 7 * 24 * 60 * 60; // 7 days for rewards

    // Default rates (can be modified through governance)
    const DEFAULT_TRANSACTION_FEE_BPS: u16 = 100; // 1%
    const DEFAULT_BURN_RATE_BPS: u16 = 50; // 0.5%
    const DEFAULT_REWARD_RATE_BPS: u16 = 50; // 0.5%
    const DEFAULT_MAX_WALLET_BPS: u16 = 200; // 2% max wallet size
    const DEFAULT_MAX_TX_BPS: u16 = 100; // 1% max transaction size

    // Governance constants
    const PROPOSAL_VOTING_PERIOD: i64 = 3 * 24 * 60 * 60; // 3 days
    const MIN_PROPOSAL_THRESHOLD_BPS: u16 = 100; // 1% of supply to create proposal
    const MIN_QUORUM_BPS: u16 = 1000; // 10% of supply for quorum

    #[derive(Accounts)]
    pub struct Initialize<'info> {
        #[account(mut)]
        pub creator: Signer<'info>,

        #[account(
            init,
            payer = creator,
            mint::decimals = 9,
            mint::authority = creator.key(),
        )]
        pub mint: Box<Account<'info, token::Mint>>,

        #[account(
            init,
            payer = creator,
            associated_token::mint = mint,
            associated_token::authority = creator,
        )]
        pub creator_token_account: Box<Account<'info, token::TokenAccount>>,

        #[account(
            init,
            payer = creator,
            space = 8 + TokenConfig::LEN
        )]
        pub token_config: Account<'info, TokenConfig>,

        #[account(
            init,
            payer = creator,
            space = 8 + RewardPool::LEN
        )]
        pub reward_pool: Account<'info, RewardPool>,

        #[account(
            init,
            payer = creator,
            space = 8 + GovernanceConfig::LEN
        )]
        pub governance_config: Account<'info, GovernanceConfig>,

        #[account(
            init,
            payer = creator,
            space = 8 + Analytics::LEN
        )]
        pub analytics: Account<'info, Analytics>,

        pub system_program: Program<'info, System>,
        pub token_program: Program<'info, Token>,
        pub associated_token_program: Program<'info, AssociatedToken>,
        pub rent: Sysvar<'info, Rent>,
    }

    #[derive(Accounts)]
    pub struct CreateProposal<'info> {
        #[account(mut)]
        pub proposer: Signer<'info>,
        #[account(mut)]
        pub proposer_token_account: Box<Account<'info, token::TokenAccount>>,
        #[account(mut)]
        pub governance_config: Account<'info, GovernanceConfig>,
        #[account(
            init,
            payer = proposer,
            space = 8 + Proposal::LEN
        )]
        pub proposal: Account<'info, Proposal>,
        pub system_program: Program<'info, System>,
    }

    #[derive(Accounts)]
    pub struct Vote<'info> {
        #[account(mut)]
        pub voter: Signer<'info>,
        #[account(mut)]
        pub voter_token_account: Box<Account<'info, token::TokenAccount>>,
        #[account(mut)]
        pub proposal: Account<'info, Proposal>,
    }

    #[account]
    pub struct TokenConfig {
        pub creator: Pubkey,
        pub mint: Pubkey,
        pub liquidity_unlock_time: i64,
        pub total_supply: u64,
        pub circulating_supply: u64,
        pub transaction_fee_bps: u16,
        pub burn_rate_bps: u16,
        pub reward_rate_bps: u16,
        pub max_wallet_bps: u16,
        pub max_tx_bps: u16,
        pub total_burned: u64,
        pub total_rewards_distributed: u64,
    }

    #[account]
    pub struct RewardPool {
        pub accumulated_rewards: u64,
        pub last_distribution_time: i64,
        pub reward_per_token_stored: u64,
        pub total_staked: u64,
        pub holders: Vec<HolderInfo>,
    }

    #[derive(AnchorSerialize, AnchorDeserialize, Clone)]
    pub struct HolderInfo {
        pub address: Pubkey,
        pub balance: u64,
        pub last_deposit_time: i64,
        pub rewards_per_token_paid: u64,
        pub unclaimed_rewards: u64,
        pub staked_amount: u64,
        pub last_stake_time: i64,
    }

    impl RewardPool {
        pub const LEN: usize = 8 + 8 + 8 + 8 + (32 + 8 + 8 + 8 + 8 + 8 + 8) * 1000; // Space for 1000 holders
    }

    #[account]
    pub struct GovernanceConfig {
        pub proposals: Vec<Pubkey>,
        pub executed_proposals: Vec<Pubkey>,
        pub proposal_count: u64,
        pub min_proposal_threshold_bps: u16,
        pub min_quorum_bps: u16,
    }

    #[account]
    pub struct Proposal {
        pub id: u64,
        pub proposer: Pubkey,
        pub description: String,
        pub proposal_type: ProposalType,
        pub status: ProposalStatus,
        pub start_time: i64,
        pub end_time: i64,
        pub for_votes: u64,
        pub against_votes: u64,
        pub executed: bool,
        pub voters: Vec<Pubkey>,
    }

    #[account]
    pub struct Analytics {
        pub total_transactions: u64,
        pub unique_holders: u64,
        pub volume_24h: u64,
        pub largest_transfer: u64,
        pub total_proposals: u64,
        pub total_votes: u64,
        pub hourly_volume: [u64; 24],
        pub daily_active_wallets: u64,
        pub avg_holding_time: i64,
        pub price_impact_data: Vec<PriceImpactEntry>,
    }

    #[derive(AnchorSerialize, AnchorDeserialize, Clone)]
    pub struct PriceImpactEntry {
        pub timestamp: i64,
        pub amount: u64,
        pub price_impact_bps: i16,
    }

    #[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
    pub enum ProposalType {
        UpdateFees { new_fee_bps: u16 },
        UpdateBurnRate { new_burn_bps: u16 },
        UpdateRewardRate { new_reward_bps: u16 },
        UpdateMaxWallet { new_max_wallet_bps: u16 },
        UpdateMaxTransaction { new_max_tx_bps: u16 },
    }

    #[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
    pub enum ProposalStatus {
        Active,
        Succeeded,
        Failed,
        Executed,
    }

    impl TokenConfig {
        pub const LEN: usize = 32 + 32 + 8 + 8 + 8 + 2 + 2 + 2 + 2 + 2 + 8 + 8;
    }

    impl GovernanceConfig {
        pub const LEN: usize = 32 * 100 + 32 * 100 + 8 + 2 + 2; // Space for 100 active and executed proposals
    }

    impl Proposal {
        pub const LEN: usize = 8 + 32 + 200 + 32 + 1 + 8 + 8 + 8 + 8 + 1 + 32 * 1000; // Space for 1000 voters
    }

    impl Analytics {
        pub const LEN: usize = 8 + 8 + 8 + 8 + 8 + 8 + 8 * 24 + 8 + 8 + 32 * 100; // Space for 100 price impact entries
    }

    #[event]
    pub struct TokenTransfer {
        from: Pubkey,
        to: Pubkey,
        amount: u64,
        fee: u64,
        burned: u64,
        timestamp: i64,
        price_impact_bps: i16,
    }

    #[event]
    pub struct ProposalCreated {
        proposal_id: u64,
        proposer: Pubkey,
        proposal_type: ProposalType,
        start_time: i64,
        end_time: i64,
    }

    #[event]
    pub struct VoteCast {
        proposal_id: u64,
        voter: Pubkey,
        vote_amount: u64,
        support: bool,
        timestamp: i64,
    }

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let token_config = &mut ctx.accounts.token_config;
        let governance_config = &mut ctx.accounts.governance_config;
        let analytics = &mut ctx.accounts.analytics;
        let clock = Clock::get()?;

        // Initialize token configuration
        token_config.creator = ctx.accounts.creator.key();
        token_config.mint = ctx.accounts.mint.key();
        token_config.liquidity_unlock_time = clock.unix_timestamp + LIQUIDITY_LOCK_PERIOD;
        token_config.total_supply = INITIAL_SUPPLY;
        token_config.circulating_supply = INITIAL_SUPPLY;
        token_config.transaction_fee_bps = DEFAULT_TRANSACTION_FEE_BPS;
        token_config.burn_rate_bps = DEFAULT_BURN_RATE_BPS;
        token_config.reward_rate_bps = DEFAULT_REWARD_RATE_BPS;
        token_config.max_wallet_bps = DEFAULT_MAX_WALLET_BPS;
        token_config.max_tx_bps = DEFAULT_MAX_TX_BPS;

        // Initialize governance configuration
        governance_config.proposal_count = 0;
        governance_config.min_proposal_threshold_bps = MIN_PROPOSAL_THRESHOLD_BPS;
        governance_config.min_quorum_bps = MIN_QUORUM_BPS;

        // Initialize analytics
        analytics.total_transactions = 0;
        analytics.unique_holders = 1; // Creator is first holder
        analytics.hourly_volume = [0; 24];

        // Mint initial supply to creator
        token::mint_to(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::MintTo {
                    mint: ctx.accounts.mint.to_account_info(),
                    to: ctx.accounts.creator_token_account.to_account_info(),
                    authority: ctx.accounts.creator.to_account_info(),
                },
            ),
            INITIAL_SUPPLY,
        )?;

        Ok(())
    }

    pub fn create_proposal(
        ctx: Context<CreateProposal>,
        description: String,
        proposal_type: ProposalType,
    ) -> Result<()> {
        let governance_config = &mut ctx.accounts.governance_config;
        let proposal = &mut ctx.accounts.proposal;
        let clock = Clock::get()?;

        // Check proposer has enough tokens
        let required_tokens = (INITIAL_SUPPLY as u128)
            .checked_mul(governance_config.min_proposal_threshold_bps as u128)
            .unwrap()
            .checked_div(10000)
            .unwrap() as u64;

        require!(
            ctx.accounts.proposer_token_account.amount >= required_tokens,
            MemeTokenError::InsufficientProposalTokens
        );

        // Initialize proposal
        proposal.id = governance_config.proposal_count;
        proposal.proposer = ctx.accounts.proposer.key();
        proposal.description = description;
        proposal.proposal_type = proposal_type;
        proposal.status = ProposalStatus::Active;
        proposal.start_time = clock.unix_timestamp;
        proposal.end_time = clock.unix_timestamp + PROPOSAL_VOTING_PERIOD;
        proposal.for_votes = 0;
        proposal.against_votes = 0;
        proposal.executed = false;

        // Update governance config
        governance_config.proposals.push(proposal.key());
        governance_config.proposal_count += 1;

        // Emit event
        emit!(ProposalCreated {
            proposal_id: proposal.id,
            proposer: proposal.proposer,
            proposal_type: proposal.proposal_type.clone(),
            start_time: proposal.start_time,
            end_time: proposal.end_time,
        });

        Ok(())
    }

    pub fn vote(ctx: Context<Vote>, support: bool) -> Result<()> {
        let proposal = &mut ctx.accounts.proposal;
        let clock = Clock::get()?;

        // Validate voting period
        require!(
            clock.unix_timestamp >= proposal.start_time
                && clock.unix_timestamp <= proposal.end_time,
            MemeTokenError::InvalidVotingPeriod
        );

        // Check voter hasn't already voted
        require!(
            !proposal.voters.contains(&ctx.accounts.voter.key()),
            MemeTokenError::AlreadyVoted
        );

        // Record vote
        let vote_weight = ctx.accounts.voter_token_account.amount;
        if support {
            proposal.for_votes = proposal.for_votes.checked_add(vote_weight).unwrap();
        } else {
            proposal.against_votes = proposal.against_votes.checked_add(vote_weight).unwrap();
        }

        proposal.voters.push(ctx.accounts.voter.key());

        // Emit event
        emit!(VoteCast {
            proposal_id: proposal.id,
            voter: ctx.accounts.voter.key(),
            vote_amount: vote_weight,
            support,
            timestamp: clock.unix_timestamp,
        });

        Ok(())
    }

    // Keep the public instruction and its struct
    #[derive(Accounts)]
    pub struct UpdateRewards<'info> {
        #[account(mut)]
        pub holder: Signer<'info>,
        #[account(mut)]
        pub reward_pool: Account<'info, RewardPool>,
    }

    pub fn update_rewards(ctx: Context<UpdateRewards>, amount: u64) -> Result<()> {
        let reward_pool = &mut ctx.accounts.reward_pool;
        let holder = ctx.accounts.holder.key();
        let clock = Clock::get()?;
        let timestamp = clock.unix_timestamp;

        if reward_pool.total_staked == 0 {
            return Ok(());
        }

        let time_elapsed = timestamp - reward_pool.last_distribution_time;
        if time_elapsed > 0 {
            let reward_rate = reward_pool
                .accumulated_rewards
                .checked_div(time_elapsed as u64)
                .unwrap_or(0);

            reward_pool.reward_per_token_stored = reward_pool
                .reward_per_token_stored
                .checked_add(
                    reward_rate
                        .checked_mul(1_000_000)
                        .unwrap()
                        .checked_div(reward_pool.total_staked)
                        .unwrap_or(0),
                )
                .unwrap();
        }

        // Store the current reward_per_token_stored value
        let current_reward_per_token = reward_pool.reward_per_token_stored;

        if let Some(holder_info) = reward_pool.holders.iter_mut().find(|h| h.address == holder) {
            let reward_delta = amount
                .checked_mul(
                    current_reward_per_token
                        .checked_sub(holder_info.rewards_per_token_paid)
                        .unwrap_or(0),
                )
                .unwrap()
                .checked_div(1_000_000)
                .unwrap();

            holder_info.unclaimed_rewards = holder_info
                .unclaimed_rewards
                .checked_add(reward_delta)
                .unwrap();
            holder_info.rewards_per_token_paid = current_reward_per_token;
            holder_info.balance = amount;
            holder_info.last_deposit_time = timestamp;
        } else {
            reward_pool.holders.push(HolderInfo {
                address: holder,
                balance: amount,
                last_deposit_time: timestamp,
                rewards_per_token_paid: current_reward_per_token,
                unclaimed_rewards: 0,
                staked_amount: 0,
                last_stake_time: timestamp,
            });
        }

        reward_pool.last_distribution_time = timestamp;
        Ok(())
    }

    // Additional helper functions would be here for:
    // - Executing proposals
    // - Updating analytics
    // - Processing transfers with anti-whale checks
    // - Managing reward distribution
    // - Handling liquidity pool interactions

    #[error_code]
    pub enum MemeTokenError {
        #[msg("Liquidity is locked until the specified unlock time")]
        LiquidityLocked,
        #[msg("Insufficient tokens to create proposal")]
        InsufficientProposalTokens,
        #[msg("Invalid voting period")]
        InvalidVotingPeriod,
        #[msg("Already voted on this proposal")]
        AlreadyVoted,
        #[msg("Transaction would exceed maximum wallet size")]
        ExceedsMaxWallet,
        #[msg("Transaction would exceed maximum transaction size")]
        ExceedsMaxTransaction,
        #[msg("Proposal has already been executed")]
        ProposalAlreadyExecuted,
        #[msg("Proposal failed to meet quorum")]
        QuorumNotMet,
    }
}
