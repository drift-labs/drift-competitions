import {
	Competition,
	CompetitionAccountEvents,
	CompetitionAccountSubscriber,
} from '../types/types';
import { PollingProgramAccountSubscriberBase } from './pollingProgramAccountSubscriberBase';

export class PollingCompetitionSubscriber
	extends PollingProgramAccountSubscriberBase<
		Competition,
		CompetitionAccountEvents
	>
	implements CompetitionAccountSubscriber
{
	async addToAccountLoader(): Promise<void> {
		if (this.callbackId) {
			console.log('Account for competition already added to account loader');
			return;
		}

		this.callbackId = await this.accountLoader.addAccount(
			this.pubkey,
			(buffer, slot) => {
				if (!buffer) return;

				if (this.account && this.account.slot > slot) {
					return;
				}

				const account = this.program.account.competition.coder.accounts.decode(
					'competition',
					buffer
				);
				this.account = { data: account, slot };
				this._eventEmitter.emit('competitionUpdate', account);
				this._eventEmitter.emit('update');
			}
		);

		this.errorCallbackId = this.accountLoader.addErrorCallbacks((error) => {
			this._eventEmitter.emit('error', error);
		});
	}

	async fetch(): Promise<void> {
		await this.accountLoader.load();
		const { buffer, slot } = this.accountLoader.getBufferAndSlot(this.pubkey);
		const currentSlot = this.account?.slot ?? 0;
		if (buffer && slot > currentSlot) {
			const account = this.program.account.competition.coder.accounts.decode(
				'competition',
				buffer
			);
			this.account = { data: account, slot };
		}
	}

	updateData(competitionAcc: Competition, slot: number): void {
		if (!this.account || this.account.slot < slot) {
			this.account = { data: competitionAcc, slot };
			this._eventEmitter.emit('competitionUpdate', competitionAcc);
			this._eventEmitter.emit('update');
		}
	}
}
