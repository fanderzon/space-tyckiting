"""
Collection of actions that may be sent to the server.

Each action should implement a `to_dict` method that transforms the object
to a dictionary.
"""


class Action(object):
    def __init__(self, bot_id=0):
        self.bot_id = bot_id


class PosAction(Action):
    def __init__(self, x=0, y=0, **kwargs):
        super(PosAction, self).__init__(**kwargs)
        self.x = x
        self.y = y

    def to_dict(self):
        return {
            'botId': self.bot_id,
            'type': self.type,
            'pos': {
                'x': self.x,
                'y': self.y
            }
        }


class Move(PosAction):
    type = 'move'


class Radar(PosAction):
    type = 'radar'


class Cannon(PosAction):
    type = 'cannon'
