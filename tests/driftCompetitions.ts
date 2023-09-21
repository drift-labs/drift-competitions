import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { DriftCompetitions } from '../target/types/drift_competitions';
import {
	decodeName,
	encodeName,
	getCompetitionAddressSync,
	getCompetitorAddressSync,
} from '../ts/sdk/src';
import { Keypair } from '@solana/web3.js';
import { assert } from 'chai';
import { AdminClient, BN, ONE, ZERO } from '@drift-labs/sdk';
import {
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

		await adminClient.program.instruction.initializeUserStats({
			accounts: {
				userStats: userStatsKey,
				authority: adminClient.wallet.publicKey,
				payer: adminClient.wallet.publicKey,
				rent: anchor.web3.SYSVAR_RENT_PUBKEY,
				systemProgram: adminClient.program.programId,
				state: await adminClient.getStatePublicKey(),
			},
		});

		console.log('userStatsKey:', userStatsKey.toString());
		console.log('authority:', adminClient.wallet.publicKey.toString());

		const tx = await program.methods
			.initializeCompetitor()
			.accounts({
				competitor: competitorAddress,
				competition: competitionAddress,
				driftUserStats: userStatsKey,
			})
			.rpc();

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
	});
});
