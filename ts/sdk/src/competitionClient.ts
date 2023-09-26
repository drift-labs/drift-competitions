import {
	BN,
	DriftClient,
	getInsuranceFundStakeAccountPublicKey, getInsuranceFundVaultPublicKey,
	getSpotMarketPublicKey,
	QUOTE_SPOT_MARKET_INDEX
} from '@drift-labs/sdk';
import { AnchorProvider, Program } from '@coral-xyz/anchor';
import { DriftCompetitions, IDL } from './types/drift_competitions';
import { PublicKey, TransactionSignature } from '@solana/web3.js';
import { encodeName } from './name';
import {
	getCompetitionAddressSync,
	getCompetitionAuthorityAddressSync, getCompetitorAddressSync,
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
	'9FHbMuNCRRCXsvKEkA3V8xJmysAqkZrGfrppvUhGTq7x'
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
	}: {
		name: string;
		nextRoundExpiryTs: BN;
		competitionExpiryTs: BN;
		roundDuration: BN;
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
		}: {
			nextRoundExpiryTs?: BN | null;
			competitionExpiryTs?: BN | null;
			roundDuration?: BN | null;
		}
	): Promise<TransactionSignature> {
		return await this.program.methods
			.updateCompetition({
				nextRoundExpiryTs,
				competitionExpiryTs,
				roundDuration,
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
		competition: PublicKey,
		userStats: PublicKey
	): Promise<TransactionSignature> {
		const competitor = getCompetitorAddressSync(
			this.program.programId,
			competition,
			this.program.provider.publicKey,
		);

		return await this.program.methods
			.initializeCompetitor()
			.accounts({
				competitor,
				competition: competition,
				driftUserStats: userStats,
			})
			.rpc();
	}

	public async claimEntry(
		competition: PublicKey,
		userStats: PublicKey
	): Promise<TransactionSignature> {
		const competitor = getCompetitorAddressSync(
			this.program.programId,
			competition,
			this.program.provider.publicKey,
		);

		return await this.program.methods
			.claimEntry()
			.accounts({
				competitor,
				competition: competition,
				driftUserStats: userStats,
				instructions: anchor.web3.SYSVAR_INSTRUCTIONS_PUBKEY
			})
			.rpc();
	}

	public async claimWinnings(
		competition: PublicKey,
		userStats: PublicKey,
		shares?: BN,
	): Promise<TransactionSignature> {
		const competitor = getCompetitorAddressSync(
			this.program.programId,
			competition,
			this.program.provider.publicKey,
		);

		const spotMarket = await getSpotMarketPublicKey(this.program.provider.publicKey, QUOTE_SPOT_MARKET_INDEX);
		const insuranceFundVault = await getInsuranceFundVaultPublicKey(this.program.provider.publicKey, QUOTE_SPOT_MARKET_INDEX);
		const insuranceFundStake = await getInsuranceFundStakeAccountPublicKey(this.driftClient.program.programId, this.program.provider.publicKey, QUOTE_SPOT_MARKET_INDEX);

		return await this.program.methods
			.claimWinnings(shares ?? null)
			.accounts({
				competitor,
				competition: competition,
				driftUserStats: userStats,
				spotMarket,
				insuranceFundStake,
				insuranceFundVault
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

	public async requestRandomness(
		competition: PublicKey
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


		const spotMarket = await getSpotMarketPublicKey(this.program.provider.publicKey, QUOTE_SPOT_MARKET_INDEX);
		const insuranceFundVault = await getInsuranceFundVaultPublicKey(this.program.provider.publicKey, QUOTE_SPOT_MARKET_INDEX);

		return await this.program.methods
			.requestRandomness()
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
		return await this.program.methods
			.settleWinner()
			.accounts({
				competitor,
				competition: competition,
				driftUserStats: userStats,
			})
			.rpc();
	}
}
