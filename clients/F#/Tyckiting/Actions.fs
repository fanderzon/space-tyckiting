module Actions

open Domain

let move botId point = 
    Move (botId,point)

let radar botId point = 
    Radar (botId,point)

let cannon botId point = 
    Cannon (botId,point)

