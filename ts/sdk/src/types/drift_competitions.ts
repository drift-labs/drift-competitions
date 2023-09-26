export type DriftCompetitions = {
	version: '0.1.0';
	name: 'drift_competitions';
	instructions: [
		{
			name: 'initializeCompetition';
			accounts: [
				{
					name: 'competition';
					isMut: true;
					isSigner: false;
				},
				{
					name: 'sponsor';
					isMut: false;
					isSigner: true;
				},
				{
					name: 'payer';
					isMut: true;
					isSigner: true;
				},
				{
					name: 'rent';
					isMut: false;
					isSigner: false;
				},
				{
					name: 'systemProgram';
					isMut: false;
					isSigner: false;
				}
			];
			args: [
				{
					name: 'params';
					type: {
						defined: 'CompetitionParams';
					};
				}
			];
		},
		{
			name: 'updateCompetition';
			accounts: [
				{
					name: 'competition';
					isMut: true;
					isSigner: false;
				},
				{
					name: 'sponsor';
					isMut: false;
					isSigner: true;
				}
			];
			args: [
				{
					name: 'params';
					type: {
						defined: 'UpdateCompetitionParams';
					};
				}
			];
		},
		{
			name: 'updateSwitchboardFunction';
			accounts: [
				{
					name: 'sponsor';
					isMut: true;
					isSigner: true;
				},
				{
					name: 'competition';
					isMut: true;
					isSigner: false;
				},
				{
					name: 'competitionAuthority';
					isMut: false;
					isSigner: false;
					docs: ['CHECK'];
				},
				{
					name: 'switchboard';
					isMut: false;
					isSigner: false;
				},
				{
					name: 'switchboardState';
					isMut: false;
					isSigner: false;
				},
				{
					name: 'switchboardAttestationQueue';
					isMut: false;
					isSigner: false;
				},
				{
					name: 'switchboardFunction';
					isMut: true;
					isSigner: false;
				},
				{
					name: 'switchboardRequest';
					isMut: true;
					isSigner: true;
				},
				{
					name: 'switchboardRequestEscrow';
					isMut: true;
					isSigner: false;
				},
				{
					name: 'switchboardMint';
					isMut: false;
					isSigner: false;
				},
				{
					name: 'tokenProgram';
					isMut: false;
					isSigner: false;
				},
				{
					name: 'associatedTokenProgram';
					isMut: false;
					isSigner: false;
				},
				{
					name: 'systemProgram';
					isMut: false;
					isSigner: false;
				}
			];
			args: [];
		},
		{
			name: 'initializeCompetitor';
			accounts: [
				{
					name: 'competitor';
					isMut: true;
					isSigner: false;
				},
				{
					name: 'competition';
					isMut: true;
					isSigner: false;
				},
				{
					name: 'driftUserStats';
					isMut: false;
					isSigner: false;
				},
				{
					name: 'authority';
					isMut: false;
					isSigner: true;
				},
				{
					name: 'payer';
					isMut: true;
					isSigner: true;
				},
				{
					name: 'rent';
					isMut: false;
					isSigner: false;
				},
				{
					name: 'systemProgram';
					isMut: false;
					isSigner: false;
				}
			];
			args: [];
		},
		{
			name: 'claimEntry';
			accounts: [
				{
					name: 'authority';
					isMut: true;
					isSigner: true;
				},
				{
					name: 'competitor';
					isMut: true;
					isSigner: false;
				},
				{
					name: 'competition';
					isMut: true;
					isSigner: false;
				},
				{
					name: 'driftUserStats';
					isMut: false;
					isSigner: false;
				},
				{
					name: 'instructions';
					isMut: false;
					isSigner: false;
				}
			];
			args: [];
		},
		{
			name: 'claimWinnings';
			accounts: [
				{
					name: 'authority';
					isMut: true;
					isSigner: true;
				},
				{
					name: 'competitor';
					isMut: true;
					isSigner: false;
				},
				{
					name: 'competition';
					isMut: true;
					isSigner: false;
				},
				{
					name: 'driftUserStats';
					isMut: false;
					isSigner: false;
				},
				{
					name: 'spotMarket';
					isMut: true;
					isSigner: false;
				},
				{
					name: 'insuranceFundVault';
					isMut: false;
					isSigner: false;
				},
				{
					name: 'insuranceFundStake';
					isMut: true;
					isSigner: false;
				}
			];
			args: [
				{
					name: 'nShares';
					type: {
						option: 'u64';
					};
				}
			];
		},
		{
			name: 'settleCompetitor';
			accounts: [
				{
					name: 'keeper';
					isMut: true;
					isSigner: true;
				},
				{
					name: 'competitor';
					isMut: true;
					isSigner: false;
				},
				{
					name: 'competition';
					isMut: true;
					isSigner: false;
				},
				{
					name: 'driftUserStats';
					isMut: false;
					isSigner: false;
				}
			];
			args: [];
		},
		{
			name: 'settleCompetition';
			accounts: [
				{
					name: 'keeper';
					isMut: true;
					isSigner: true;
				},
				{
					name: 'competition';
					isMut: true;
					isSigner: false;
				}
			];
			args: [];
		},
		{
			name: 'requestRandomness';
			accounts: [
				{
					name: 'competition';
					isMut: true;
					isSigner: false;
				},
				{
					name: 'keeper';
					isMut: true;
					isSigner: true;
				},
				{
					name: 'competitionAuthority';
					isMut: false;
					isSigner: false;
					docs: ['CHECK'];
				},
				{
					name: 'spotMarket';
					isMut: false;
					isSigner: false;
				},
				{
					name: 'insuranceFundVault';
					isMut: false;
					isSigner: false;
				},
				{
					name: 'switchboard';
					isMut: false;
					isSigner: false;
				},
				{
					name: 'switchboardState';
					isMut: false;
					isSigner: false;
				},
				{
					name: 'switchboardAttestationQueue';
					isMut: false;
					isSigner: false;
				},
				{
					name: 'switchboardFunction';
					isMut: true;
					isSigner: false;
				},
				{
					name: 'switchboardRequest';
					isMut: true;
					isSigner: false;
				},
				{
					name: 'switchboardRequestEscrow';
					isMut: true;
					isSigner: false;
				},
				{
					name: 'tokenProgram';
					isMut: false;
					isSigner: false;
				},
				{
					name: 'systemProgram';
					isMut: false;
					isSigner: false;
				}
			];
			args: [];
		},
		{
			name: 'receiveRandomness';
			accounts: [
				{
					name: 'competition';
					isMut: true;
					isSigner: false;
				},
				{
					name: 'spotMarket';
					isMut: false;
					isSigner: false;
				},
				{
					name: 'insuranceFundVault';
					isMut: false;
					isSigner: false;
				},
				{
					name: 'switchboardFunction';
					isMut: false;
					isSigner: false;
				},
				{
					name: 'switchboardRequest';
					isMut: false;
					isSigner: false;
				},
				{
					name: 'enclaveSigner';
					isMut: false;
					isSigner: true;
				}
			];
			args: [
				{
					name: 'winnerRandomness';
					type: 'u128';
				},
				{
					name: 'prizeRandomness';
					type: 'u128';
				}
			];
		},
		{
			name: 'settleWinner';
			accounts: [
				{
					name: 'keeper';
					isMut: true;
					isSigner: true;
				},
				{
					name: 'competitor';
					isMut: true;
					isSigner: false;
				},
				{
					name: 'competition';
					isMut: true;
					isSigner: false;
				},
				{
					name: 'driftUserStats';
					isMut: false;
					isSigner: false;
				},
				{
					name: 'spotMarket';
					isMut: true;
					isSigner: false;
				}
			];
			args: [];
		}
	];
	accounts: [
		{
			name: 'competition';
			type: {
				kind: 'struct';
				fields: [
					{
						name: 'name';
						type: {
							array: ['u8', 32];
						};
					},
					{
						name: 'sponsorInfo';
						type: {
							defined: 'SponsorInfo';
						};
					},
					{
						name: 'switchboardFunction';
						type: 'publicKey';
					},
					{
						name: 'switchboardFunctionRequest';
						type: 'publicKey';
					},
					{
						name: 'switchboardFunctionRequestEscrow';
						type: 'publicKey';
					},
					{
						name: 'competitionAuthority';
						type: 'publicKey';
					},
					{
						name: 'numberOfCompetitors';
						type: 'u128';
					},
					{
						name: 'numberOfCompetitorsSettled';
						type: 'u128';
					},
					{
						name: 'totalScoreSettled';
						type: 'u128';
					},
					{
						name: 'maxEntriesPerCompetitor';
						type: 'u128';
					},
					{
						name: 'prizeAmount';
						type: 'u128';
					},
					{
						name: 'prizeBase';
						type: 'u128';
					},
					{
						name: 'winnerRandomness';
						type: 'u128';
					},
					{
						name: 'prizeRandomness';
						type: 'u128';
					},
					{
						name: 'prizeRandomnessMax';
						type: 'u128';
					},
					{
						name: 'roundNumber';
						type: 'u64';
					},
					{
						name: 'nextRoundExpiryTs';
						type: 'i64';
					},
					{
						name: 'competitionExpiryTs';
						type: 'i64';
					},
					{
						name: 'roundDuration';
						type: 'u64';
					},
					{
						name: 'status';
						type: {
							defined: 'CompetitionRoundStatus';
						};
					},
					{
						name: 'competitionAuthorityBump';
						type: 'u8';
					},
					{
						name: 'padding';
						type: {
							array: ['u8', 6];
						};
					}
				];
			};
		},
		{
			name: 'competitor';
			type: {
				kind: 'struct';
				fields: [
					{
						name: 'authority';
						type: 'publicKey';
					},
					{
						name: 'competition';
						type: 'publicKey';
					},
					{
						name: 'userStats';
						type: 'publicKey';
					},
					{
						name: 'minDraw';
						type: 'u128';
					},
					{
						name: 'maxDraw';
						type: 'u128';
					},
					{
						name: 'unclaimedWinningsBase';
						type: 'u128';
					},
					{
						name: 'unclaimedWinnings';
						type: 'u64';
					},
					{
						name: 'competitionRoundNumber';
						type: 'u64';
					},
					{
						name: 'previousSnapshotScore';
						type: 'u64';
					},
					{
						name: 'latestSnapshotScore';
						type: 'u64';
					},
					{
						name: 'bonusScore';
						type: 'u64';
					}
				];
			};
		}
	];
	types: [
		{
			name: 'CompetitionParams';
			type: {
				kind: 'struct';
				fields: [
					{
						name: 'name';
						type: {
							array: ['u8', 32];
						};
					},
					{
						name: 'nextRoundExpiryTs';
						type: 'i64';
					},
					{
						name: 'competitionExpiryTs';
						type: 'i64';
					},
					{
						name: 'roundDuration';
						type: 'u64';
					}
				];
			};
		},
		{
			name: 'UpdateCompetitionParams';
			type: {
				kind: 'struct';
				fields: [
					{
						name: 'nextRoundExpiryTs';
						type: {
							option: 'i64';
						};
					},
					{
						name: 'competitionExpiryTs';
						type: {
							option: 'i64';
						};
					},
					{
						name: 'roundDuration';
						type: {
							option: 'u64';
						};
					}
				];
			};
		},
		{
			name: 'SponsorInfo';
			type: {
				kind: 'struct';
				fields: [
					{
						name: 'sponsor';
						type: 'publicKey';
					},
					{
						name: 'minSponsorAmount';
						type: 'u64';
					},
					{
						name: 'maxSponsorFraction';
						type: 'u64';
					}
				];
			};
		},
		{
			name: 'CompetitionRoundStatus';
			type: {
				kind: 'enum';
				variants: [
					{
						name: 'Active';
					},
					{
						name: 'WinnerAndPrizeRandomnessRequested';
					},
					{
						name: 'WinnerAndPrizeRandomnessComplete';
					},
					{
						name: 'WinnerSettlementComplete';
					},
					{
						name: 'Expired';
					}
				];
			};
		}
	];
	events: [
		{
			name: 'CompetitionRoundWinnerRecord';
			fields: [
				{
					name: 'roundNumber';
					type: 'u64';
					index: false;
				},
				{
					name: 'competitor';
					type: 'publicKey';
					index: false;
				},
				{
					name: 'minDraw';
					type: 'u128';
					index: false;
				},
				{
					name: 'maxDraw';
					type: 'u128';
					index: false;
				},
				{
					name: 'numberOfCompetitorsSettled';
					type: 'u128';
					index: false;
				},
				{
					name: 'winnerRandomness';
					type: 'u128';
					index: false;
				},
				{
					name: 'totalScoreSettled';
					type: 'u128';
					index: false;
				},
				{
					name: 'prizeRandomness';
					type: 'u128';
					index: false;
				},
				{
					name: 'prizeRandomnessMax';
					type: 'u128';
					index: false;
				},
				{
					name: 'prizeAmount';
					type: 'u128';
					index: false;
				},
				{
					name: 'prizeBase';
					type: 'u128';
					index: false;
				},
				{
					name: 'ts';
					type: 'i64';
					index: false;
				}
			];
		}
	];
	errors: [
		{
			code: 6000;
			name: 'Default';
			msg: 'Default';
		},
		{
			code: 6001;
			name: 'DriftError';
			msg: 'DriftError';
		},
		{
			code: 6002;
			name: 'CompetitionRoundOngoing';
			msg: 'CompetitionRoundOngoing';
		},
		{
			code: 6003;
			name: 'CompetitionRoundInSettlementPhase';
			msg: 'CompetitionRoundInSettlementPhase';
		},
		{
			code: 6004;
			name: 'CompetitionStatusNotActive';
			msg: 'CompetitionStatusNotActive';
		},
		{
			code: 6005;
			name: 'CompetitionExpired';
			msg: 'CompetitionExpired';
		},
		{
			code: 6006;
			name: 'InvalidRoundSettlementDetected';
			msg: 'InvalidRoundSettlementDetected';
		},
		{
			code: 6007;
			name: 'CompetitionWinnerNotDetermined';
			msg: 'CompetitionWinnerNotDetermined';
		},
		{
			code: 6008;
			name: 'CompetitorHasWrongRoundNumber';
			msg: 'CompetitorHasWrongRoundNumber';
		},
		{
			code: 6009;
			name: 'CompetitorNotWinner';
			msg: 'CompetitorNotWinner';
		},
		{
			code: 6010;
			name: 'InvalidStatusUpdateDetected';
			msg: 'InvalidStatusUpdateDetected';
		},
		{
			code: 6011;
			name: 'InvalidIFRebase';
			msg: 'InvalidIFRebase';
		},
		{
			code: 6012;
			name: 'CompetitorHasAlreadyClaimedEntry';
			msg: 'CompetitorHasAlreadyClaimedEntry';
		},
		{
			code: 6013;
			name: 'CompetitorNeedsToRebaseInsuranceFundStake';
			msg: 'CompetitorNeedsToRebaseInsuranceFundStake';
		},
		{
			code: 6014;
			name: 'CompetitorHasNoUnclaimedWinnings';
			msg: 'CompetitorHasNoUnclaimedWinnings';
		},
		{
			code: 6015;
			name: 'CompetitionRoundNumberIssue';
			msg: 'CompetitionRoundNumberIssue';
		},
		{
			code: 6016;
			name: 'CompetitorSnapshotIssue';
			msg: 'CompetitorSnapshotIssue';
		}
	];
};

export const IDL: DriftCompetitions = {
	version: '0.1.0',
	name: 'drift_competitions',
	instructions: [
		{
			name: 'initializeCompetition',
			accounts: [
				{
					name: 'competition',
					isMut: true,
					isSigner: false,
				},
				{
					name: 'sponsor',
					isMut: false,
					isSigner: true,
				},
				{
					name: 'payer',
					isMut: true,
					isSigner: true,
				},
				{
					name: 'rent',
					isMut: false,
					isSigner: false,
				},
				{
					name: 'systemProgram',
					isMut: false,
					isSigner: false,
				},
			],
			args: [
				{
					name: 'params',
					type: {
						defined: 'CompetitionParams',
					},
				},
			],
		},
		{
			name: 'updateCompetition',
			accounts: [
				{
					name: 'competition',
					isMut: true,
					isSigner: false,
				},
				{
					name: 'sponsor',
					isMut: false,
					isSigner: true,
				},
			],
			args: [
				{
					name: 'params',
					type: {
						defined: 'UpdateCompetitionParams',
					},
				},
			],
		},
		{
			name: 'updateSwitchboardFunction',
			accounts: [
				{
					name: 'sponsor',
					isMut: true,
					isSigner: true,
				},
				{
					name: 'competition',
					isMut: true,
					isSigner: false,
				},
				{
					name: 'competitionAuthority',
					isMut: false,
					isSigner: false,
					docs: ['CHECK'],
				},
				{
					name: 'switchboard',
					isMut: false,
					isSigner: false,
				},
				{
					name: 'switchboardState',
					isMut: false,
					isSigner: false,
				},
				{
					name: 'switchboardAttestationQueue',
					isMut: false,
					isSigner: false,
				},
				{
					name: 'switchboardFunction',
					isMut: true,
					isSigner: false,
				},
				{
					name: 'switchboardRequest',
					isMut: true,
					isSigner: true,
				},
				{
					name: 'switchboardRequestEscrow',
					isMut: true,
					isSigner: false,
				},
				{
					name: 'switchboardMint',
					isMut: false,
					isSigner: false,
				},
				{
					name: 'tokenProgram',
					isMut: false,
					isSigner: false,
				},
				{
					name: 'associatedTokenProgram',
					isMut: false,
					isSigner: false,
				},
				{
					name: 'systemProgram',
					isMut: false,
					isSigner: false,
				},
			],
			args: [],
		},
		{
			name: 'initializeCompetitor',
			accounts: [
				{
					name: 'competitor',
					isMut: true,
					isSigner: false,
				},
				{
					name: 'competition',
					isMut: true,
					isSigner: false,
				},
				{
					name: 'driftUserStats',
					isMut: false,
					isSigner: false,
				},
				{
					name: 'authority',
					isMut: false,
					isSigner: true,
				},
				{
					name: 'payer',
					isMut: true,
					isSigner: true,
				},
				{
					name: 'rent',
					isMut: false,
					isSigner: false,
				},
				{
					name: 'systemProgram',
					isMut: false,
					isSigner: false,
				},
			],
			args: [],
		},
		{
			name: 'claimEntry',
			accounts: [
				{
					name: 'authority',
					isMut: true,
					isSigner: true,
				},
				{
					name: 'competitor',
					isMut: true,
					isSigner: false,
				},
				{
					name: 'competition',
					isMut: true,
					isSigner: false,
				},
				{
					name: 'driftUserStats',
					isMut: false,
					isSigner: false,
				},
				{
					name: 'instructions',
					isMut: false,
					isSigner: false,
				},
			],
			args: [],
		},
		{
			name: 'claimWinnings',
			accounts: [
				{
					name: 'authority',
					isMut: true,
					isSigner: true,
				},
				{
					name: 'competitor',
					isMut: true,
					isSigner: false,
				},
				{
					name: 'competition',
					isMut: true,
					isSigner: false,
				},
				{
					name: 'driftUserStats',
					isMut: false,
					isSigner: false,
				},
				{
					name: 'spotMarket',
					isMut: true,
					isSigner: false,
				},
				{
					name: 'insuranceFundVault',
					isMut: false,
					isSigner: false,
				},
				{
					name: 'insuranceFundStake',
					isMut: true,
					isSigner: false,
				},
			],
			args: [
				{
					name: 'nShares',
					type: {
						option: 'u64',
					},
				},
			],
		},
		{
			name: 'settleCompetitor',
			accounts: [
				{
					name: 'keeper',
					isMut: true,
					isSigner: true,
				},
				{
					name: 'competitor',
					isMut: true,
					isSigner: false,
				},
				{
					name: 'competition',
					isMut: true,
					isSigner: false,
				},
				{
					name: 'driftUserStats',
					isMut: false,
					isSigner: false,
				},
			],
			args: [],
		},
		{
			name: 'settleCompetition',
			accounts: [
				{
					name: 'keeper',
					isMut: true,
					isSigner: true,
				},
				{
					name: 'competition',
					isMut: true,
					isSigner: false,
				},
			],
			args: [],
		},
		{
			name: 'requestRandomness',
			accounts: [
				{
					name: 'competition',
					isMut: true,
					isSigner: false,
				},
				{
					name: 'keeper',
					isMut: true,
					isSigner: true,
				},
				{
					name: 'competitionAuthority',
					isMut: false,
					isSigner: false,
					docs: ['CHECK'],
				},
				{
					name: 'spotMarket',
					isMut: false,
					isSigner: false,
				},
				{
					name: 'insuranceFundVault',
					isMut: false,
					isSigner: false,
				},
				{
					name: 'switchboard',
					isMut: false,
					isSigner: false,
				},
				{
					name: 'switchboardState',
					isMut: false,
					isSigner: false,
				},
				{
					name: 'switchboardAttestationQueue',
					isMut: false,
					isSigner: false,
				},
				{
					name: 'switchboardFunction',
					isMut: true,
					isSigner: false,
				},
				{
					name: 'switchboardRequest',
					isMut: true,
					isSigner: false,
				},
				{
					name: 'switchboardRequestEscrow',
					isMut: true,
					isSigner: false,
				},
				{
					name: 'tokenProgram',
					isMut: false,
					isSigner: false,
				},
				{
					name: 'systemProgram',
					isMut: false,
					isSigner: false,
				},
			],
			args: [],
		},
		{
			name: 'receiveRandomness',
			accounts: [
				{
					name: 'competition',
					isMut: true,
					isSigner: false,
				},
				{
					name: 'spotMarket',
					isMut: false,
					isSigner: false,
				},
				{
					name: 'insuranceFundVault',
					isMut: false,
					isSigner: false,
				},
				{
					name: 'switchboardFunction',
					isMut: false,
					isSigner: false,
				},
				{
					name: 'switchboardRequest',
					isMut: false,
					isSigner: false,
				},
				{
					name: 'enclaveSigner',
					isMut: false,
					isSigner: true,
				},
			],
			args: [
				{
					name: 'winnerRandomness',
					type: 'u128',
				},
				{
					name: 'prizeRandomness',
					type: 'u128',
				},
			],
		},
		{
			name: 'settleWinner',
			accounts: [
				{
					name: 'keeper',
					isMut: true,
					isSigner: true,
				},
				{
					name: 'competitor',
					isMut: true,
					isSigner: false,
				},
				{
					name: 'competition',
					isMut: true,
					isSigner: false,
				},
				{
					name: 'driftUserStats',
					isMut: false,
					isSigner: false,
				},
				{
					name: 'spotMarket',
					isMut: true,
					isSigner: false,
				},
			],
			args: [],
		},
	],
	accounts: [
		{
			name: 'competition',
			type: {
				kind: 'struct',
				fields: [
					{
						name: 'name',
						type: {
							array: ['u8', 32],
						},
					},
					{
						name: 'sponsorInfo',
						type: {
							defined: 'SponsorInfo',
						},
					},
					{
						name: 'switchboardFunction',
						type: 'publicKey',
					},
					{
						name: 'switchboardFunctionRequest',
						type: 'publicKey',
					},
					{
						name: 'switchboardFunctionRequestEscrow',
						type: 'publicKey',
					},
					{
						name: 'competitionAuthority',
						type: 'publicKey',
					},
					{
						name: 'numberOfCompetitors',
						type: 'u128',
					},
					{
						name: 'numberOfCompetitorsSettled',
						type: 'u128',
					},
					{
						name: 'totalScoreSettled',
						type: 'u128',
					},
					{
						name: 'maxEntriesPerCompetitor',
						type: 'u128',
					},
					{
						name: 'prizeAmount',
						type: 'u128',
					},
					{
						name: 'prizeBase',
						type: 'u128',
					},
					{
						name: 'winnerRandomness',
						type: 'u128',
					},
					{
						name: 'prizeRandomness',
						type: 'u128',
					},
					{
						name: 'prizeRandomnessMax',
						type: 'u128',
					},
					{
						name: 'roundNumber',
						type: 'u64',
					},
					{
						name: 'nextRoundExpiryTs',
						type: 'i64',
					},
					{
						name: 'competitionExpiryTs',
						type: 'i64',
					},
					{
						name: 'roundDuration',
						type: 'u64',
					},
					{
						name: 'status',
						type: {
							defined: 'CompetitionRoundStatus',
						},
					},
					{
						name: 'competitionAuthorityBump',
						type: 'u8',
					},
					{
						name: 'padding',
						type: {
							array: ['u8', 6],
						},
					},
				],
			},
		},
		{
			name: 'competitor',
			type: {
				kind: 'struct',
				fields: [
					{
						name: 'authority',
						type: 'publicKey',
					},
					{
						name: 'competition',
						type: 'publicKey',
					},
					{
						name: 'userStats',
						type: 'publicKey',
					},
					{
						name: 'minDraw',
						type: 'u128',
					},
					{
						name: 'maxDraw',
						type: 'u128',
					},
					{
						name: 'unclaimedWinningsBase',
						type: 'u128',
					},
					{
						name: 'unclaimedWinnings',
						type: 'u64',
					},
					{
						name: 'competitionRoundNumber',
						type: 'u64',
					},
					{
						name: 'previousSnapshotScore',
						type: 'u64',
					},
					{
						name: 'latestSnapshotScore',
						type: 'u64',
					},
					{
						name: 'bonusScore',
						type: 'u64',
					},
				],
			},
		},
	],
	types: [
		{
			name: 'CompetitionParams',
			type: {
				kind: 'struct',
				fields: [
					{
						name: 'name',
						type: {
							array: ['u8', 32],
						},
					},
					{
						name: 'nextRoundExpiryTs',
						type: 'i64',
					},
					{
						name: 'competitionExpiryTs',
						type: 'i64',
					},
					{
						name: 'roundDuration',
						type: 'u64',
					},
				],
			},
		},
		{
			name: 'UpdateCompetitionParams',
			type: {
				kind: 'struct',
				fields: [
					{
						name: 'nextRoundExpiryTs',
						type: {
							option: 'i64',
						},
					},
					{
						name: 'competitionExpiryTs',
						type: {
							option: 'i64',
						},
					},
					{
						name: 'roundDuration',
						type: {
							option: 'u64',
						},
					},
				],
			},
		},
		{
			name: 'SponsorInfo',
			type: {
				kind: 'struct',
				fields: [
					{
						name: 'sponsor',
						type: 'publicKey',
					},
					{
						name: 'minSponsorAmount',
						type: 'u64',
					},
					{
						name: 'maxSponsorFraction',
						type: 'u64',
					},
				],
			},
		},
		{
			name: 'CompetitionRoundStatus',
			type: {
				kind: 'enum',
				variants: [
					{
						name: 'Active',
					},
					{
						name: 'WinnerAndPrizeRandomnessRequested',
					},
					{
						name: 'WinnerAndPrizeRandomnessComplete',
					},
					{
						name: 'WinnerSettlementComplete',
					},
					{
						name: 'Expired',
					},
				],
			},
		},
	],
	events: [
		{
			name: 'CompetitionRoundWinnerRecord',
			fields: [
				{
					name: 'roundNumber',
					type: 'u64',
					index: false,
				},
				{
					name: 'competitor',
					type: 'publicKey',
					index: false,
				},
				{
					name: 'minDraw',
					type: 'u128',
					index: false,
				},
				{
					name: 'maxDraw',
					type: 'u128',
					index: false,
				},
				{
					name: 'numberOfCompetitorsSettled',
					type: 'u128',
					index: false,
				},
				{
					name: 'winnerRandomness',
					type: 'u128',
					index: false,
				},
				{
					name: 'totalScoreSettled',
					type: 'u128',
					index: false,
				},
				{
					name: 'prizeRandomness',
					type: 'u128',
					index: false,
				},
				{
					name: 'prizeRandomnessMax',
					type: 'u128',
					index: false,
				},
				{
					name: 'prizeAmount',
					type: 'u128',
					index: false,
				},
				{
					name: 'prizeBase',
					type: 'u128',
					index: false,
				},
				{
					name: 'ts',
					type: 'i64',
					index: false,
				},
			],
		},
	],
	errors: [
		{
			code: 6000,
			name: 'Default',
			msg: 'Default',
		},
		{
			code: 6001,
			name: 'DriftError',
			msg: 'DriftError',
		},
		{
			code: 6002,
			name: 'CompetitionRoundOngoing',
			msg: 'CompetitionRoundOngoing',
		},
		{
			code: 6003,
			name: 'CompetitionRoundInSettlementPhase',
			msg: 'CompetitionRoundInSettlementPhase',
		},
		{
			code: 6004,
			name: 'CompetitionStatusNotActive',
			msg: 'CompetitionStatusNotActive',
		},
		{
			code: 6005,
			name: 'CompetitionExpired',
			msg: 'CompetitionExpired',
		},
		{
			code: 6006,
			name: 'InvalidRoundSettlementDetected',
			msg: 'InvalidRoundSettlementDetected',
		},
		{
			code: 6007,
			name: 'CompetitionWinnerNotDetermined',
			msg: 'CompetitionWinnerNotDetermined',
		},
		{
			code: 6008,
			name: 'CompetitorHasWrongRoundNumber',
			msg: 'CompetitorHasWrongRoundNumber',
		},
		{
			code: 6009,
			name: 'CompetitorNotWinner',
			msg: 'CompetitorNotWinner',
		},
		{
			code: 6010,
			name: 'InvalidStatusUpdateDetected',
			msg: 'InvalidStatusUpdateDetected',
		},
		{
			code: 6011,
			name: 'InvalidIFRebase',
			msg: 'InvalidIFRebase',
		},
		{
			code: 6012,
			name: 'CompetitorHasAlreadyClaimedEntry',
			msg: 'CompetitorHasAlreadyClaimedEntry',
		},
		{
			code: 6013,
			name: 'CompetitorNeedsToRebaseInsuranceFundStake',
			msg: 'CompetitorNeedsToRebaseInsuranceFundStake',
		},
		{
			code: 6014,
			name: 'CompetitorHasNoUnclaimedWinnings',
			msg: 'CompetitorHasNoUnclaimedWinnings',
		},
		{
			code: 6015,
			name: 'CompetitionRoundNumberIssue',
			msg: 'CompetitionRoundNumberIssue',
		},
		{
			code: 6016,
			name: 'CompetitorSnapshotIssue',
			msg: 'CompetitorSnapshotIssue',
		},
	],
};
