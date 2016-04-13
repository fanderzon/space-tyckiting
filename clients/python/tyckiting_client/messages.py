"""
Collection of messages received from server.
"""

import collections


# Nested objects
class Config(object):
    def __init__(self, bots=3, fieldRadius=14, move=2, startHp=10, cannon=1,
                 radar=3, see=2, maxCount=200, loopTime=300, **kwargs):
        self.bots = bots
        self.field_radius = fieldRadius
        self.move = move
        self.start_hp = startHp
        self.cannon = cannon
        self.radar = radar
        self.see = see
        self.max_count = maxCount
        self.loop_time = loopTime


Pos = collections.namedtuple('Pos', ['x', 'y'])


class Bot(object):
    def __init__(self, botId, name, teamId, alive=None, pos=None, hp=None, **kwargs):
        self.bot_id = botId
        self.name = name
        self.team_id = teamId
        self.alive = alive
        self.pos = Pos(**pos) if pos else None
        self.hp = hp


class Team(object):
    def __init__(self, name, teamId, bots=None, **kwargs):
        self.name = name
        self.team_id = teamId
        self.bots = map(lambda b: Bot(**b), bots or [])


class Event(object):
    def __init__(self, event, botId=None, source=None, damage=None, pos=None,
                 winnerTeamId=None, **kwargs):
        self.event = event
        self.bot_id = botId
        self.source = source
        self.damage = damage
        self.pos = Pos(**pos) if pos else None
        self.winner_team_id = winnerTeamId


# Message types
class Message(object):
    def __init__(self, type, **kwargs):
        self.type = type


class Connected(Message):
    def __init__(self, teamId=None, config=None, **kwargs):
        super(Connected, self).__init__(**kwargs)
        self.team_id = teamId
        self.config = Config(**(config or {}))


class Start(Message):
    def __init__(self, you=None, otherTeams=None, **kwargs):
        super(Start, self).__init__(**kwargs)
        self.you = Team(**(you or {}))
        self.other_teams = map(lambda t: Team(**t), otherTeams or [])


class End(Message):
    def __init__(self, winnerTeamId, **kwargs):
        super(End, self).__init__(**kwargs)
        self.winner_team_id = winnerTeamId


class Events(Message):
    def __init__(self, roundId, you=None, otherTeams=None, events=None,
                 config=None, **kwargs):
        super(Events, self).__init__(**kwargs)
        self.round_id = roundId
        self.you = Team(**(you or {}))
        self.other_teams = map(lambda t: Team(**t), otherTeams or [])
        self.events = map(lambda e: Event(**e), events or [])
        self.config = Config(**(config or {}))


class Error(Message):
    def __init__(self, data, **kwargs):
        self.data = data

TYPE_MAP = {
    'connected': Connected,
    'start': Start,
    'end': End,
    'events': Events,
    'error': Error,
}
