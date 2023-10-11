const fs = require('fs');
const path = require('path');
const driftCompetitionIdl = require('../idl/drift_competitions.json');

let uiErrors = {
	errorsList: {},
	errorCodesMap: {},
};

const DRIFT_ERRORS_PATH = path.resolve(
	__dirname,
	'../constants/driftCompetitionsErrors.json'
);

try {
	uiErrors = require(DRIFT_ERRORS_PATH);
} catch (e) {
	console.log("ui errors file doesn't exist yet");
}

// errorCodesMap should get reset every time, because the numbers could potentially change in the protocol's output
// UI will use this to map the error code (number) to the name, so we can keep our manually added messages identified by the name but independent of the number
uiErrors.errorCodesMap = {};

driftCompetitionIdl.errors.forEach((err) => {
	uiErrors.errorCodesMap[err.code] = err.name;
	if (!uiErrors.errorsList[err.name]) {
		uiErrors.errorsList[err.name] = err;
	} else {
		// Only update error code in errorsList
		uiErrors.errorsList[err.name].code = err.code;
	}
});

const copyErrorCodes = () => {
	fs.writeFileSync(
		DRIFT_ERRORS_PATH,
		`${JSON.stringify(uiErrors, null, '	')}\n`
	);
};

copyErrorCodes();
