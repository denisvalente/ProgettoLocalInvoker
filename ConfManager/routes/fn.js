var express = require('express');
var router = express.Router();

let functions = new Map();

/* get all functions. */
function getFunctions(){
    return functions
}
router.get('/', function (req, res, next) {
    res.send(JSON.stringify([...functions]));
});

function getFunction(id) {
    return functions.get(id);
}
router.get('/:id', function (req, res, next) {
    res.send(JSON.stringify([...functions.get(req.params.id)]));
});

function setFunction(id, fn) {
    functions.set(id, fn)
}
router.post('/:id', function (req, res, next) {
    setFunction(req.param.id, {container:req.body.container, envvar:req.body.envvar})
    res.send("ok");

});


module.exports = router;
module.exports.getFunctions = getFunctions;
module.exports.getFunction = getFunction;
module.exports.setFunction = setFunction;
