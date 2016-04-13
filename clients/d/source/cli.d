/** D Tyckiting client - A websocket client for a fight to kill all other bots
 *  Copyright greenify (2016)
 *
 *  This file is part of D Tyckiting client.
 *
 *  D Tyckiting client is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation, either version 3 of the License, or
 *  (at your option) any later version.
 *
 *  Rust Tyckiting client is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU General Public License for more details.
 *
 *  You should have received a copy of the GNU General Public License
 *  along with D Tyckiting client.  If not, see <http://www.gnu.org/licenses/>.
 */

import std.getopt;

struct GameConfig
{
    string host = "localhost";
    ushort port = 3000;
    string name = "Awesome D bot";
    bool verbose;
}

GameConfig parseCLI(string[] args)
{
    GameConfig gconfig;
    auto helpInformation = getopt(
    args,
    config.passThrough,
    "host|H",  &gconfig.host,
    "port|P",    &gconfig.port,
    "verbose|v", &gconfig.verbose,
    "name|n", &gconfig.name,
    );
    if (helpInformation.helpWanted || args.length > 1)
    {
        import std.c.stdlib;
        defaultGetoptPrinter("D Tyckiting client ",
        helpInformation.options);
        exit(0);
    }
    return gconfig;
}
