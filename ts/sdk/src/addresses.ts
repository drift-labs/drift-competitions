import { PublicKey } from '@solana/web3.js';
import * as anchor from '@coral-xyz/anchor';

export function getCompetitionAddressSync(
	programId: PublicKey,
): PublicKey {
	return PublicKey.findProgramAddressSync(
		[
			Buffer.from(anchor.utils.bytes.utf8.encode('competition')),
		],
		programId
	)[0];
}

export function getCompetitorAddressSync(
	programId: PublicKey,
	competitionPublicKey: PublicKey,
	authority: PublicKey
): PublicKey {
	return PublicKey.findProgramAddressSync(
		[
			Buffer.from(anchor.utils.bytes.utf8.encode('competitor')),
			Buffer.from(competitionPublicKey.toBytes()),
			Buffer.from(authority.toBytes()),
		],
		programId
	)[0];
}

export function getCompetitionAuthorityAddressSync(
	programId: PublicKey,
	competition: PublicKey
): PublicKey {
	return PublicKey.findProgramAddressSync(
		[
			Buffer.from(anchor.utils.bytes.utf8.encode('competition_authority')),
			competition.toBuffer(),
		],
		programId
	)[0];
}
