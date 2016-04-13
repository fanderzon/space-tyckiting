/* global describe:true, it:true */
"use strict";

// Reference: http://chaijs.com/api/bdd/
var expect = require("chai").expect;
var dummy = require("../ai/dummy/ai.js");
var position = require("../position.js");

describe("dummy", function () {
  it("teamName starts with Dummy", function () {
    var ai = dummy();
    expect(ai.teamName).to.match(/^Dummy/);
  });
});

describe("position.neighbours", function () {
  it("return 6 positions when radius is 1", function () {
    var ps = position.neighbours(position.origo, 1);
    expect(ps.length).to.equal(6);
  });
});
