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
import std.conv;
import std.array;
import std.algorithm;
import std.exception;
import core.sys.posix.signal;

import vibe.http.websockets : WebSocket, WebSocketException, connectWebSocket;
import vibe.inet.url;
import vibe.data.json;

public import interfaces;
public import cli;

// set correct exit behavior
extern(C) void close(int signal)
{
    import std.c.stdlib;
    if(signal == SIGINT)
    {
        exit(-1);
    }
}

static this()
{
    sigset(SIGINT, &close);
}

class TykitingClient
{
    public Team team;
    public TykitingServerConfig config;
    URL url;
    AI ai;
    WebSocket ws;
    bool activeGame = true;

    this(string host, ushort port, string teamName, AI ai)
    {
        this.url = URL("ws://", host, port, Path(""));
        team.name = teamName;
        this.ai = ai;
    }

    void close()
    {
        activeGame = false;
    }

    void run()
    {
        try
        {
            ws = connectWebSocket(url);
        }
        catch(Exception ex)
        {
            // host is down etc.
            writeln(ex.msg);
            return;
        }
        scope(exit) ws.close;
        while (activeGame) {
            if (!ws.connected) break;
            string text;
            try
            {
                text = ws.receiveText;
            }catch(WebSocketException ex)
            {
                writeln("Connection to server failed.");
                break;
            }
            auto message = text.parseJsonString;
            writeln(message["type"]);
            ServerMessage msg;
            switch(message["type"].to!string){
                case "connected":
                    team.teamId = message["teamId"].to!uint;
                    msg = deserializeJson!ConnectedMessage(message);
                    JoinClientMessage joinMsg = new JoinClientMessage(team.name);
                    ws.send(joinMsg.serializeToJsonString);
                    break;
                case "start":
                    auto startMessage = deserializeJson!StartMessage(message);
                    msg = startMessage;
                    break;
                case "events":
                    auto eventsMessage = deserializeJson!EventsMessage(message);
                    msg = eventsMessage;
                    // ask the AI what to do
                    BotAction[] actions = ai.makeDecisions(eventsMessage.roundId, eventsMessage.events,
                        eventsMessage.you.bots, eventsMessage.otherTeams.map!("a.bots").join, eventsMessage.config);
                    auto acm = new ActionClientMessage(eventsMessage.roundId, actions);
                    acm.serializeToJsonString.writeln;
                    ws.send(acm.serializeToJsonString);
                    break;
                case "end":
                    auto endMessage = deserializeJson!EndMessage(message);
                    msg = endMessage;
                    if (team.teamId == endMessage.winnerTeamId)
                        writeln("You win");
                    else if(endMessage.winnerTeamId == -1)
                        writeln("Tie");
                    else
                        writefln("%d wins", endMessage.winnerTeamId);
                    close();
                    break;
                case "error":
                    msg = deserializeJson!ErrorMessage(message);
                    writeln("Error", msg);
                    break;
                default:
                    writeln("Unknown msg", msg);
            }
            if(msg !is null)
                msg.writeln;
        }
    }
}
