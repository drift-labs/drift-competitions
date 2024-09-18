import {
	BASE_PRECISION_EXP,
	BN,
	BigNum,
	PERCENTAGE_PRECISION_EXP,
	PRICE_PRECISION_EXP,
	PublicKey,
	QUOTE_PRECISION_EXP,
} from '@drift-labs/sdk';
import {
	Deserialize,
	JsonObject,
	Serialize,
	SetDeserializeKeyTransform,
	SetSerializeKeyTransform,
	SnakeCase,
	autoserializeAs,
	autoserializeAsArray,
	autoserializeUsing,
} from 'cerializr';
import {
	CompetitionResult,
	CompetitionRoundSummaryRecord,
	CompetitionRoundWinnerRecord,
	LiveCompetitionInfo,
} from '../types/types';

const BNSerializationFn = (target: BN) =>
	target ? target.toString() : undefined;
const BNDeserializationFn = (val: string) => (val ? new BN(val) : undefined);
const BNSerializeAndDeserializeFns = {
	Serialize: BNSerializationFn,
	Deserialize: BNDeserializationFn,
};

const BNArraySerializationFn = (target: BN[]) =>
	target.map((val) => (val ? val.toString() : undefined));
const BNArrayDeserializationFn = (values: string[]) =>
	values.map((val) => (val ? new BN(val) : undefined));
const BNArraySerializeAndDeserializeFns = {
	Serialize: BNArraySerializationFn,
	Deserialize: BNArrayDeserializationFn,
};

const QuoteBigNumSerializationFn = (target: BigNum | BN) =>
	target
		? target instanceof BigNum
			? target.print()
			: target.toString()
		: undefined;
const QuoteBigNumDeserializationFn = (val: string) =>
	val
		? BigNum.from(
				typeof val === 'string' ? val.replace('.', '') : val,
				QUOTE_PRECISION_EXP
		  )
		: undefined;
const QuoteBigNumSerializeAndDeserializeFns = {
	Serialize: QuoteBigNumSerializationFn,
	Deserialize: QuoteBigNumDeserializationFn,
};

const PctBigNumSerializationFn = (target: BigNum | BN) =>
	target
		? target instanceof BigNum
			? target.print()
			: target.toString()
		: undefined;

const PctBigNumDeserializationFn = (val: string) =>
	val
		? BigNum.from(
				typeof val === 'string' ? val.replace('.', '') : val,
				PERCENTAGE_PRECISION_EXP
		  )
		: undefined;

const PctBigNumSerializeAndDeserializeFns = {
	Serialize: PctBigNumSerializationFn,
	Deserialize: PctBigNumDeserializationFn,
};

const BaseBigNumSerializationFn = (target: BigNum | BN) =>
	target
		? target instanceof BigNum
			? target.print()
			: target.toString()
		: undefined;
const BaseBigNumDeserializationFn = (val: string) =>
	val
		? BigNum.from(
				typeof val === 'string' ? val.replace('.', '') : val,
				BASE_PRECISION_EXP
		  )
		: undefined;
const BaseBigNumSerializeAndDeserializeFns = {
	Serialize: BaseBigNumSerializationFn,
	Deserialize: BaseBigNumDeserializationFn,
};

const PriceBigNumSerializationFn = (target: BigNum | BN) =>
	target
		? target instanceof BigNum
			? target.print()
			: target.toString()
		: undefined;
const PriceBigNumDeserializationFn = (val: string) =>
	val
		? BigNum.from(
				typeof val === 'string' ? val.replace('.', '') : val,
				PRICE_PRECISION_EXP
		  )
		: undefined;
const PriceBigNumSerializeAndDeserializeFns = {
	Serialize: PriceBigNumSerializationFn,
	Deserialize: PriceBigNumDeserializationFn,
};

const PublicKeySerializationFn = (target: PublicKey) =>
	target ? target.toString() : undefined;
const PublicKeyDeserializationFn = (val: string) =>
	val ? new PublicKey(val) : undefined;
const PublicKeySerializeAndDeserializeFns = {
	Serialize: PublicKeySerializationFn,
	Deserialize: PublicKeyDeserializationFn,
};

const EnumSerializationFn = (target: Record<string, unknown>) => {
	if (!target) return null;

	return Object.keys(target)[0];
};
const EnumDeserializationFn = (val: any) => {
	if (!val) return null;

	{
		if (typeof val === 'string') return { [val]: {} };

		return val;
	}
};
const EnumSerializeAndDeserializeFns = {
	Serialize: EnumSerializationFn,
	Deserialize: EnumDeserializationFn,
};

const SERIALIZATION_UTILS = {
	BNSerializeAndDeserializeFns,
	BNArraySerializeAndDeserializeFns,
	QuoteBigNumSerializeAndDeserializeFns,
	PctBigNumSerializeAndDeserializeFns,
	BaseBigNumSerializeAndDeserializeFns,
	PriceBigNumSerializeAndDeserializeFns,
	PublicKeySerializeAndDeserializeFns,
	EnumSerializeAndDeserializeFns,
};

export class SerializableSummaryEvent implements CompetitionRoundSummaryRecord {
	@autoserializeUsing(SERIALIZATION_UTILS.PublicKeySerializeAndDeserializeFns)
	competition: PublicKey;
	@autoserializeUsing(SERIALIZATION_UTILS.BNSerializeAndDeserializeFns)
	roundNumber: BN;
	@autoserializeUsing(SERIALIZATION_UTILS.BNSerializeAndDeserializeFns)
	roundStartTs: BN;
	@autoserializeUsing(SERIALIZATION_UTILS.BNSerializeAndDeserializeFns)
	roundEndTs: BN;
	@autoserializeUsing(SERIALIZATION_UTILS.BNSerializeAndDeserializeFns)
	prizePlacement: BN;
	@autoserializeUsing(SERIALIZATION_UTILS.BNSerializeAndDeserializeFns)
	prizeOddsNumerator: BN;
	@autoserializeUsing(SERIALIZATION_UTILS.BNSerializeAndDeserializeFns)
	prizeRandomness: BN;
	@autoserializeUsing(SERIALIZATION_UTILS.BNSerializeAndDeserializeFns)
	prizeRandomnessMax: BN;
	@autoserializeUsing(SERIALIZATION_UTILS.BNSerializeAndDeserializeFns)
	maxPrizeBucketValue: BN;
	@autoserializeUsing(SERIALIZATION_UTILS.BNSerializeAndDeserializeFns)
	prizeAmount: BN;
	@autoserializeUsing(SERIALIZATION_UTILS.BNSerializeAndDeserializeFns)
	prizeValue: BN;
	@autoserializeUsing(SERIALIZATION_UTILS.BNSerializeAndDeserializeFns)
	prizeBase: BN;
	@autoserializeUsing(SERIALIZATION_UTILS.BNSerializeAndDeserializeFns)
	numberOfWinners: BN;
	@autoserializeUsing(SERIALIZATION_UTILS.BNSerializeAndDeserializeFns)
	numberOfCompetitorsSettled: BN;
	@autoserializeUsing(SERIALIZATION_UTILS.BNSerializeAndDeserializeFns)
	totalScoreSettled: BN;
	@autoserializeUsing(SERIALIZATION_UTILS.BNSerializeAndDeserializeFns)
	insuranceVaultBalance: BN;
	@autoserializeUsing(SERIALIZATION_UTILS.BNSerializeAndDeserializeFns)
	protocolIfShares: BN;
	@autoserializeUsing(SERIALIZATION_UTILS.BNSerializeAndDeserializeFns)
	totalIfShares: BN;
	@autoserializeUsing(SERIALIZATION_UTILS.BNSerializeAndDeserializeFns) ts: BN;
	@autoserializeAs(String) txSig: string;
	@autoserializeAs(Number) slot: number;
}

export class SerializableCompetitionRoundWinner
	implements CompetitionRoundWinnerRecord
{
	@autoserializeUsing(SERIALIZATION_UTILS.BNSerializeAndDeserializeFns)
	roundNumber: BN;
	@autoserializeUsing(SERIALIZATION_UTILS.PublicKeySerializeAndDeserializeFns)
	competitor: PublicKey;
	@autoserializeUsing(SERIALIZATION_UTILS.PublicKeySerializeAndDeserializeFns)
	competition: PublicKey;
	@autoserializeUsing(SERIALIZATION_UTILS.PublicKeySerializeAndDeserializeFns)
	competitorAuthority: PublicKey;
	@autoserializeUsing(SERIALIZATION_UTILS.BNSerializeAndDeserializeFns)
	minDraw: BN;
	@autoserializeUsing(SERIALIZATION_UTILS.BNSerializeAndDeserializeFns)
	maxDraw: BN;
	@autoserializeAs(Number) winnerPlacement: number;
	@autoserializeAs(Number) numberOfWinners: number;
	@autoserializeUsing(SERIALIZATION_UTILS.BNSerializeAndDeserializeFns)
	numberOfCompetitorsSettled: BN;
	@autoserializeUsing(SERIALIZATION_UTILS.BNSerializeAndDeserializeFns)
	winnerRandomness: BN;
	@autoserializeUsing(SERIALIZATION_UTILS.BNSerializeAndDeserializeFns)
	totalScoreSettled: BN;
	@autoserializeUsing(SERIALIZATION_UTILS.BNSerializeAndDeserializeFns)
	prizeRandomness: BN;
	@autoserializeUsing(SERIALIZATION_UTILS.BNSerializeAndDeserializeFns)
	prizeRandomnessMax: BN;
	@autoserializeUsing(SERIALIZATION_UTILS.BNSerializeAndDeserializeFns)
	prizeAmount: BN;
	@autoserializeUsing(SERIALIZATION_UTILS.BNSerializeAndDeserializeFns)
	prizeBase: BN;
	@autoserializeUsing(SERIALIZATION_UTILS.BNSerializeAndDeserializeFns)
	prizeValue: BN;
	@autoserializeUsing(SERIALIZATION_UTILS.BNSerializeAndDeserializeFns) ts: BN;
	@autoserializeAs(String) txSig: string;
	@autoserializeAs(Number) slot: number;
}

export class SerializableCompetitionResult implements CompetitionResult {
	@autoserializeAs(Number) startTs: number;
	@autoserializeAs(Number) endTs: number;
	@autoserializeAs(Number) roundNumber: number;
	@autoserializeUsing(SERIALIZATION_UTILS.BNSerializeAndDeserializeFns)
	competitors: BN;
	@autoserializeUsing(SERIALIZATION_UTILS.BNSerializeAndDeserializeFns)
	totalTickets: BN;
	// @ts-ignore
	@autoserializeAs(SerializableSummaryEvent)
	// @ts-ignore
	summaryEvent: SerializableSummaryEvent;
	//@ts-ignore
	@autoserializeAsArray(SerializableCompetitionRoundWinner)
	// @ts-ignore
	winners: SerializableCompetitionRoundWinner[];
}

class SerializableTopCompetitor {
	@autoserializeUsing(SERIALIZATION_UTILS.PublicKeySerializeAndDeserializeFns)
	authority: PublicKey;
	@autoserializeUsing(SERIALIZATION_UTILS.BNSerializeAndDeserializeFns)
	ticketCount: BN;
}

export class SerializableLiveCompetitionInfo implements LiveCompetitionInfo {
	@autoserializeAs(Number) lastFetchedTs: number;
	@autoserializeAs(Number) roundNumber: number;
	@autoserializeAs(Number) endTs: number;
	@autoserializeAs(Number) totalCompetitors: number;
	@autoserializeUsing(SERIALIZATION_UTILS.BNSerializeAndDeserializeFns)
	totalTickets: BN;
	@autoserializeAsArray(SerializableTopCompetitor) topCompetitors;
}

export const Serializer = {
	Serialize: {
		SerializableSummaryEvent: (cls: SerializableSummaryEvent) =>
			Serialize(cls, SerializableSummaryEvent),
		SerializableCompetitionRoundWinner: (
			cls: SerializableCompetitionRoundWinner
		) => Serialize(cls, SerializableCompetitionRoundWinner),
		SerializableCompetitionResult: (cls: SerializableCompetitionResult) =>
			Serialize(cls, SerializableCompetitionResult),
		SerializableLiveCompetitionInfo: (cls: SerializableLiveCompetitionInfo) =>
			Serialize(cls, SerializableLiveCompetitionInfo),
	},
	Deserialize: {
		SerializableSummaryEvent: (cls: Record<string, unknown>) =>
			// @ts-ignore
			Deserialize(
				cls as JsonObject,
				SerializableSummaryEvent
			) as SerializableSummaryEvent,
		SerializableCompetitionRoundWinner: (cls: Record<string, unknown>) =>
			// @ts-ignore
			Deserialize(
				cls as JsonObject,
				SerializableCompetitionRoundWinner
			) as SerializableCompetitionRoundWinner,
		SerializableCompetitionResult: (cls: Record<string, unknown>) =>
			// @ts-ignore
			Deserialize(
				cls as JsonObject,
				SerializableCompetitionResult
			) as SerializableCompetitionResult,
		SerializableLiveCompetitionInfo: (cls: Record<string, unknown>) =>
			// @ts-ignore
			Deserialize(
				cls as JsonObject,
				SerializableLiveCompetitionInfo
			) as SerializableLiveCompetitionInfo,
	},
	setDeserializeFromSnakeCase: () => {
		SetDeserializeKeyTransform(SnakeCase);
	},
	setSerializeFromSnakeCase: () => {
		SetSerializeKeyTransform(SnakeCase);
	},
};
