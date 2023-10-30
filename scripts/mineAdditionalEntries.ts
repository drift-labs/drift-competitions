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

async function mineEntries(provider, authority: PublicKey, n: number) {
	// Configure client to use the provider.
	anchor.setProvider(provider);

	const payer = (provider.wallet as anchor.Wallet).payer;
	console.log(`Payer: ${payer.publicKey}`);
	console.log(`Recipient: ${authority}`);

	const driftClient = new DriftClient({
		connection: provider.connection,
		env: ENV,
		wallet: provider.wallet,
		authority: authority,
	});

	const competitionClient = new CompetitionsClient({
		driftClient,
	});

	const program = competitionClient.program as Program<DriftCompetitions>;

	const name = 'sweepstakes';
	const competitionKey = competitionClient.getCompetitionPublicKey(name);
	let competitionAccount = await program.account.competition.fetch(
		competitionKey
	);

	await competitionClient.driftClient.subscribe();
	const details = await competitionClient.getCompetitionDetails(competitionKey);

	console.log('max prize: ', details.prizePools[2].toNumber() / 1e6);

	if (isVariant(competitionAccount.status, 'active')) {
		const competitorKey = getCompetitorAddressSync(
			competitionClient.program.programId,
			competitionKey,
			authority
		);

		const competitorAccount = await program.account.competitor.fetch(
			competitorKey
		);
		const userStatsKey = driftClient.getUserStatsAccountPublicKey();
		console.log(
			'current bonusScore =',
			competitorAccount.bonusScore.toNumber()
		);
		for (let i = 1; i <= n; i++) {
			console.log('claiming! entry:', i, '/', n);
			try {
				competitionClient.claimEntry(
					competitionKey,
					competitorKey,
					userStatsKey
				);
				await sleep(1000);
			} catch {
				console.error('skip');
			}
		}
	} else {
		console.log(
			'competition is under resolution. cannot mine additional entries.'
		);
	}

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
	.option('-n, --number-of-entries <number>', 'number of entries to mine')
	.parse();
const opts = program.opts();

console.log('RPC:', RPC_ENDPOINT);
mineEntries(
	anchor.AnchorProvider.local(RPC_ENDPOINT, {
		preflightCommitment: 'confirmed',
		skipPreflight: true,
		commitment: 'confirmed',
	}),
	new PublicKey(opts.authority),
	opts.n ?? 5
);
