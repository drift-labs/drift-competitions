import { PublicKey } from '@solana/web3.js';
import * as anchor from '@coral-xyz/anchor';

export function getCompetitionAddressSync(
	programId: PublicKey,
	encodedName: number[]
): PublicKey {
	return PublicKey.findProgramAddressSync(
		[
			Buffer.from(anchor.utils.bytes.utf8.encode('competition')),
			Buffer.from(encodedName),
		],
		programId
	)[0];
}