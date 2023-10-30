import {
	EventSubscriptionOrderBy,
	EventSubscriptionOrderDirection,
} from '@drift-labs/sdk';
import { CompetitionsEventMap, EventType, SortFn } from '..//types/types';

function clientSortAscFn(): 'less than' {
	return 'less than';
}

function clientSortDescFn(): 'greater than' {
	return 'greater than';
}

function defaultBlockchainSortFn(
	currentEvent: CompetitionsEventMap[EventType],
	newEvent: CompetitionsEventMap[EventType]
): 'less than' | 'greater than' {
	return currentEvent.slot <= newEvent.slot ? 'less than' : 'greater than';
}

export function getSortFn(
	orderBy: EventSubscriptionOrderBy,
	orderDir: EventSubscriptionOrderDirection
): SortFn {
	if (orderBy === 'client') {
		return orderDir === 'asc' ? clientSortAscFn : clientSortDescFn;
	}

	return defaultBlockchainSortFn;
}
