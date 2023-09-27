import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import {
	decodeName,
	encodeName,
	getCompetitionAddressSync,
	getCompetitorAddressSync,
} from '../ts/sdk/src';
import { DriftCompetitions } from '../target/types/drift_competitions';

import { ComputeBudgetProgram, Transaction } from '@solana/web3.js';

import { CompetitionsClient } from '../ts/sdk/src/competitionClient';
import {
	BN,
	AdminClient,
	ONE,
	ZERO,
	DriftClient,
	getInsuranceFundStakeAccountPublicKey,
	getInsuranceFundVaultPublicKey,
	getSpotMarketPublicKey,
	QUOTE_SPOT_MARKET_INDEX,
	TWO,
} from '@drift-labs/sdk';
import { Keypair } from '@solana/web3.js';
import { assert } from 'chai';
import {
	createUserWithUSDCAccount,
	initializeQuoteSpotMarket,
	mockUSDCMint,
	mockUserUSDCAccount,
	// printTxLogs,
} from './testHelpers';

describe('drift competitions', () => {
	// Configure the client to use the local cluster.
	const provider = anchor.AnchorProvider.local(undefined, {
		preflightCommitment: 'confirmed',
		skipPreflight: false,
		commitment: 'confirmed',
	});

	anchor.setProvider(provider);

	const adminClient = new AdminClient({
		connection: provider.connection,
		wallet: provider.wallet,
	});

	const program = anchor.workspace
		.DriftCompetitions as Program<DriftCompetitions>;

	let usdcMint: Keypair;
	let userUSDCAccount: Keypair;
	const usdcAmount = new BN(1000 * 10 ** 6);

	before(async () => {
		usdcMint = await mockUSDCMint(provider);
		userUSDCAccount = await mockUserUSDCAccount(usdcMint, usdcAmount, provider);
		await adminClient.initialize(usdcMint.publicKey, false);
		await adminClient.subscribe();
		await adminClient.initializeUserAccount();
		await initializeQuoteSpotMarket(adminClient, usdcMint.publicKey);
	});

	after(async () => {
		await adminClient.unsubscribe();
	});

	it('initialize competition', async () => {
		const name = 'test';
		const encodedName = encodeName(name);

		const competitionAddress = getCompetitionAddressSync(
			program.programId,
			encodedName
		);

		// Add your test here.
		const tx = await program.methods
			.initializeCompetition({
				name: encodeName(name),
			})
			.accounts({
				competition: competitionAddress,
			})
			.rpc();

		const competitionAccount = await program.account.competition.fetch(
			competitionAddress
		);
		assert(decodeName(competitionAccount.name) === name);
		// assert(competitionAccount.sponsor.equals(provider.wallet.publicKey));
	});

	it('initialize competitor', async () => {
		const name = 'test';
		const encodedName = encodeName(name);

		const competitionAddress = getCompetitionAddressSync(
			program.programId,
			encodedName
		);

		const competitionAccount = await program.account.competition.fetch(
			competitionAddress
		);
		console.log(competitionAccount);

		const authority = provider.wallet.publicKey;

		const competitorAddress = getCompetitorAddressSync(
			program.programId,
			competitionAddress,
			authority
		);

		// make user stats account
		const userStatsKey = adminClient.getUserStatsAccountPublicKey();
		console.log('userStatsKey:', userStatsKey.toString());
		console.log('authority:', adminClient.wallet.publicKey.toString());

		try {
			const tx = await program.methods
				.initializeCompetitor()
				.accounts({
					competitor: competitorAddress,
					competition: competitionAddress,
					driftUserStats: userStatsKey,
				})
				.rpc();
		} catch (e) {
			console.error(e);
		}

		const competitorAccount = await program.account.competitor.fetch(
			competitorAddress
		);
		const competitionAccountAfter = await program.account.competition.fetch(
			competitionAddress
		);
		assert(competitionAccount.numberOfCompetitors.eq(ZERO));

		assert(decodeName(competitionAccountAfter.name) === name);
		assert(competitionAccountAfter.numberOfCompetitors.eq(ONE));

		assert(competitorAccount.authority.equals(authority));
		assert(competitorAccount.competition.equals(competitionAddress));
		assert(competitorAccount.bonusScore.eq(ONE));
	});

	it('competitor claimEntry', async () => {
		const competitionClient = new CompetitionsClient({
			driftClient: adminClient,
			program: program,
		});

		const name = 'test';
		const encodedName = encodeName(name);

		const competitionAddress = getCompetitionAddressSync(
			program.programId,
			encodedName
		);

		const userStatsKey = adminClient.getUserStatsAccountPublicKey();

		await competitionClient.claimEntry(competitionAddress, userStatsKey);

		const authority = provider.wallet.publicKey;

		const competitorAddress = getCompetitorAddressSync(
			program.programId,
			competitionAddress,
			authority
		);
		let competitorAccount = await program.account.competitor.fetch(
			competitorAddress
		);
		assert(competitorAccount.bonusScore.eq(TWO));

		// cannot batch them
		const tx = new Transaction();
		// tx.add(
		// 	ComputeBudgetProgram.requestUnits({
		// 		units: 1_400_000,
		// 		additionalFee: 0,
		// 	})
		// );
		tx.add(
			program.instruction.claimEntry({
				accounts: {
					authority: authority,
					competitor: competitorAddress,
					competition: competitionAddress,
					driftUserStats: userStatsKey,
					instructions: anchor.web3.SYSVAR_INSTRUCTIONS_PUBKEY,
				},
			})
		);
		tx.add(
			program.instruction.claimEntry({
				accounts: {
					authority: authority,
					competitor: competitorAddress,
					competition: competitionAddress,
					driftUserStats: userStatsKey,
					instructions: anchor.web3.SYSVAR_INSTRUCTIONS_PUBKEY,
				},
			})
		);

		await adminClient.txSender
			.send(tx, [], adminClient.opts)
			.then((txSig) => {
				assert(false);
			})
			.catch((e) => {
				assert(String(e).includes('custom program error: 0x1770'));
				console.log(e);
			});

		competitorAccount = await program.account.competitor.fetch(
			competitorAddress
		);
		assert(competitorAccount.bonusScore.eq(TWO));
		assert(competitorAccount.competitionRoundNumber.eq(ZERO));

		const competitionAccount = await program.account.competition.fetch(
			competitionAddress
		);
		assert(competitionAccount.totalScoreSettled.eq(ZERO));
		assert(competitionAccount.numberOfCompetitorsSettled.eq(ZERO));
		assert(competitionAccount.numberOfCompetitors.eq(ONE));
	});

	it('resolve round 0 (partially)', async () => {
		const competitionClient = new CompetitionsClient({
			driftClient: adminClient,
			program: program,
		});

		const name = 'test';
		const encodedName = encodeName(name);

		const competitionAddress = getCompetitionAddressSync(
			program.programId,
			encodedName
		);

		const userStatsKey = adminClient.getUserStatsAccountPublicKey();

		await competitionClient.claimEntry(competitionAddress, userStatsKey);
		const authority = provider.wallet.publicKey;

		const competitorAddress = getCompetitorAddressSync(
			program.programId,
			competitionAddress,
			authority
		);

		await competitionClient.settleCompetitor(
			competitionAddress,
			competitorAddress,
			userStatsKey
		);

		competitionClient
			.settleCompetitor(competitionAddress, competitorAddress, userStatsKey)
			.then((txSig) => {
				assert(false);
			})
			.catch((e) => {
				assert(String(e).includes('custom program error'));
				console.log(e);
			});

		const competitorAccountAfter = await program.account.competitor.fetch(
			competitorAddress
		);
		assert(competitorAccountAfter.competitionRoundNumber.eq(ONE));
		assert(competitorAccountAfter.bonusScore.eq(ONE)); // halfed
		assert(competitorAccountAfter.previousSnapshotScore.eq(ZERO));

		const competitionAccount = await program.account.competition.fetch(
			competitionAddress
		);
		console.log(competitionAccount);

		assert(competitionAccount.maxEntriesPerCompetitor, ZERO); // not set
		assert(competitionAccount.sponsorInfo.maxSponsorFraction, ZERO); // not set
		assert(competitionAccount.sponsorInfo.minSponsorAmount, ZERO); // not set

		assert(competitionAccount.numberOfCompetitorsSettled.eq(ONE));
		assert(competitionAccount.roundNumber.eq(ZERO));
		assert(competitionAccount.totalScoreSettled.eq(new BN(3)));

		// await competitionClient.requestRandomness(competitionAddress);
	});
});
