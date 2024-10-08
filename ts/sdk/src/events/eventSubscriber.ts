import { Connection, PublicKey, TransactionSignature } from '@solana/web3.js';
import { Program } from '@coral-xyz/anchor';

import StrictEventEmitter from 'strict-event-emitter-types';
import EventEmitter from 'events';
import {
	CompetitionsEventMap,
	DefaultEventSubscriptionOptions,
	EventSubscriberEvents,
	EventSubscriptionOptions,
	EventType,
	WrappedEvents,
} from '../types/types';
import { EventList } from './eventList';
import { getSortFn } from './sort';
import {
	LogProvider,
	PollingLogProviderConfig,
	fetchLogs,
} from '@drift-labs/sdk';
import { WebSocketLogProvider } from '@drift-labs/sdk';
import { PollingLogProvider } from '@drift-labs/sdk';
import { TxEventCache } from './txEventCache';
import { LogParser } from '../parsers';

export class EventSubscriber {
	private address: PublicKey;
	private eventListMap: Map<EventType, EventList<EventType>>;
	private txEventCache: TxEventCache;
	private awaitTxPromises = new Map<string, Promise<void>>();
	private awaitTxResolver = new Map<string, () => void>();
	private logProvider: LogProvider;
	private logParser: LogParser;
	public eventEmitter: StrictEventEmitter<EventEmitter, EventSubscriberEvents>;
	private lastSeenSlot: number;
	private lastSeenBlockTime: number | undefined;
	public lastSeenTxSig: string;

	public constructor(
		private connection: Connection,
		private program: Program,
		private options: EventSubscriptionOptions = DefaultEventSubscriptionOptions
	) {
		this.options = Object.assign({}, DefaultEventSubscriptionOptions, options);
		this.address = this.options.address ?? program.programId;
		this.txEventCache = new TxEventCache(this.options.maxTx);
		this.eventListMap = new Map<EventType, EventList<EventType>>();
		this.logParser = new LogParser(program);
		for (const eventType of this.options.eventTypes) {
			this.eventListMap.set(
				eventType,
				new EventList(
					eventType,
					this.options.maxEventsPerType,
					getSortFn(this.options.orderBy, this.options.orderDir),
					this.options.orderDir
				)
			);
		}
		this.eventEmitter = new EventEmitter();
		if (this.options.logProviderConfig.type === 'websocket') {
			this.logProvider = new WebSocketLogProvider(
				this.connection,
				this.address,
				this.options.commitment
			);
		} else {
			this.logProvider = new PollingLogProvider(
				this.connection,
				this.address,
				options.commitment,
				(this.options.logProviderConfig as PollingLogProviderConfig).frequency,
				(this.options.logProviderConfig as PollingLogProviderConfig).batchSize
			);
		}
	}

	public async subscribe(): Promise<boolean> {
		try {
			if (this.logProvider.isSubscribed()) {
				return true;
			}

			this.logProvider.subscribe((txSig, slot, logs, mostRecentBlockTime) => {
				this.handleTxLogs(txSig, slot, logs, mostRecentBlockTime);
			}, true);

			return true;
		} catch (e) {
			console.error('Error fetching previous txs in event subscriber');
			console.error(e);
			return false;
		}
	}

	private handleTxLogs(
		txSig: TransactionSignature,
		slot: number,
		logs: string[],
		mostRecentBlockTime: number | undefined
	): void {
		if (this.txEventCache.has(txSig)) {
			return;
		}

		const wrappedEvents = this.parseEventsFromLogs(txSig, slot, logs);
		for (const wrappedEvent of wrappedEvents) {
			this.eventListMap.get(wrappedEvent.eventType).insert(wrappedEvent);
		}

		// dont emit event till we've added all the events to the eventListMap
		for (const wrappedEvent of wrappedEvents) {
			this.eventEmitter.emit('newEvent', wrappedEvent);
		}

		if (this.awaitTxPromises.has(txSig)) {
			this.awaitTxPromises.delete(txSig);
			this.awaitTxResolver.get(txSig)();
			this.awaitTxResolver.delete(txSig);
		}

		if (!this.lastSeenSlot || slot > this.lastSeenSlot) {
			this.lastSeenTxSig = txSig;
		}

		if (
			this.lastSeenBlockTime === undefined ||
			mostRecentBlockTime > this.lastSeenBlockTime
		) {
			this.lastSeenBlockTime = mostRecentBlockTime;
		}

		this.txEventCache.add(txSig, wrappedEvents);
	}

	public async fetchPreviousTx(fetchMax?: boolean): Promise<void> {
		if (!this.options.untilTx && !fetchMax) {
			return;
		}

		let txFetched = 0;
		let beforeTx: TransactionSignature = undefined;
		const untilTx: TransactionSignature = this.options.untilTx;
		while (txFetched < this.options.maxTx) {
			const response = await fetchLogs(
				this.connection,
				this.address,
				this.options.commitment === 'finalized' ? 'finalized' : 'confirmed',
				beforeTx,
				untilTx
			);

			if (response === undefined) {
				break;
			}

			txFetched += response.transactionLogs.length;
			beforeTx = response.earliestTx;

			for (const { txSig, slot, logs } of response.transactionLogs) {
				this.handleTxLogs(txSig, slot, logs, response.mostRecentBlockTime);
			}
		}
	}

	public async unsubscribe(): Promise<boolean> {
		return await this.logProvider.unsubscribe();
	}

	private parseEventsFromLogs(
		txSig: TransactionSignature,
		slot: number,
		logs: string[]
	): WrappedEvents {
		const events = this.logParser
			.parseEventsFromLogs({ txSig, slot, logs })
			.filter((event) => this.eventListMap.has(event.eventType));
		return events;
	}

	public awaitTx(txSig: TransactionSignature): Promise<void> {
		if (this.awaitTxPromises.has(txSig)) {
			return this.awaitTxPromises.get(txSig);
		}

		if (this.txEventCache.has(txSig)) {
			return Promise.resolve();
		}

		const promise = new Promise<void>((resolve) => {
			this.awaitTxResolver.set(txSig, resolve);
		});
		this.awaitTxPromises.set(txSig, promise);
		return promise;
	}

	public getEventList<Type extends keyof CompetitionsEventMap>(
		eventType: Type
	): EventList<Type> {
		return this.eventListMap.get(eventType) as EventList<Type>;
	}

	/**
	 * This requires the EventList be cast to an array, which requires reallocation of memory.
	 * Would bias to using getEventList over getEvents
	 *
	 * @param eventType
	 */
	public getEventsArray<Type extends EventType>(
		eventType: Type
	): CompetitionsEventMap[Type][] {
		return this.eventListMap
			.get(eventType)
			.toArray() as CompetitionsEventMap[Type][];
	}

	public getEventsByTx(txSig: TransactionSignature): WrappedEvents | undefined {
		return this.txEventCache.get(txSig);
	}
}
