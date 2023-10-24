import { Program } from '@coral-xyz/anchor';
import { TransactionSignature } from '@solana/web3.js';
import { WrappedEvents } from '../types/types';

export type EventLog = { txSig: TransactionSignature; slot: number; logs: string[] };

export class LogParser {
	constructor(private program: Program) {}

	public parseEventsFromLogs(eventLogs: EventLog): WrappedEvents {
		const records: WrappedEvents = [];

		if (!eventLogs.logs) return records;

		// @ts-ignore
		const eventGenerator = this.program._events._eventParser.parseLogs(
			eventLogs.logs,
			false
		);

		for (const eventLog of eventGenerator) {
			eventLog.data.txSig = eventLogs.txSig;
			eventLog.data.slot = eventLogs.slot;
			eventLog.data.eventType = eventLog.name;
			records.push(eventLog.data);
		}

		return records;
	}
}
