import {
	BN,
	DEFAULT_USER_NAME,
	DriftClient,
	getInsuranceFundStakeAccountPublicKey,
	getInsuranceFundVaultPublicKey,
	getSpotMarketPublicKey,
	QUOTE_SPOT_MARKET_INDEX,
	ReferrerInfo,
	ZERO,
	unstakeSharesToAmount,
	QUOTE_PRECISION,
	PERCENTAGE_PRECISION,
	fetchLogs,
} from '@drift-labs/sdk';
import { Program } from '@coral-xyz/anchor';
import { DriftCompetitions, IDL } from './types/drift_competitions';
import {
	ComputeBudgetProgram,
	PublicKey,
	SetComputeUnitLimitParams,
	SYSVAR_RENT_PUBKEY,
	Transaction,
	TransactionInstruction,
	TransactionSignature,
} from '@solana/web3.js';
import { encodeName } from './name';
import {
	getCompetitionAddressSync,
	getCompetitionAuthorityAddressSync,
	getCompetitorAddressSync,
} from './addresses';
import {
	DEVNET_GENESIS_HASH,
	FunctionRequestAccount,
	MAINNET_GENESIS_HASH,
	SwitchboardProgram,
} from '@switchboard-xyz/solana.js';
import * as anchor from '@coral-xyz/anchor';
import { DRIFT_COMPETITION_PROGRAM_ID } from './constants';
import { LogParser } from './parsers';

export class CompetitionsClient {
	driftClient: DriftClient;
	program: Program<DriftCompetitions>;

	constructor({
		driftClient,
		program,
	}: {
		driftClient: DriftClient;
		program?: Program<DriftCompetitions>;
	}) {
		this.driftClient = driftClient;

		if (!program) {
			program = new Program(
				IDL,
				DRIFT_COMPETITION_PROGRAM_ID,
				driftClient.provider
			);
		}
		this.program = program;
	}

	public async initializeCompetition({
		name,
		nextRoundExpiryTs,
		competitionExpiryTs,
		roundDuration,
		maxEntriesPerCompetitor,
		minSponsorAmount,
		maxSponsorFraction,
		numberOfWinners,
	}: {
		name: string;
		nextRoundExpiryTs: BN;
		competitionExpiryTs: BN;
		roundDuration: BN;
		maxEntriesPerCompetitor: BN;
		minSponsorAmount: BN;
		maxSponsorFraction: BN;
		numberOfWinners: number;
	}): Promise<TransactionSignature> {
		const encodedName = encodeName(name);
		const competitionAddress = getCompetitionAddressSync(
			this.program.programId,
			encodedName
		);

		return await this.program.methods
			.initializeCompetition({
				name: encodedName,
				nextRoundExpiryTs,
				competitionExpiryTs,
				roundDuration,
				maxEntriesPerCompetitor,
				minSponsorAmount,
				maxSponsorFraction,
				numberOfWinners,
			})
			.accounts({
				competition: competitionAddress,
			})
			.rpc();
	}

	public async updateCompetition(
		competition: PublicKey,
		{
			nextRoundExpiryTs = null,
			competitionExpiryTs = null,
			roundDuration = null,
			maxEntriesPerCompetitor = null,
			minSponsorAmount = null,
			maxSponsorFraction = null,
			numberOfWinners = null,
		}: {
			nextRoundExpiryTs?: BN | null;
			competitionExpiryTs?: BN | null;
			roundDuration?: BN | null;
			maxEntriesPerCompetitor?: BN | null;
			minSponsorAmount?: BN | null;
			maxSponsorFraction?: BN | null;
			numberOfWinners?: number | null;
		}
	): Promise<TransactionSignature> {
		return await this.program.methods
			.updateCompetition({
				nextRoundExpiryTs,
				competitionExpiryTs,
				roundDuration,
				maxEntriesPerCompetitor,
				minSponsorAmount,
				maxSponsorFraction,
				numberOfWinners,
			})
			.accounts({
				competition: competition,
			})
			.rpc();
	}

	public async updateSwitchboardFunction(
		competition: PublicKey,
		switchboardFunction: PublicKey
	): Promise<TransactionSignature> {
		const switchboardProgram = await SwitchboardProgram.fromProvider(
			// @ts-ignore
			this.program.provider
		);

		const genesisHash =
			await switchboardProgram.provider.connection.getGenesisHash();
		const attestationQueueAddress =
			genesisHash === MAINNET_GENESIS_HASH
				? new PublicKey('2ie3JZfKcvsRLsJaP5fSo43gUo1vsurnUAtAgUdUAiDG')
				: genesisHash === DEVNET_GENESIS_HASH
				? new PublicKey('CkvizjVnm2zA5Wuwan34NhVT3zFc7vqUyGnA6tuEF5aE')
				: undefined;

		const switchboardRequestKeypair = anchor.web3.Keypair.generate();
		const switchboardRequest = new FunctionRequestAccount(
			switchboardProgram,
			switchboardRequestKeypair.publicKey
		);

		const switchboardRequestEscrowPubkey =
			await anchor.utils.token.associatedAddress({
				mint: switchboardProgram.mint.address,
				owner: switchboardRequestKeypair.publicKey,
			});

		const competitionAuthority = getCompetitionAuthorityAddressSync(
			this.program.programId,
			competition
		);

		return await this.program.methods
			.updateSwitchboardFunction()
			.accounts({
				sponsor: this.program.provider.publicKey,
				competition,
				competitionAuthority,
				switchboard: switchboardProgram.attestationProgramId,
				switchboardState: switchboardProgram.attestationProgramState.publicKey,
				switchboardAttestationQueue: attestationQueueAddress,
				switchboardFunction: switchboardFunction,
				switchboardRequest: switchboardRequest.publicKey,
				switchboardRequestEscrow: switchboardRequestEscrowPubkey,
				switchboardMint: switchboardProgram.mint.address,
			})
			.signers([switchboardRequestKeypair])
			.rpc();
	}

	public async initializeCompetitor({
		competition,
		initDriftUser,
		referrerInfo,
	}: {
		competition: PublicKey;
		initDriftUser?: boolean;
		referrerInfo?: ReferrerInfo;
	}): Promise<TransactionSignature> {
		const competitor = getCompetitorAddressSync(
			this.program.programId,
			competition,
			this.program.provider.publicKey
		);

		const accounts = {
			competitor,
			competition: competition,
			driftUserStats: this.driftClient.getUserStatsAccountPublicKey(),
		};

		const instructions: TransactionInstruction[] = [];

		if (initDriftUser) {
			const initUserStatsIx = await this.driftClient.getInitializeUserStatsIx();
			const [_userAccountPublicKey, initializeUserAccountIx] =
				await this.driftClient.getInitializeUserInstructions(
					0,
					DEFAULT_USER_NAME,
					referrerInfo
				);
			instructions.push(initUserStatsIx);
			instructions.push(initializeUserAccountIx);
		}

		const initCompetitorIx = this.program.instruction.initializeCompetitor({
			accounts: {
				...accounts,
				payer: this.program.provider.publicKey,
				rent: SYSVAR_RENT_PUBKEY,
				authority: this.program.provider.publicKey,
				systemProgram: anchor.web3.SystemProgram.programId,
			},
		});
		instructions.push(initCompetitorIx);

		return await this.createAndSendTxn(instructions, {
			computeUnitParams: {
				units: 1_400_000,
			},
		});
	}

	public async claimEntry(
		competition: PublicKey,
		userStatsKey?: PublicKey
	): Promise<TransactionSignature> {
		const competitor = getCompetitorAddressSync(
			this.program.programId,
			competition,
			this.program.provider.publicKey
		);

		if (!userStatsKey) {
			userStatsKey = this.driftClient.getUserStatsAccountPublicKey();
		}

		const accounts = {
			competitor,
			competition: competition,
			driftUserStats: userStatsKey,
			instructions: anchor.web3.SYSVAR_INSTRUCTIONS_PUBKEY,
		};

		const claimEntryIx = this.program.instruction.claimEntry({
			accounts: {
				...accounts,
				authority: this.program.provider.publicKey,
			},
		});
		return await this.createAndSendTxn([claimEntryIx], {
			noComputeBudgetIx: true, // claim entry needs to be a standalone ix in a tx
		});
	}

	public async claimWinnings({
		competition,
		shares,
		initIFStake,
	}: {
		competition: PublicKey;
		shares?: BN;
		initIFStake?: boolean;
	}): Promise<TransactionSignature> {
		const competitor = getCompetitorAddressSync(
			this.program.programId,
			competition,
			this.program.provider.publicKey
		);
		const competitionAuthority = getCompetitionAuthorityAddressSync(
			this.program.programId,
			competition
		);

		const spotMarket = await getSpotMarketPublicKey(
			this.driftClient.program.programId,
			QUOTE_SPOT_MARKET_INDEX
		);
		const insuranceFundVault = await getInsuranceFundVaultPublicKey(
			this.driftClient.program.programId,
			QUOTE_SPOT_MARKET_INDEX
		);
		const insuranceFundStake = await getInsuranceFundStakeAccountPublicKey(
			this.driftClient.program.programId,
			this.program.provider.publicKey,
			QUOTE_SPOT_MARKET_INDEX
		);
		const driftState = await this.driftClient.getStatePublicKey();
		const driftTransferConfig = PublicKey.findProgramAddressSync(
			[
				Buffer.from(
					anchor.utils.bytes.utf8.encode('if_shares_transfer_config')
				),
			],
			this.driftClient.program.programId
		)[0];

		const accounts = {
			competitor,
			competition: competition,
			driftUserStats: this.driftClient.getUserStatsAccountPublicKey(),
			spotMarket,
			insuranceFundStake,
			insuranceFundVault,
			driftProgram: this.driftClient.program.programId,
			competitionAuthority,
			driftState,
			driftTransferConfig,
		};

		const instructions: TransactionInstruction[] = [];

		if (initIFStake) {
			const initIFStakeIx =
				await this.driftClient.getInitializeInsuranceFundStakeIx(0);
			instructions.push(initIFStakeIx);
		}

		const claimIx = this.program.instruction.claimWinnings(shares ?? null, {
			accounts: {
				...accounts,
				authority: this.program.provider.publicKey,
			},
		});
		instructions.push(claimIx);

		return await this.createAndSendTxn(instructions);
	}

	public async settleCompetitor(
		competition: PublicKey,
		competitor: PublicKey,
		userStats: PublicKey
	): Promise<TransactionSignature> {
		return await this.program.methods
			.settleCompetitor()
			.accounts({
				competition,
				competitor,
				driftUserStats: userStats,
			})
			.rpc();
	}

	public async settleAllCompetitors(competition: PublicKey, roundNumber: BN, chunkSize=10): Promise<void> {
		const competitorProgramAccounts =
			await this.program.account.competitor.all();
		let instructions = [];

		for (const competitor of competitorProgramAccounts) {
			if (competitor.account.competition.equals(competition)) {
				if(roundNumber && !competitor.account.roundNumber.eq(roundNumber)) {
						continue;
				}
				const initCompetitorIx = this.program.instruction.settleCompetitor({
					accounts: {
						competition: competitor.account.competition,
						competitor: competitor.publicKey,
						driftUserStats: competitor.account.userStats,
						keeper: this.program.provider.publicKey,
					},
				});
				instructions.push(initCompetitorIx);
				if (instructions.length >= chunkSize) {

					// no need to await
					this.createAndSendTxn(instructions, {
						computeUnitParams: {
							units: 1_400_000,
						},
					});
					instructions = []
				}

			}
		}
	}

	public async settleNextWinner(competition: PublicKey): Promise<void> {
		const competitionAccount = await this.program.account.competition.fetch(
			competition
		);

		if (competitionAccount.winnerRandomness.gt(ZERO)) {
			const spotMarket = await getSpotMarketPublicKey(
				this.driftClient.program.programId,
				QUOTE_SPOT_MARKET_INDEX
			);
			const insuranceFundVault = await getInsuranceFundVaultPublicKey(
				this.driftClient.program.programId,
				QUOTE_SPOT_MARKET_INDEX
			);

			const competitorProgramAccounts =
				await this.program.account.competitor.all();

			for (const competitor of competitorProgramAccounts) {
				if (
					competitor.account.competition.equals(competition) &&
					competitor.account.minDraw.lt(competitionAccount.winnerRandomness) &&
					competitor.account.maxDraw.gte(competitionAccount.winnerRandomness)
				) {
					const txSig = await this.program.methods
						.settleWinner()
						.accounts({
							competition,
							competitor: competitor.publicKey,
							driftUserStats: competitor.account.userStats,
							spotMarket,
							insuranceFundVault,
						})
						.rpc();
					console.log(
						`Settled winner authority ${competitor.account.authority.toBase58()}:`,
						txSig
					);
				}
			}
		}
	}

	public async requestRandomness(
		competition: PublicKey,
		bounty?: BN
	): Promise<TransactionSignature> {
		const switchboardProgram = await SwitchboardProgram.fromProvider(
			// @ts-ignore
			this.program.provider
		);

		const genesisHash =
			await switchboardProgram.provider.connection.getGenesisHash();
		const attestationQueueAddress =
			genesisHash === MAINNET_GENESIS_HASH
				? new PublicKey('2ie3JZfKcvsRLsJaP5fSo43gUo1vsurnUAtAgUdUAiDG')
				: genesisHash === DEVNET_GENESIS_HASH
				? new PublicKey('CkvizjVnm2zA5Wuwan34NhVT3zFc7vqUyGnA6tuEF5aE')
				: undefined;

		const competitionAccount = await this.program.account.competition.fetch(
			competition
		);

		const spotMarket = await getSpotMarketPublicKey(
			this.driftClient.program.programId,
			QUOTE_SPOT_MARKET_INDEX
		);
		const insuranceFundVault = await getInsuranceFundVaultPublicKey(
			this.driftClient.program.programId,
			QUOTE_SPOT_MARKET_INDEX
		);

		return await this.program.methods
			.requestRandomness(bounty ?? null)
			.accounts({
				competition,
				switchboard: switchboardProgram.attestationProgramId,
				switchboardState: switchboardProgram.attestationProgramState.publicKey,
				switchboardAttestationQueue: attestationQueueAddress,
				switchboardFunction: competitionAccount.switchboardFunction,
				competitionAuthority: competitionAccount.competitionAuthority,
				switchboardRequest: competitionAccount.switchboardFunctionRequest,
				switchboardRequestEscrow:
					competitionAccount.switchboardFunctionRequestEscrow,
				insuranceFundVault,
				spotMarket,
			})
			.rpc();
	}

	public async settleWinner(
		competition: PublicKey,
		competitor: PublicKey,
		userStats: PublicKey
	): Promise<TransactionSignature> {
		const spotMarket = await getSpotMarketPublicKey(
			this.driftClient.program.programId,
			QUOTE_SPOT_MARKET_INDEX
		);
		const insuranceFundVault = await getInsuranceFundVaultPublicKey(
			this.driftClient.program.programId,
			QUOTE_SPOT_MARKET_INDEX
		);
		return await this.program.methods
			.settleWinner()
			.accounts({
				competitor,
				competition: competition,
				driftUserStats: userStats,
				spotMarket,
				insuranceFundVault: insuranceFundVault,
			})
			.rpc();
	}

	public getCompetitionPublicKey(name: string): PublicKey {
		const encodedName = encodeName(name);
		return getCompetitionAddressSync(this.program.programId, encodedName);
	}

	public async getCompetitionDetails(
		competition: PublicKey,
		insuranceFundVaultBalance?: BN
	) {
		const competitionAccount = await this.program.account.competition.fetch(
			competition
		);
		const quoteSpotMarketAccount = this.driftClient.getQuoteSpotMarketAccount();

		if (!insuranceFundVaultBalance) {
			insuranceFundVaultBalance = new BN(
				(
					await this.driftClient.provider.connection.getTokenAccountBalance(
						quoteSpotMarketAccount.insuranceFund.vault
					)
				).value.amount
			);
		}

		const protocolOwnedSharesRemaining = BN.max(
			quoteSpotMarketAccount.insuranceFund.totalShares
				.sub(quoteSpotMarketAccount.insuranceFund.userShares)
				.sub(competitionAccount.outstandingUnclaimedWinnings),
			ZERO
		);

		const protocolOwnedBalanceRemaining = unstakeSharesToAmount(
			protocolOwnedSharesRemaining,
			quoteSpotMarketAccount.insuranceFund.totalShares,
			insuranceFundVaultBalance
		);

		const maxPrize = BN.max(
			protocolOwnedBalanceRemaining.sub(
				competitionAccount.sponsorInfo.minSponsorAmount
			),
			ZERO
		)
			.mul(competitionAccount.sponsorInfo.maxSponsorFraction)
			.div(PERCENTAGE_PRECISION);

		const prizePools = [
			BN.min(new BN(1000).mul(QUOTE_PRECISION), maxPrize.div(new BN(10))),
			BN.min(new BN(5000).mul(QUOTE_PRECISION), maxPrize.div(new BN(2))),
			maxPrize,
		];

		return {
			roundNumber: competitionAccount.roundNumber,
			roundEndTs: competitionAccount.nextRoundExpiryTs,
			prizePools: prizePools,
		};
	}
	/**
	 * Used for UI wallet adapters compatibility
	 */
	async createAndSendTxn(
		ixs: TransactionInstruction[],
		txOpts?: {
			computeUnitParams?: SetComputeUnitLimitParams;
			noComputeBudgetIx?: boolean;
		}
	): Promise<TransactionSignature> {
		const tx = new Transaction();
		if (!txOpts?.noComputeBudgetIx) {
			tx.add(
				ComputeBudgetProgram.setComputeUnitLimit(
					txOpts?.computeUnitParams || {
						units: 400_000,
					}
				)
			);
		}
		tx.add(...ixs);
		const { txSig } = await this.driftClient.sendTransaction(
			tx,
			[],
			this.driftClient.opts
		);

		return txSig;
	}

	/**
	 * Fetch all time competition events.
	 * NOTE: THIS IS A TEMPORARY SOLUTION AND WILL BE VERY HEAVY ONCE THERE HAVE BEEN A LOT OF HISTORICAL EVENTS EMITTED.
	 */
	async getAllCompetitionEvents() {

		let logs : Awaited<ReturnType<typeof fetchLogs>>['transactionLogs'] = [];
		let fetchedAllLogs = false;
		let oldestFetchedTx: string;

		while (!fetchedAllLogs) {
			const response = await fetchLogs(
				this.driftClient.connection,
				this.program.programId,
				'confirmed',
				oldestFetchedTx
			);

			if (!response?.transactionLogs || response.transactionLogs.length === 0) {
				fetchedAllLogs = true;
				break;
			}
			
			oldestFetchedTx = response.earliestTx;

			const newLogs = response.transactionLogs;
			logs = logs.concat(newLogs);
		}

		const logParser = new LogParser(this.program);
		const events = logs.map((log) => logParser.parseEventsFromLogs(log)).flat();

		return events;
	}
}
