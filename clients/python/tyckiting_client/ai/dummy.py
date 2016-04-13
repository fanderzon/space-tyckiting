import random

from tyckiting_client.ai import base
from tyckiting_client import actions


class Ai(base.BaseAi):
    """
    Dummy bot that moves randomly around the board.
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

            move_pos = random.choice(list(self.get_valid_moves(bot)))
            response.append(actions.Move(bot_id=bot.bot_id,
                                         x=move_pos.x,
                                         y=move_pos.y))
        return response
