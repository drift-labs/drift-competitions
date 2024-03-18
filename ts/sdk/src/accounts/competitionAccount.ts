import { BulkAccountLoader, PublicKey } from '@drift-labs/sdk';
import { PollingCompetitionSubscriber } from '../accountSubscribers/pollingCompetitionSubscriber';
import { DriftCompetitions } from '../types/drift_competitions';
import { Competition, CompetitionAccountEvents } from '../types/types';
import { ProgramAccountBase } from './programAccountBase';
import { Program } from '@coral-xyz/anchor';

export class CompetitionAccount extends ProgramAccountBase<
	Competition,
	CompetitionAccountEvents
> {
	constructor(
		program: Program<DriftCompetitions>,
		competitionPubkey: PublicKey,
		accountLoader: BulkAccountLoader,
		accountSubscriptionType: 'polling' | 'websocket' = 'polling'
	) {
		super();

		if (accountSubscriptionType === 'polling') {
			this.accountSubscriber = new PollingCompetitionSubscriber(
				program,
				competitionPubkey,
				accountLoader
			);
		} else {
			throw new Error('Websocket subscription not yet implemented');
		}
	}
}
