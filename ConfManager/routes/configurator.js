var express = require('express');
var router = express.Router();
var Docker = require('dockerode');
const { pipeline2Rules } = require('../ruler');


router.post('/', function (req, res, next) {
    pipeline2Rules(req.body);
    res.send("ok");

});


module.exports = router;
