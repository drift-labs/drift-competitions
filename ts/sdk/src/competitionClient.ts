import {
	BN,
	DriftClient,
	getInsuranceFundStakeAccountPublicKey,
	getInsuranceFundVaultPublicKey,
	getSpotMarketPublicKey,
	QUOTE_SPOT_MARKET_INDEX,
	ZERO,
	unstakeSharesToAmount,
	QUOTE_PRECISION,
} from '@drift-labs/sdk';
import { AnchorProvider, Program } from '@coral-xyz/anchor';
import { DriftCompetitions, IDL } from './types/drift_competitions';
import { PublicKey, TransactionSignature } from '@solana/web3.js';
import { encodeName } from './name';
import {
	getCompetitionAddressSync,
	getCompetitionAuthorityAddressSync,
	getCompetitorAddressSync,
} from './addresses';
import {
	AttestationQueueAccount,
	DEVNET_GENESIS_HASH,
	FunctionRequestAccount,
	MAINNET_GENESIS_HASH,
	SwitchboardProgram,
} from '@switchboard-xyz/solana.js';
import * as anchor from '@coral-xyz/anchor';

const defaultProgramId = new PublicKey(
	'8yCtqd9UetSttHhbGJFRk3MpcQny3bNvEfv5YrADuHEp'
);

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
			program = new Program(IDL, defaultProgramId, driftClient.provider);
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

	public async initializeCompetitor(
		competition: PublicKey
	): Promise<TransactionSignature> {
		const competitor = getCompetitorAddressSync(
			this.program.programId,
			competition,
			this.program.provider.publicKey
		);

		return await this.program.methods
			.initializeCompetitor()
			.accounts({
				competitor,
				competition: competition,
				driftUserStats: this.driftClient.getUserStatsAccountPublicKey(),
			})
			.rpc();
	}

	public async claimEntry(
		competition: PublicKey
	): Promise<TransactionSignature> {
		const competitor = getCompetitorAddressSync(
			this.program.programId,
			competition,
			this.program.provider.publicKey
		);

		return await this.program.methods
			.claimEntry()
			.accounts({
				competitor,
				competition: competition,
				driftUserStats: this.driftClient.getUserStatsAccountPublicKey(),
				instructions: anchor.web3.SYSVAR_INSTRUCTIONS_PUBKEY,
			})
			.rpc();
	}

	public async claimWinnings(
		competition: PublicKey,
		shares?: BN
	): Promise<TransactionSignature> {
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

		return await this.program.methods
			.claimWinnings(shares ?? null)
			.accounts({
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
			})
			.rpc();
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

	public async settleAllCompetitors(competition: PublicKey): Promise<void> {
		const competitorProgramAccounts =
			await this.program.account.competitor.all();

		for (const competitor of competitorProgramAccounts) {
			if (competitor.account.competition.equals(competition)) {
				const txSig = await this.settleCompetitor(
					competition,
					competitor.publicKey,
					competitor.account.userStats
				);
				console.log(
					`Settled authority ${competitor.account.authority.toBase58()}:`,
					txSig
				);
			}
		}
	}

	public async settleNextWinner(competition: PublicKey): Promise<void> {
		const competitionAccount = await this.program.account.competition.fetch(
			competition
		);

		const winnerDraw = competitionAccount.winnerRandomness;

		if (winnerDraw.gt(ZERO)) {
			const spotMarket = this.driftClient.getQuoteSpotMarketAccount().pubkey;

			const competitorProgramAccounts =
				await this.program.account.competitor.all();

			for (const competitor of competitorProgramAccounts) {
				if (
					competitor.account.competition.equals(competition) &&
					competitor.account.minDraw.lt(winnerDraw) &&
					competitor.account.maxDraw.gte(winnerDraw)
				) {
					const txSig = await this.program.methods
						.settleWinner()
						.accounts({
							competition,
							competitor: competitor.publicKey,
							driftUserStats: competitor.account.userStats,
							spotMarket,
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

		return await this.program.methods
			.settleWinner()
			.accounts({
				competitor,
				competition: competition,
				driftUserStats: userStats,
				spotMarket,
			})
			.rpc();
	}

	public getCompetitionPublicKey(name: string): PublicKey {
		const encodedName = encodeName(name);
		return getCompetitionAddressSync(this.program.programId, encodedName);
	}

	public async getCompetitionDetails(
		competition: PublicKey,
		insuranceFundVaultBalance: BN
	) {
		const competitionAccount = await this.program.account.competition.fetch(
			competition
		);

		const quoteSpotMarketAccount = this.driftClient.getQuoteSpotMarketAccount();
		const protocolOwnedShares =
			quoteSpotMarketAccount.insuranceFund.totalShares.sub(
				quoteSpotMarketAccount.insuranceFund.userShares
			);

		const protocolOwnedBalance = unstakeSharesToAmount(
			protocolOwnedShares,
			quoteSpotMarketAccount.insuranceFund.totalShares,
			insuranceFundVaultBalance
		);

		const maxPrize = protocolOwnedBalance
			.sub(competitionAccount.sponsorInfo.minSponsorAmount)
			.mul(competitionAccount.sponsorInfo.maxSponsorFraction);

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
}
