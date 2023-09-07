import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { DriftCompetitions } from '../target/types/drift_competitions';
import {decodeName, encodeName, getCompetitionAddressSync} from "../ts/sdk/src";
import {assert} from "chai";

describe('drift competitions', () => {
	// Configure the client to use the local cluster.
	const provider = anchor.AnchorProvider.local(undefined, {
		preflightCommitment: 'confirmed',
		skipPreflight: false,
		commitment: 'confirmed',
	});

	anchor.setProvider(provider);

	const program = anchor.workspace.DriftCompetitions as Program<DriftCompetitions>;

	it('initialize competition', async () => {
		const name = "test";
		const encodedName = encodeName(name);

		const competitionAddress = getCompetitionAddressSync(program.programId, encodedName);

		// Add your test here.
		const tx = await program.methods.initializeCompetition({
			name: encodeName(name)
			}).accounts({
				competition: competitionAddress,
		}).rpc();

		const competitionAccount = await program.account.competition.fetch(competitionAddress);
		assert(decodeName(competitionAccount.name) === name);
		assert(competitionAccount.sponsor.equals(provider.wallet.publicKey));
	});
});