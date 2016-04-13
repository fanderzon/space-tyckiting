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

import std.stdio;
import client;

class DummyAI: AI
{
    override BotAction[] makeDecisions(int roundId, GameEvent[] events, Bot[] myBots, Bot[] otherBots,  TykitingServerConfig config)
    body
    {
        // process the previous events
        foreach (event; events)
        {
            final switch(event.type) with(GameEventType)
            {
                case damaged:
                    break;
                case hit:
                    break;
                case die:
                    break;
                case see:
                    break;
                case seeAsteroid:
                    break;
                case radarEcho:
                    break;
                case detected:
                    break;
                case noaction:
                    writeln("WARNING - no action submitted by client");
                    break;
                case move:
                    break;
            }
        }


        BotAction[] actions = new BotAction[](myBots.length);

        import std.random;
        import std.array: array;

        foreach (i,bot; myBots)
        {
            // we randomly pick a valid action
            final switch(roundId % 3)
            {
                case 0:
                    Pos pos = getValidMoves(bot).array.randomSample(1).front;
                    actions[i] = new MoveAction(bot.botId, pos);
                    break;
                case 1:
                    Pos pos = getValidCannons(bot).array.randomSample(1).front;
                    actions[i] = new CannonAction(bot.botId, pos);
                    break;
                case 2:
                    Pos pos = getValidRadars(bot).array.randomSample(1).front;
                    actions[i] = new RadarAction(bot.botId, pos);
                    break;
            }
        }
        return actions;
    }
}

void main(string[] args)
{
    auto config = parseCLI(args);
    auto ai = new DummyAI();
    auto cl = new TykitingClient(config.host, config.port, config.name, ai);
    cl.run;
}
