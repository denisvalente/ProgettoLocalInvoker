var express = require('express');
const { getFunctions } = require('./fn');
const { getScalers } = require('./scaler');
const { getTriggers } = require('./trigger');
const fs = require('fs');
var router = express.Router();
const FILERULES = process.env.FILERULES === undefined ? "./config/rules.json" : process.env.FILERULES;

let rawdata = fs.readFileSync(FILERULES);
let def = JSON.parse(rawdata);



/* GET home page. */
router.get('/', function(req, res, next) {
  let functions= JSON.stringify([...getFunctions()]) ;
  let triggers= JSON.stringify([...getTriggers()]) ;
  let scalers= JSON.stringify([...getScalers()]) ;
  res.render('dashboard', { title: 'Express',id: 'perfvass', functions,triggers,scalers });
});

router.get('/createPipeline', function(req, res, next) {
  res.render('index', { title: 'Pipeline creation',id: 'perfvass', def });
});

module.exports = router;
