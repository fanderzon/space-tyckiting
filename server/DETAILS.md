# Detailed documentation

Clients and servers communicate WebSockets (https://developer.mozilla.org/en-US/docs/Web/API/WebSockets_API).

## Client-side messages

All actions for bots are sent in an actions message. If several actions
for one round or one bot is provided only the latest in the round will be taken in account.

#### Join

After connecting to server, client needs to send join message

```json
{
   "type": "join",
   "teamName": <name of your team>
}
```

### Actions

Actions are sent with an actions-message. An action for each bot is set
in actions-array. If there are several actions for one bot, only the
latest is resolved. Similarly if multiple actions-messages are sent
during a round only the last one will be resolved.

```json
{
   "type": "actions",
   "roundId": <the id of the round>
   "actions": <array of actions>
}
```

### Action types

A bot's action can be one of the following: `move`, `radar` or `cannon`.

#### Move

```json
{
    "type": "move",
    "botId": <id of bot to move>>
    "pos": {
        "x": <target hex x-coordinate>,   
        "y": <target hex y-coordinate>
    }
}
```
#### Radar

```json
{
    "type": "radar",
    "botId": <id of bot to move>>
    "pos": {
        "x": <target hex x-coordinate>,   
        "y": <target hex y-coordinate>
    }
}
```
#### Cannon

```json
{
    "type": "cannon",
    "botId": <id of bot to move>>
    "pos": {
        "x": <target hex x-coordinate>,   
        "y": <target hex y-coordinate>
    }
}
```

## Server broadcasts to clients

* `connected` Send after a client connects to server

    ```json
    {
        "type": "connected",
        "teamId" <id of the team>,
        "config": <configurations as an object>
    }
    ```

* `start` Initial data of the game. This will be received after all teams have joined and the game is started.

    ```json
        {
        "type": "start",
        "you": {
            "name": <name of your team>,
            "teamId": <id of your team>,
            "bots": [<array of bots>]

        },
        "config": <configurations as an object>        
        "otherTeams": [<array of opponent teams>]
    ```

* `events` All bots receive events after each turn has been played. This message will tell you what happened after your move.

    ```json
    {
        "type": "events",
        "roundId": <the id of round>,
        "config": <configuration object>,
        "you": <team information>,
        "otherTeams": [<array of other teams>],
        "events": <array of events>
    }
    ```

    All possible events:

    * `hit` Bot has been hit. This might mean that you have been hit, or you hit another bot.
        ```json
        {
            "event": "hit",
            "botId": <the id of bot that was hit>,
            "source": <the id of bot that shot the hit>"
        }
        ```
    * `die` Bot has died. This might mean that you died, or that you killed another bot.
        ```json
        {
            "event": "die",
            "botId": <the id of bot that died>
        }
        ```
    * `see` Results seeing bots. You will never be included in this event.
        ```json
        {
            "event": "see",
            "source": <the id of bot that saw the target>"
            "botId": <the id of bot that was seen>,
            "pos": {
                "x": <x-coordinate of target>,
                "y": <y-coordinate of target>
            }
        }
        ```
    * `radarEcho` Results of radaring bots. You will never be included in this event.
        ```json
        {
            "event": "radarEcho",
            "pos": {
                "x": <x-coordinate of target that was detected>,
                "y": <y-coordinate of target that was detected>
            }
        }
        ```        
    * `detected` You will be notified that you have been seen or radared.
        ```json
        {
            "event": "detected",
            "botId": <the id of bot that was detected>
        }
        ```   
    * `damaged` Your bot was damaged
        ```json
        {
            "event": "damaged",
            "botId": <the id of bot that was damaged>,
            "damage" <amount of damage>
        }
        ```   
    * `move` One of your team's bots new position after a move action. Cannot move out of the map.
        ```json
        {
            "event": "move",
            "botId": <id of bot that moved>,
            "pos": {
                "x": <new x-coordinate of bot>,
                "y": <new y-coordinate of bot>
            }
        }
        ```   
    * `noaction` Bot performed no actions.
        ```json
        {
            "event": "noaction",
            "botId": <the id of bot that missed turn>
        }
        ```

* `end` Game has ended. Game ends when there are less than two bots alive.

    ```json
    {
        "type": "end",
        "winnerTeamId": <winner team id or -1 if a tie>,
        "you": <your team information>
    }
    ```
    
### Specific information types

* `configurations`
    ```json
        {
            "bots": <number of bots>,
            "fieldRadius": <size of the hex grid>,
            "move": <the maximum movement amount, inclusive>
            "startHp": <starting hit points>,
            "cannon": <cannon radius, direct damage is radius + 1, damage decay linearly as function of distance>,
            "radar": <radar radius, inclusive>,
            "see": <sight radius, inclusive>,
            "maxCount": <game length in rounds>,
            "asteroids": <asteroids>,
            "loopTime": <delay between rounds>,
            "noWait": <start next round if all players have registered their actions>
        }
    ```
* `Team specifications`
    ```json
        {
            "name": <name of the team>,
            "teamId": <the id of the team>,
            "bots": <array of bots. If an opponent, bots don't show hitpoints nor position>        
        }    
    ```
    
* `Bot`
    In general if a bot message is about an enemy bot, both current hitpoints and positions is omitted.

    ```json
        {
            "botId": <id of bot>,
            "name": <name of the bot>,
            "teamId": <id of the team of the bot>,
            "hp": <current hit points>,
            "alive": <true if bot is alive, false if destroyed>,
            "pos": {
                "x": <x-coordinate of bot>,
                "y": <y-coordinate of bot>
            }
        }
    ```
