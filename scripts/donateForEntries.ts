import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { DriftCompetitions } from '../ts/sdk/src/types/drift_competitions';
import { CompetitionsClient } from '../ts/sdk/src';
import { DriftClient, PublicKey, isVariant } from '@drift-labs/sdk';
import dotenv from 'dotenv';
import { getCompetitorAddressSync } from '../ts/sdk/src/addresses';
import { program, Option } from 'commander';

dotenv.config();

function sleep(ms) {
	return new Promise((resolve) => setTimeout(resolve, ms));
}
const ENV = 'mainnet-beta';

const RPC_ENDPOINT =
	process.env.RPC_OVERRIDE ?? 'https://api.' + ENV + '.solana.com';

async function donateForEntries(
	provider,
	authority: PublicKey,
	t: number,
	m: number
) {
	// Configure client to use the provider.
	anchor.setProvider(provider);

	const payer = (provider.wallet as anchor.Wallet).payer;
	console.log(`Payer: ${payer.publicKey}`);
	console.log(`Recipient: ${authority}`);

	console.log('m:', m, 't:', t);

	const marketIndex = Number(m);
	const tokenAmountNoPrecision = Number(t);

	const driftClient = new DriftClient({
		connection: provider.connection,
		wallet: provider.wallet,
		authority: authority,
		spotMarketIndexes: [marketIndex],
	});

	await driftClient.subscribe();

	const competitionClient = new CompetitionsClient({
		driftClient,
	});

	const program = competitionClient.program as Program<DriftCompetitions>;

	const name = 'sweepstakes';
	const competitionKey = competitionClient.getCompetitionPublicKey(name);

	const competitorKey = getCompetitorAddressSync(
		competitionClient.program.programId,
		competitionKey,
		authority
	);

	const spotMarket =
		competitionClient.driftClient.getSpotMarketAccount(marketIndex);

	const tokenAmount = new anchor.BN(tokenAmountNoPrecision * 10 ** 6);

	const entries = competitionClient.getEntriesForDonation(
		tokenAmount,
		spotMarket
	);

	console.log('entries:', entries.toNumber());

	const tokenAccount = await driftClient.getAssociatedTokenAccount(marketIndex);

	const txSig = await competitionClient.claimMultipleEntries(
		entries,
		tokenAccount,
		competitionKey,
		competitorKey
	);

	console.log(txSig);

	console.log('DONE!');
}

if (!process.env.ANCHOR_WALLET) {
	throw new Error('ANCHOR_WALLET must be set.');
}
require('dotenv').config();
program
	.option(
		'--authority <string>',
		'authority of competition account for mining entries'
	)
	.option('-t, --number-of-tokens <number>', 'number of tokens to donate')
	.option('-m, --market-index <number>', 'market index to donate for')
	.parse();
const opts = program.opts();

if (!opts.authority) {
	throw new Error('authority not set');
}

if (!opts.numberOfTokens) {
	throw new Error('number of tokens not set');
}

if (!opts.marketIndex) {
	throw new Error('market index not set');
}

console.log('RPC:', RPC_ENDPOINT);
donateForEntries(
	anchor.AnchorProvider.local(RPC_ENDPOINT, {
		preflightCommitment: 'confirmed',
		skipPreflight: true,
		commitment: 'confirmed',
	}),
	new PublicKey(opts.authority),
	opts.numberOfTokens,
	opts.marketIndex
);
