import EventEmitter from 'events';
import StrictEventEmitter from 'strict-event-emitter-types';
import {
	DriftCompetitionsProgramAccountBaseEvents,
	DriftCompetitionsProgramAccountSubscriber,
} from '../types/types';

export abstract class ProgramAccountBase<
	Account,
	AccountEvents extends DriftCompetitionsProgramAccountBaseEvents
> {
	accountSubscriber: DriftCompetitionsProgramAccountSubscriber<
		Account,
		AccountEvents
	>;

	get isSubscribed(): boolean {
		return this.accountSubscriber.isSubscribed;
	}

	get eventEmitter(): StrictEventEmitter<EventEmitter, AccountEvents> {
		return this.accountSubscriber.eventEmitter;
	}

	async subscribe(): Promise<boolean> {
		return await this.accountSubscriber.subscribe();
	}

	async unsubscribe(): Promise<void> {
		return await this.accountSubscriber.unsubscribe();
	}

	getData(): Account {
		return this.accountSubscriber.getAccountAndSlot()?.data;
	}

	async updateData(newData: Account, slot: number): Promise<void> {
		return await this.accountSubscriber.updateData(newData, slot);
	}
}
