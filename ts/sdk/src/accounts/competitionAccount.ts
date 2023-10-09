import { BulkAccountLoader, PublicKey } from '@drift-labs/sdk';
import { PollingCompetitionSubscriber } from '../accountSubscribers/pollingCompetitionSubscriber';
import { DriftCompetitions } from '../types/drift_competitions';
import { Competition, CompetitionAccountEvents } from '../types/types';
import { ProgramAccountBase } from './programAccountBase';
import { Program } from '@coral-xyz/anchor';
import { getCompetitionAddressSync } from '../addresses';
import { encodeName } from '../name';

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

	/**
	 * Get public key of competition account from competition name.
	 *
	 * @param programId Competition program ID
	 * @param name Name of competition
	 * @returns Public key of competition account
	 */
	static getPubKeyFromName(programId: PublicKey, name: string): PublicKey {
		const encodedName = encodeName(name);
		return getCompetitionAddressSync(programId, encodedName);
	}
}
