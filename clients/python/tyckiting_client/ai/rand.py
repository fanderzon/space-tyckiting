"""
We can't name this file random, since it will conflict with the import of the
'random' library.
"""

import random

from tyckiting_client.ai import base
from tyckiting_client import actions


class Ai(base.BaseAi):
    """
    Random bot that moves performs random actions.
    """
    def move(self, bots, events):
        """
        Move the bot to a random legal positon.

        Args:
            bots: List of bot states for own team
            events: List of events form previous round

        Returns:
            List of actions to perform this round.
        """
        response = []
        for bot in bots:
            if not bot.alive:
                continue

            action_handler = random.choice([self.move_random,
                                            self.cannon_random,
                                            self.radar_random])

            response.append(action_handler(bot))

        return response

    def move_random(self, bot):
            move_pos = random.choice(list(self.get_valid_moves(bot)))
            return actions.Move(bot_id=bot.bot_id,
                                x=move_pos.x,
                                y=move_pos.y)

    def cannon_random(self, bot):
            cannon_pos = random.choice(list(self.get_valid_cannons(bot)))
            return actions.Cannon(bot_id=bot.bot_id,
                                  x=cannon_pos.x,
                                  y=cannon_pos.y)

    def radar_random(self, bot):
            radar_pos = random.choice(list(self.get_valid_radars(bot)))
            return actions.Radar(bot_id=bot.bot_id,
                                 x=radar_pos.x,
                                 y=radar_pos.y)
