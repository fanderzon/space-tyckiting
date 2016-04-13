var position = require('./javascript/position.js');

var first = position.make( -1, 20 );

// Clamp makes sure the position is within bounds
// console.log( position.clamp( first, 28 ) );

// Neighbours gives you all neighbouring positions, up to the distance that's specified as radius
// console.log( position.neighbours( first, 1 ) );

// console.log( position.distance( first, { x: first.x - 2, y: first.y + -2 } ) )


function getMaxDistanceMoves( maxMoves ) {
  var start = position.make(0,0);
  return position.neighbours( start, maxMoves ).filter(
    function maxMoveFilter( pos ) {
      return position.distance( start, pos ) === maxMoves;
    }
  );
}

console.log( getMaxDistanceMoves( 3 ) );
