var express = require('express');
var router = express.Router();

/* TRIGGER*/
let triggers = new Map();

function getTriggers() {
    return triggers
}
router.get('/', function (req, res, next) {
    res.send(JSON.stringify(triggers));
});

function setTrigger(id, trigger) {
    triggers.set(id, trigger);
}
router.post('/:id', function (req, res, next) {

    setTrigger(req.params.id, req.body);
    res.send("ok");
})

function getTrigger(id) {
    return triggers.get(id);
}
router.get('/:id', function (req, res, next) {
    res.send(JSON.stringify(getTrigger(req.params.id)));
});

/* RULES */
let rules = new Map();

router.get('/rules/:id', function (req, res, next) {
    if (req.query.format === "JSON") {
        let obj = []
        rules.get(req.params.id)
            .forEach((v, k) => {
                obj.push({ id: k, rule: v })
            })
        res.send(obj);
    } else {
	console.log(rules.get(req.params.id));
        res.send(JSON.stringify([...rules.get(req.params.id)]));
    }
});

router.post('/rules/:id', function (req, res, next) {
    	
	let body = req.body;
		    rules.set(req.params.id, req.body); createRule(req.params.id, body.ruleid, body.triggerEndpoyntType, body.medium, body.topic);
		    res.send("ok");
		});

		function createRule(id, ruleid, triggerEndpoyntType, medium, topic) {

		    let rul = rules.has(id) ? rules.get(id) : new Map();
		    let obj = {
		        triggerEndpoyntType,
		        medium,
		        topic
		    }
	//console.log(typeof rul);
   rul.set(ruleid, obj);
    rules.set(id, rul);
}


module.exports = router;
module.exports.getTriggers = getTriggers;
module.exports.createRule = createRule;
module.exports.setTrigger = setTrigger;
module.exports.getTrigger = getTrigger;
