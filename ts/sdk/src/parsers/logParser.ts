import { Program } from '@coral-xyz/anchor';
import {
	TransactionResponse,
	TransactionSignature,
	VersionedTransactionResponse,
} from '@solana/web3.js';
import { WrappedEvents } from '../types/types';
import { parseLogs } from '@drift-labs/sdk';

const programId = 'DraWMeQX9LfzQQSYoeBwHAgM5JcqFkgrX7GbTfjzVMVL';
export type EventLog = {
	txSig: TransactionSignature;
	slot: number;
	logs: string[];
};

function mapTransactionResponseToLog(
	transaction: TransactionResponse | VersionedTransactionResponse
): EventLog {
	return {
		txSig: transaction.transaction.signatures[0],
		slot: transaction.slot,
		logs: transaction.meta.logMessages,
	};
}

export class LogParser {
	constructor(private program: Program) {}

	public parseEventsFromLogs(eventLogs: EventLog): WrappedEvents {
		const records: WrappedEvents = [];

		if (!eventLogs.logs) return records;

		// @ts-ignore
		const parsedLogs = parseLogs(this.program, eventLogs.logs, programId);

		for (const eventLog of parsedLogs) {
			eventLog.data.txSig = eventLogs.txSig;
			eventLog.data.slot = eventLogs.slot;
			eventLog.data.eventType = eventLog.name;
			// @ts-ignore
			records.push(eventLog.data);
		}

		return records;
	}

	public parseEventsFromTransaction(
		transaction: TransactionResponse
	): WrappedEvents {
		const transactionLogObject = mapTransactionResponseToLog(transaction);

		return this.parseEventsFromLogs(transactionLogObject);
	}
}
