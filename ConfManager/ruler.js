const fs = require('fs');
const { createRule: createTriggerRule, setTrigger, getTrigger } = require('./routes/trigger');
const { createRule: createScalerRule, setScaler, getScaler } = require('./routes/scaler');
const {setFunction, getFunction} = require('./routes/fn');
const FILERULES = process.env.FILERULES === undefined ? "./config/rules.json" : process.env.FILERULES;


function loadFile(file=FILERULES) {
    let rawdata = fs.readFileSync(file);
    let rules = JSON.parse(rawdata);

    pipeline2Rules(rules);
}



function pipeline2Rules(rules) {
    let funcs = new Map();
    rules.fn.forEach(fn => funcs.set(fn.id, fn));
    rules.fn.forEach(fn => setFunction(fn.id, fn));

    let triggers = new Map();
    rules.triggers.forEach(trig => triggers.set(trig.id, trig));
    rules.triggers.forEach(trig => setTrigger(trig.id, trig));

    let scalers = new Map();
    rules.scalers.forEach(scal => scalers.set(scal.id, scal));
    rules.scalers.forEach(scal => setScaler(scal.id, scal));


    rules.pipelines.forEach(pipe => {
        createTriggerRule(pipe.trigger, pipe.id, getTrigger(pipe.trigger).type, pipe.medium, pipe.topic);
        createScalerRule(pipe.scaler.id, pipe.id, getScaler(pipe.scaler.id).type, getTrigger(pipe.trigger).address,
            getTrigger(pipe.trigger).port, pipe.requestxc, pipe.threshold, pipe.cntupperbound, pipe.scaler.reschedulingms,
            getFunction(pipe.fn).container, getFunction(pipe.fn).envvar, pipe.topic, pipe.group, pipe.outputTopic);
    });
}


module.exports.loadFile = loadFile
module.exports.pipeline2Rules=pipeline2Rules