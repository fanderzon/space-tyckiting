var _ = require('lodash');

var radarPoints = [];
var rowStart = 0;
var rowEnd;
var rowStartChange;
var rowEndChange = 0;

function shuffle(array) {
  var currentIndex = array.length, temporaryValue, randomIndex;

  // While there remain elements to shuffle...
  while (0 !== currentIndex) {

    // Pick a remaining element...
    randomIndex = Math.floor(Math.random() * currentIndex);
    currentIndex -= 1;

    // And swap it with the current element.
    temporaryValue = array[currentIndex];
    array[currentIndex] = array[randomIndex];
    array[randomIndex] = temporaryValue;
  }

  return array;
}

function getRadarPoints( config ) {
  var point = true;
   rowEnd = config.fieldRadius - config.radar;
   rowStartChange = -( config.radar + 1);

  while( point ) {
    point = findRadarPoint( config );
    if (point !== null) {
      radarPoints.push( point );
    }
  }
  return shuffle(radarPoints);
}

function Point(x, y) {
    this.x = x;
    this.y = y;
}


function findRadarPoint( config ) {
  //
  var lastPoint = radarPoints[ radarPoints.length - 1 ];
  if (!lastPoint) {
    // Start in the top left corner, since we set the center for the radar area
    // let's subtract whatever the radar radius is from the
    return new Point( rowStart, -(config.fieldRadius - config.radar) );
  }

  if (lastPoint.x >= rowEnd ) {
    rowStart = rowStart + rowStartChange;
    rowEnd = rowEnd + rowEndChange;

    if (rowStart <= -(config.fieldRadius)) {
      rowStart = -(config.fieldRadius - config.radar);
      rowStartChange = 0;
      rowEndChange = -( config.radar + 1);
      rowEnd = config.fieldRadius - ( (lastPoint.y + 4) || 0) - config.radar;
    }

    // When the y coord is larger than the radius let's exit
    if (lastPoint.y +  config.radar + 1  > config.fieldRadius) {
      return null;
    }
    return {
      x: rowStart,
      y: lastPoint.y + ( config.radar + 1 )
    };
  }

   return _.assign( {}, lastPoint, {
    x: lastPoint.x + 4
  });
}

module.exports = {
  getRadarPoints: getRadarPoints
};
