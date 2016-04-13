var _ = require('lodash');

var radarPoints = [];
var rowStart = 0;
var rowEnd;
var rowStartChange;
var rowEndChange = 0;

function getRadarPoints( config ) {
  var point = true;
   rowEnd = config.fieldRadius - config.radar;
   rowStartChange = -( config.radar + 1);

  while( point ) {
    point = findRadarPoint( config );
    radarPoints.push( point );
  }

  return radarPoints;
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

  console.log( 'lastPoint', lastPoint, rowEnd, rowEndChange );
  if (lastPoint.x >= rowEnd ) {
    rowStart = rowStart + rowStartChange;
    rowEnd = rowEnd + rowEndChange;

    if (rowStart <= -(config.fieldRadius - config.radar)) {
      console.log('too low rowStart', rowStart);
      rowStart = -(config.fieldRadius - config.radar);
      rowStartChange = 0;
      rowEndChange = -( config.radar + 1);
      rowEnd = config.fieldRadius - ( (lastPoint.y + 4) || 0) - config.radar;
      // When the y coord is larger than the radius let's exit
      if (lastPoint.y + ( config.radar + 1 ) > config.fieldRadius) {
        return null;
      }
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
