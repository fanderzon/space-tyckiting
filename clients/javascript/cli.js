"use strict";

var program = require("commander");
var main = require("./main.js");

program
  .version("0.1.0")
  .option("-a, --ai <AI>", "Select AI")
  .option("-H, --host [host]", "Host to connect to")
  .option("-P, --port [port]", "Port to connect to")
  .option("-n, --teamname <name>", "Bot's name")
  .option("-w, --webgame <spec>", "Webgame spec")
  .parse(process.argv);

/* eslint-disable no-multi-spaces */
var host = program.host     || (program.webgame ? "tyckiting.futurice.com" : "localhost");
var port = program.port     || (program.webgame ? 80 : 3000);
var name = program.teamname || "Anon.JS";
var ai   = program.ai       || "dummy";
/* eslint-enable no-mult-spaces */

main(host, port, name, ai, program.webgame);
