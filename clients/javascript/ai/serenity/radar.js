function findRadarPoint() {
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
