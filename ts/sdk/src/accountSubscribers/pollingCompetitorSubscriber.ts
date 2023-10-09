import {
	Competitor,
	CompetitorAccountEvents,
	CompetitorAccountSubscriber,
} from '../types/types';
import { PollingProgramAccountSubscriberBase } from './pollingProgramAccountSubscriberBase';

export class PollingCompetitorSubscriber
	extends PollingProgramAccountSubscriberBase<
		Competitor,
		CompetitorAccountEvents
	>
	implements CompetitorAccountSubscriber
{
	async addToAccountLoader(): Promise<void> {
		if (this.callbackId) {
			console.log('Account for competitor already added to account loader');
			return;
		}

		this.callbackId = await this.accountLoader.addAccount(
			this.pubkey,
			(buffer, slot) => {
				if (!buffer) return;

				if (this.account && this.account.slot > slot) {
					return;
				}

				const account = this.program.account.competitor.coder.accounts.decode(
					'competitor',
					buffer
				);
				this.account = { data: account, slot };
				this._eventEmitter.emit('competitorUpdate', account);
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
			const account = this.program.account.competitor.coder.accounts.decode(
				'competitor',
				buffer
			);
			this.account = { data: account, slot };
		}
	}

	updateData(competitorAcc: Competitor, slot: number): void {
		if (!this.account || this.account.slot < slot) {
			this.account = { data: competitorAcc, slot };
			this._eventEmitter.emit('competitorUpdate', competitorAcc);
			this._eventEmitter.emit('update');
		}
	}
}
