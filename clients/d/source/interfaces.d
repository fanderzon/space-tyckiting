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

import vibe.data.serialization;

struct TykitingServerConfig
{
    int bots;
    int fieldRadius;
    int move;
    int startHp;
    int cannon;
    int radar;
    int see;
    int maxCount;
    int asteroids;
    int loopTime;
    bool noWait;
}

struct Pos
{
    int x, y;
}

struct Bot
{
    uint botId;
    string name;
    int teamId;
    bool alive;
    @optional
    Pos pos;
    @optional
    uint hp = 2;
}

struct Team
{
    string name;
    int teamId;
    Bot[] bots;
}

enum GameEventType
{
    damaged,
    hit,
    die,
    see,
    seeAsteroid,
    radarEcho,
    detected,
    noaction,
    move
}

class GameEvent {
    @name("event") @byName GameEventType type;
    @optional
    int botId;
    @optional
    int source;
    @optional
    Pos pos;
    @optional
    int damage;
}

class ServerMessage
{
    string type;
}

class ConnectedMessage: ServerMessage
{
    int teamId;
    TykitingServerConfig config;
}

class StartMessage: ServerMessage
{
    Team you;
    Team[] otherTeams;
}


class EndMessage: ServerMessage
{
    @optional
    int winnerTeamId = -1;
}


class EventsMessage: ServerMessage
{
    int roundId;
    Team you;
    Team[] otherTeams;
    GameEvent[] events;
    TykitingServerConfig config;
}


class ErrorMessage: ServerMessage
{
    string data;
}


class BotAction
{
    int botId;
    Pos pos;
    string type = "unknown";
    this(int botId, Pos pos)
    {
        this.botId = botId;
        this.pos = pos;
    }
}

class CannonAction: BotAction
{
    this(int botId, Pos pos)
    {
        super(botId, pos);
        type = "cannon";
    }
}

class MoveAction: BotAction
{
    this(int botId, Pos pos)
    {
        super(botId, pos);
        type = "move";
    }
}

class RadarAction: BotAction
{
    this(int botId, Pos pos)
    {
        super(botId, pos);
        type = "radar";
    }
}

class AI
{
    string teamName;
    TykitingServerConfig config;
    abstract BotAction[] makeDecisions(int roundId, GameEvent[] events, Bot[] myBots, Bot[] otherBots,  TykitingServerConfig config)
    out(actions)
    {
        import std.algorithm: canFind, filter, find;
        import std.range: walkLength;

        assert(myBots.filter!(a => a.alive).walkLength == actions.length,
            "Number of actions doesn't match nr. of alive bots");
        foreach (action; actions)
        {
            auto selectedBot = myBots.filter!(a => a.botId == action.botId);
            assert(!selectedBot.empty, "Invalid bot id");
            Bot bot = selectedBot.front;
            Pos pos = action.pos;
            if(cast(MoveAction) action && action.type == "move")
            {
                assert(getValidMoves(bot)[].canFind(pos));
            }
            else if(cast(RadarAction) action && action.type == "radar")
            {
                assert(getValidRadars(bot)[].canFind(pos));
            }
            else if(cast(CannonAction) action && action.type == "cannon")
            {
                assert(getValidCannons(bot)[].canFind(pos));
            }
            else
            {
                assert(0, "Invalid action");
            }
        }
    }
    body{
        BotAction[] b;
        return b;
    }


    final auto getValidMoves(Bot bot)
    {
        return getPositionsInRange(bot.pos.x, bot.pos.y, config.move);
    }

    final auto getValidCannons(Bot bot)
    {
        return getPositionsInRange(0, 0, config.fieldRadius);
    }

    final auto getValidRadars(Bot bot)
    {
        return getPositionsInRange(0, 0, config.fieldRadius);
    }

    final auto getPositionsInRange(int x=0, int y=0, int radius=1)
    {
        import std.container;
        import std.range: iota;
        import std.algorithm: min, max;

        SList!Pos slist;
        foreach (dx;iota(-radius, radius+1))
        {
            foreach (dy; iota(max(-radius, -dx-radius), min(radius, -dx+radius)+1))
            {
                slist.insert(Pos(dx+x, dy+y));
            }
        }
        return slist;
    }
}

class JoinClientMessage: ServerMessage
{
    string type = "join";
    string teamName;
    this(string name)
    {
        this.teamName = name;
    }
}

class ActionClientMessage: ServerMessage
{
    string type = "actions";
    int roundId;
    BotAction[] actions;
    this(int roundId, BotAction[] actions)
    {
        this.roundId = roundId;
        this.actions = actions;
    }
}
