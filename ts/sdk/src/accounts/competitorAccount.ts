import { BulkAccountLoader, PublicKey } from '@drift-labs/sdk';
import { PollingCompetitorSubscriber } from '../accountSubscribers/pollingCompetitorSubscriber';
import { Competitor, CompetitorAccountEvents } from '../types/types';
import { ProgramAccountBase } from './programAccountBase';
import { Program } from '@coral-xyz/anchor';
import { DriftCompetitions } from '../types/drift_competitions';
import { getCompetitorAddressSync } from '../addresses';

export class CompetitorAccount extends ProgramAccountBase<
	Competitor,
	CompetitorAccountEvents
> {
	constructor(
		program: Program<DriftCompetitions>,
		competitorPubkey: PublicKey,
		accountLoader: BulkAccountLoader,
		accountSubscriptionType: 'polling' | 'websocket' = 'polling'
	) {
		super();

		if (accountSubscriptionType === 'polling') {
			this.accountSubscriber = new PollingCompetitorSubscriber(
				program,
				competitorPubkey,
				accountLoader
			);
		} else {
			throw new Error('Websocket subscription not yet implemented');
		}
	}

	/**
	 * Finds the PDA of a competitor in a competition for the given authority.
	 */
	static getCompetitorPubKeyForCompetition(
		programId: PublicKey,
		competitionPublicKey: PublicKey,
		authority: PublicKey
	) {
		return getCompetitorAddressSync(programId, competitionPublicKey, authority);
	}
}
