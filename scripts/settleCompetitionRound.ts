import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { DriftCompetitions } from '../target/types/drift_competitions';
import {
	CompetitionsClient,
} from '../ts/sdk/src';
import { DriftClient, isVariant } from '@drift-labs/sdk';

function sleep(ms) {
	return new Promise((resolve) => setTimeout(resolve, ms));
}

async function settleSweepstakesCompetition(provider) {
	// Configure client to use the provider.
	anchor.setProvider(provider);

	const payer = (provider.wallet as anchor.Wallet).payer;
	console.log(`PAYER: ${payer.publicKey}`);

	const program = anchor.workspace
		.DriftCompetitions as Program<DriftCompetitions>;

	console.log('program.programId:', program.programId.toString());
	const driftClient = new DriftClient({
		connection: provider.connection,
		env: 'devnet',
		wallet: provider.wallet,
	});

	const competitionClient = new CompetitionsClient({
		program,
		driftClient,
	});

	const name = 'sweepstakes';
	const competitionKey = competitionClient.getCompetitionPublicKey(name);
	let competitionAccount = await program.account.competition.fetch(
		competitionKey
	);

	await competitionClient.driftClient.subscribe();
	const details = await competitionClient.getCompetitionDetails(competitionKey);

	console.log('max prize: ', details.prizePools[2].toNumber() / 1e6);

	if (isVariant(competitionAccount.status, 'active')) {
		while (
			competitionAccount.nextRoundExpiryTs.toNumber() >
			Date.now() / 1000
		) {
			const timeTilEnd = Math.round(
				competitionAccount.nextRoundExpiryTs.toNumber() - Date.now() / 1000
			);
			console.log('waiting for roundExpiry in ', timeTilEnd, 'seconds');
			await sleep(timeTilEnd * 1000);
		}

		if (
			!competitionAccount.numberOfCompetitors.eq(
				competitionAccount.numberOfCompetitorsSettled
			)
		) {
			const txSig = await competitionClient.settleAllCompetitors(
				competitionKey,
				competitionAccount.roundNumber
			);
			console.log(txSig);
		}

		const txSig2 = await competitionClient.requestRandomness(
			competitionClient.getCompetitionPublicKey(name)
		);
		console.log(txSig2);
		await sleep(2000);
	} else if (isVariant(competitionAccount.status, 'winnerAndPrizeRandomnessRequested')) {
		const txSig2 = await competitionClient.requestRandomness(
			competitionClient.getCompetitionPublicKey(name)
		);
		console.log(txSig2);
		await sleep(2000);
	}

	let isReadyForSettlement = isVariant(
		competitionAccount.status,
		'winnerAndPrizeRandomnessComplete'
	);

	while(!isReadyForSettlement){
		await sleep(1000);
		competitionAccount = await program.account.competition.fetch(
		competitionKey
		);
		isReadyForSettlement = isVariant(
			competitionAccount.status,
			'winnerAndPrizeRandomnessComplete'
		);
	}
	

	if (isReadyForSettlement) {
		while (isReadyForSettlement) {
			const txSig = await competitionClient.settleNextWinner(
				competitionClient.getCompetitionPublicKey(name)
			);

			console.log(txSig);

			await sleep(1000);
			competitionAccount = await program.account.competition.fetch(
				competitionKey
			);
			isReadyForSettlement = isVariant(
				competitionAccount.status,
				'winnerAndPrizeRandomnessComplete'
			);
		}
	}
}

try {
	if (!process.env.ANCHOR_WALLET) {
		throw new Error('ANCHOR_WALLET must be set.');
	}
	settleSweepstakesCompetition(
		anchor.AnchorProvider.local(
			'https://api.devnet.solana.com',
			{
				preflightCommitment: 'confirmed',
				skipPreflight: true,
				commitment: 'confirmed',
			}
		)
	);
} catch (e) {
	console.error(e);
}
