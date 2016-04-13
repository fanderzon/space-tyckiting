// Starts game client

var express = require('express');
var argv = require('minimist')(process.argv.slice(2));


var port = argv.port || 3030;

var app = express();
app.use('/app', express.static(__dirname + '/spectator'));
app.get('/', function(req, res) {
    res.sendFile(__dirname + '/spectator/index.html');
});

app.use('/visual', express.static(__dirname + '/visual'));

app.listen(port, function() {
    console.log('\n*** Space Tyckiting - Spectator ***\n');
    console.log(' - URL: http://localhost:' + port);
});
