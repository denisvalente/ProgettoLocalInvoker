var express = require('express');
const { getFn } = require('./fn');
var router = express.Router();

let scalers = new Map();

/* SCALER */
function getScalers(){
    return scalers;
}
router.get('/', function (req, res, next) {
    res.send(JSON.stringify(scaler));
});

function setScaler(id,scaler){
    scalers.set(id, scaler);
}
router.post('/:id', function (req, res, next) {
    setScaler(req.params.id, req.body);
    res.send("ok");
})

function getScaler(id){
    return scalers.get(id)
}
router.get('/:id', function (req, res, next) {
    res.send(JSON.stringify(getScaler(req.params.id)));
});


/* RULES */
let rules = new Map();

router.get('/rules/:id', function (req, res, next) {
    res.send(JSON.stringify([...rules.get(req.params.id)]));
});

router.post('/rules/:id', function (req, res, next) {
    let body = req.body;
    let container = getFn(body.functionID).container;
    let envvar = getFn(body.functionID).envvar;
    createRule(id, body.ruleid, body.endpoyntType, body.host, body.port,
        body.requestxc, body.threshold, body.cntupperbound, body.reschedulingms,
        container, envvar, body.topic, body.group, body.outputTopic);
    res.send("ok");
});

function createRule(id, ruleid, endpoyntType, host, port,
    requestxc, threshold, cntupperbound, reschedulingms, container, envvar, topic, group, outputTopic) {
    let rul = rules.has(id) ? rules.get(id) : new Map();
    let obj = {
        endpoyntType,
        host,
        port,
        requestxc,
        threshold,
        cntupperbound,
        reschedulingms,
        container,
        triggerTopic: topic,
        group,
        outputTopic,
        envvar
    }
    rul.set(ruleid, obj);
    rules.set(id, rul);
}


module.exports = router;
module.exports.getScalers = getScalers;
module.exports.createRule = createRule;
module.exports.setScaler = setScaler;
module.exports.getScaler = getScaler;