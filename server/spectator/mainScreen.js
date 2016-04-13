/*global define*/
"use strict";

define([
    "lodash",
    "svg",
    "hex",
    "cube",
    "point",
    "hexGrid",
    "colors"
], function (
    _,
    svg,
    Hex,
    Cube,
    Point,
    HexGrid,
    Colors
) {

    return function (root, gridSize, fieldRadius, blastRadius, detectRadius) {

        var width = (fieldRadius || 12) * 2 + 1;
        var height = (fieldRadius || 12) * 2 + 1;
        var BLAST_RADIUS = blastRadius || 1;
        var DETECT_RADIUS = detectRadius || 3;

        var SQRT_3_2 = Math.sqrt(3) / 2;

        var COLOR_RED = "#f00";
        var COLOR_GREEN = "#0f0";
        var COLOR_MID_GREEN = "#0a0";

        var COLOR_MID_GREY = "#777";

        var ships = {};
        var temp = [];

        var asteroids = [];
        var asteroidsSvgs = [];

        var HEX_HEIGHT = gridSize * 2;
        var HEX_WIDTH = (Math.sqrt(3) / 2) * HEX_HEIGHT;
        var VERT = HEX_HEIGHT * 3 / 4;
        var HORIZ = HEX_WIDTH;
        var QUARTER_HEIGHT = HEX_HEIGHT / 4;
        var HALF_WIDTH = HEX_WIDTH / 2;

        var draw = svg(root).size(width * HORIZ, height * VERT + HEX_HEIGHT / 4);
        var shipTemplate = draw.defs().path(shipSvgPath(gridSize));
        var asteroidTemplate = draw.defs().path(asteroidSvgPath(gridSize));
        var hexTemplate = draw.defs().path(hexSvgPath(gridSize));

        var main = draw.group();

        init();

        function init() {
            var pattern = draw.pattern(HORIZ, HEX_HEIGHT + gridSize, function (add) {
                var path = "M WIDTH 0 l WIDTH 4_HEIGHT m 0 " +
                    "H_HEIGHT l -WIDTH 4_HEIGHT l 0 H_HEIGHT m 0 -H_HEIGHT " +
                    "l -WIDTH -4_HEIGHT l 0 -H_HEIGHT l WIDTH -4_HEIGHT";

                path = path.replace(/WIDTH/g, HALF_WIDTH)
                    .replace(/4_HEIGHT/g, QUARTER_HEIGHT)
                    .replace(/H_HEIGHT/g, gridSize);

                add.path(path).stroke({width: 1, color: COLOR_MID_GREY});
            });

            draw.rect("100%", "100%").fill(pattern);

            main.front();

            main.translate(fieldRadius * HORIZ + HALF_WIDTH, fieldRadius * VERT + gridSize);

            var hexes = HexGrid.hexesInRadius(new Hex(0, 0), fieldRadius);

            hexes.reduce(function (memo, hex) {
                var hexTile = drawHex(hex, COLOR_MID_GREEN, "none", true);
                main.add(hexTile);
                memo[hex.toString()] = hexTile;
                return memo;
            }, {});
        }

        function getAngle(startPoint, endPoint) {
            return 90 + Math.atan2(endPoint.y - startPoint.y, endPoint.x - startPoint.x) * 180 / Math.PI;
        }

        function shipSvgPath(size) {
            var halfWidth = SQRT_3_2 * size;
            var quarterHeight = size / 2;
            var template =
                "M 0 -4_HEIGHT L 4_HEIGHT 4_HEIGHT L -4_HEIGHT 4_HEIGHT  L 0 -4_HEIGHT";
            return template.replace(/WIDTH/g, halfWidth).replace(/4_HEIGHT/g, quarterHeight).replace(/H_HEIGHT/g, size);
        }

        function asteroidSvgPath(size) {
            var halfWidth = SQRT_3_2 * size;
            var quarterHeight = size / 2;
            var template =
                "M -WIDTH -H_HEIGHT m 0 4_HEIGHT l WIDTH -4_HEIGHT " +
                "l WIDTH 4_HEIGHT l 0 H_HEIGHT l -WIDTH 4_HEIGHT " +
                "l -WIDTH -4_HEIGHT l 0 -H_HEIGHT";
            return template
                .replace(/WIDTH/g, halfWidth)
                .replace(/4_HEIGHT/g, quarterHeight)
                .replace(/H_HEIGHT/g, size);
        }

        function hexSvgPath(size) {
            var halfWidth = SQRT_3_2 * size;
            var quarterHeight = size / 2;
            var template =
                "M -WIDTH -H_HEIGHT m 0 4_HEIGHT l WIDTH -4_HEIGHT " +
                "l WIDTH 4_HEIGHT l 0 H_HEIGHT l -WIDTH 4_HEIGHT " +
                "l -WIDTH -4_HEIGHT l 0 -H_HEIGHT";
            return template.replace(/WIDTH/g, halfWidth).replace(/4_HEIGHT/g, quarterHeight).replace(/H_HEIGHT/g, size);
        }

        function drawHex(hex, color, fill, includeText) {
            var location = HexGrid.hexToPoint(hex, gridSize);

            var hexGroup = draw.group();
            hexGroup.add(draw.use(hexTemplate).stroke({width: 1, color: color}).attr({
                fill: fill,
                "fill-opacity": 0.4}));

            if (includeText) {
                hexGroup.add(draw.text(hex.q + ", " + hex.r)
                    .attr({fill: color, color: color, "font-size": "8px", "text-anchor": "middle"})
                    .move(0, 2));
            }

            return hexGroup.move(location.x, location.y);
        }

        function drawArea(x, y, radius, color, fill, targetGroup) {
            var group = targetGroup || draw.group();
            var results = HexGrid.hexesInRadius(new Hex(x, y), radius);
            results.forEach(function (hex) {
                group.add(drawHex(hex, color, fill));
            });
            return group;
        }

        function addShip(id, x, y, teamIndex) {
            var fill = Colors.bg(teamIndex);
            var color = Colors.main(teamIndex);

            var target = HexGrid.hexToPoint(new Hex(x, y), gridSize);
            var nested = draw.nested();
            var drawing = nested
                .use(shipTemplate)
                .fill(fill).stroke({width: 1, color: color});

            nested.move(target.x, target.y);

            ships[id] = {
                pos: target,
                svg: drawing,
                base: nested
            };
            main.add(ships[id].base);
        }

        function removeShip(id) {
            ships[id].svg.remove();
            delete ships[id];
        }

        function hasShip(id) {
            return ships[id];
        }

        function moveShip(id, x, y) {
            if (ships[id]) {
                var start = ships[id].pos;
                var end = HexGrid.hexToPoint(new Hex(x, y), gridSize);
                if (start.x !== end.x || start.y !== end.y) {
                    var angle = getAngle(start, end);
                    ships[id].base.animate(100).move(end.x, end.y);
                    ships[id].svg.rotate(angle, 0, 0);
                    ships[id].pos = end;
                }
            }
        }

        function rotateShipInPlace(id, endPoint) {
            if (ships[id]) {
                var angle = getAngle(ships[id].pos, endPoint);
                ships[id].svg.animate(100).rotate(angle, 0, 0);
            }
        }

        function destroyShip(id) {
            if (ships[id]) {
                ships[id].svg.stroke({width: 2, color: COLOR_MID_GREY}).fill(COLOR_MID_GREY);
            }
        }

        function drawLineWithArea(botId, x, y, radius, lineColor, fillColor) {
            if (ships[botId]) {
                var set = draw.group();

                var sourcePoint = ships[botId].pos;
                var targetPoint = HexGrid.hexToPoint(new Hex(x, y), gridSize);

                rotateShipInPlace(botId, targetPoint);

                set.add(draw.line(sourcePoint.x, sourcePoint.y, targetPoint.x, targetPoint.y)
                    .stroke({ width: 1, color: lineColor })
                    .back()
                    .forward());

                drawArea(x, y, radius, lineColor, fillColor, set);

                temp.push(set);
                main.add(set);
            }
        }

        function blast(botId, x, y, teamIndex) {
            drawLineWithArea(botId, x, y, BLAST_RADIUS, Colors.main(teamIndex), COLOR_RED);
        }

        function radar(botId, x, y, teamIndex) {
            drawLineWithArea(botId, x, y, DETECT_RADIUS, Colors.main(teamIndex), Colors.bg(teamIndex));
        }

        function clear() {
            temp.forEach(function (elem) {
                elem.remove();
            });
            temp = [];
        }

        function addAsteroid(pos) {
            var asteroid = _.find(asteroids, function (a) {
                return _.isEqual(pos, a);
            });

            // already added
            if (asteroid) {
                return;
            }

            // adding asteroid
            asteroids.push(pos);

            // draw
            var fill = COLOR_MID_GREEN;
            var color = COLOR_GREEN;

            var target = HexGrid.hexToPoint(new Hex(pos.x, pos.y), gridSize);
            var drawing = draw
                .use(asteroidTemplate)
                .fill(fill).stroke({width: 1, color: color})
                .move(target.x, target.y);

            asteroidsSvgs.push(drawing);
            main.add(drawing);
        }

        function clearAll() {
            clear();
            _.each(ships, function (ship) {
                ship.svg.remove();
            });
            temp = [];
            ships = {};

            // remove asteroids
            _.each(asteroidsSvgs, function (astSvg) {
                astSvg.remove();
            });
            asteroidsSvgs = [];
            asteroids = [];
        }

        return {
            radar: radar,
            blast: blast,
            addAsteroid: addAsteroid,
            addShip: addShip,
            hasShip: hasShip,
            moveShip: moveShip,
            removeShip: removeShip,
            destroyShip: destroyShip,
            clear: clear,
            clearAll: clearAll
        };
    };
});
