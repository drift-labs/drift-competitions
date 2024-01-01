import { BN } from "@coral-xyz/anchor";
import { getSpotMarketPublicKey, QUOTE_SPOT_MARKET_INDEX, getInsuranceFundVaultPublicKey } from "@drift-labs/sdk";
import { PublicKey, TransactionSignature } from "@solana/web3.js";
import { SwitchboardProgram, MAINNET_GENESIS_HASH, DEVNET_GENESIS_HASH, FunctionRequestAccount } from "@switchboard-xyz/solana.js";
import { CompetitionsClient } from "./competitionClient";
import { getCompetitionAuthorityAddressSync } from "./addresses";
import * as anchor from '@coral-xyz/anchor';

export class SwitchboardClient {
  constructor(readonly competitionsClient: CompetitionsClient, readonly provider: anchor.Provider) {}

	public async requestRandomness(
		competition: PublicKey,
		bounty?: BN
	): Promise<TransactionSignature> {
		const switchboardProgram = await SwitchboardProgram.fromProvider(
			// @ts-ignore
			this.provider
		);

		const genesisHash =
			await switchboardProgram.provider.connection.getGenesisHash();
		const attestationQueueAddress =
			genesisHash === MAINNET_GENESIS_HASH
				? new PublicKey('2ie3JZfKcvsRLsJaP5fSo43gUo1vsurnUAtAgUdUAiDG')
				: genesisHash === DEVNET_GENESIS_HASH
				? new PublicKey('CkvizjVnm2zA5Wuwan34NhVT3zFc7vqUyGnA6tuEF5aE')
				: undefined;

		const competitionAccount = await this.competitionsClient.program.account.competition.fetch(
			competition
		);

		const spotMarket = await getSpotMarketPublicKey(
			this.competitionsClient.driftClient.program.programId,
			QUOTE_SPOT_MARKET_INDEX
		);
		const insuranceFundVault = await getInsuranceFundVaultPublicKey(
			this.competitionsClient.driftClient.program.programId,
			QUOTE_SPOT_MARKET_INDEX
		);

		return await this.competitionsClient.program.methods
			.requestRandomness(bounty ?? null)
			.accounts({
				competition,
				switchboard: switchboardProgram.attestationProgramId,
				switchboardState: switchboardProgram.attestationProgramState.publicKey,
				switchboardAttestationQueue: attestationQueueAddress,
				switchboardFunction: competitionAccount.switchboardFunction,
				competitionAuthority: competitionAccount.competitionAuthority,
				switchboardRequest: competitionAccount.switchboardFunctionRequest,
				switchboardRequestEscrow:
					competitionAccount.switchboardFunctionRequestEscrow,
				insuranceFundVault,
				spotMarket,
			})
			.rpc();
	}

  public async updateSwitchboardFunction(
		competition: PublicKey,
		switchboardFunction: PublicKey
	): Promise<TransactionSignature> {
		const switchboardProgram = await SwitchboardProgram.fromProvider(
			// @ts-ignore
			this.program.provider
		);

		const genesisHash =
			await switchboardProgram.provider.connection.getGenesisHash();
		const attestationQueueAddress =
			genesisHash === MAINNET_GENESIS_HASH
				? new PublicKey('2ie3JZfKcvsRLsJaP5fSo43gUo1vsurnUAtAgUdUAiDG')
				: genesisHash === DEVNET_GENESIS_HASH
				? new PublicKey('CkvizjVnm2zA5Wuwan34NhVT3zFc7vqUyGnA6tuEF5aE')
				: undefined;

		const switchboardRequestKeypair = anchor.web3.Keypair.generate();
		const switchboardRequest = new FunctionRequestAccount(
			switchboardProgram,
			switchboardRequestKeypair.publicKey
		);

		const switchboardRequestEscrowPubkey =
			await anchor.utils.token.associatedAddress({
				mint: switchboardProgram.mint.address,
				owner: switchboardRequestKeypair.publicKey,
			});

		const competitionAuthority = getCompetitionAuthorityAddressSync(
			this.competitionsClient.program.programId,
			competition
		);

		return await this.competitionsClient.program.methods
			.updateSwitchboardFunction()
			.accounts({
				sponsor: this.competitionsClient.program.provider.publicKey,
				competition,
				competitionAuthority,
				switchboard: switchboardProgram.attestationProgramId,
				switchboardState: switchboardProgram.attestationProgramState.publicKey,
				switchboardAttestationQueue: attestationQueueAddress,
				switchboardFunction: switchboardFunction,
				switchboardRequest: switchboardRequest.publicKey,
				switchboardRequestEscrow: switchboardRequestEscrowPubkey,
				switchboardMint: switchboardProgram.mint.address,
			})
			.signers([switchboardRequestKeypair])
			.rpc();
	}
}