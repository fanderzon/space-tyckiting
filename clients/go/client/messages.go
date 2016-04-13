package client

import "gopkg.in/guregu/null.v3"

type ClientMessageType string

const (
	CLIENT_JOIN    ClientMessageType = "join"
	CLIENT_ACTIONS ClientMessageType = "actions"
)

type BotActionType string

const (
	BOT_MOVE   BotActionType = "move"
	BOT_RADAR  BotActionType = "radar"
	BOT_CANNON BotActionType = "cannon"
)

type JoinMessage struct {
	Type     ClientMessageType `json:"type"`
	TeamName string            `json:"teamName"`
}

type ActionsMessage struct {
	Type    ClientMessageType `json:"type"`
	RoundId int               `json:"roundId"`
	Actions []Action          `json:"actions"`
}

type Position struct {
	X int `json:"x"`
	Y int `json:"y"`
}

type Action struct {
	Type     BotActionType `json:"type"`
	BotId    int           `json:"botId"`
	Position Position      `json:"pos"`
}

type ServerMessageType string

type ServerMessage struct {
	Type ServerMessageType `json:"type"`
}

const (
	SERVER_CONNECTED ServerMessageType = "connected"
	SERVER_START     ServerMessageType = "start"
	SERVER_END       ServerMessageType = "end"
	SERVER_EVENTS    ServerMessageType = "events"
	SERVER_ERROR     ServerMessageType = "error"
)

type EventType string

const (
	EVENT_HIT        EventType = "hit"
	EVENT_DIE        EventType = "die"
	EVENT_RADAR_ECHO EventType = "radarEcho"
	EVENT_SEE        EventType = "see"
	EVENT_DETECTED   EventType = "detected"
	EVENT_DAMAGED    EventType = "damaged"
	EVENT_MOVE       EventType = "move"
	EVENT_NOACTION   EventType = "noaction"
	EVENT_END        EventType = "end"
)

type ConnectedMessage struct {
	TeamId int        `json:"teamId"`
	Config GameConfig `json:"config"`
}

type GameConfig struct {
	Bots        int  `json:"bots"`
	FieldRadius int  `json:"fieldRadius"`
	Move        int  `json:"move"`
	StartHp     int  `json:"startHp"`
	Cannon      int  `json:"cannon"`
	Radar       int  `json:"radar"`
	See         int  `json:"see"`
	MaxCount    int  `json:"maxCount"`
	Asteroids   int  `json:"asteroids"`
	LoopTime    int  `json:"loopTime"`
	NoWait      bool `json:"noWait"`
}

type StartMessage struct {
	You        Team       `json:"you"`
	Config     GameConfig `json:"config"`
	OtherTeams []Team     `json:"otherTeams"`
}

type EventsMessage struct {
	RoundId    int        `json:"roundId"`
	You        Team       `json:"you"`
	Config     GameConfig `json:"config"`
	OtherTeams []Team     `json:"otherTeams"`
	Events     []Event    `json:"events"`
}

type EndMessage struct {
	WinnerTeamId int  `json:"winnerTeamId"`
	You          Team `json:"you"`
}

type Bot struct {
	BotId    int      `json:"botId"`
	Name     string   `json:"name"`
	TeamId   int      `json:"teamId"`
	Hp       int      `json:"hp"`
	Alive    bool     `json:"alive"`
	Position Position `json:"pos"`
}

type Team struct {
	Name   string `json:"name"`
	TeamId int    `json:"teamId"`
	Bots   []Bot  `json:"bots"`
}

type Event struct {
	Type     EventType `json:"event"`
	BotId    null.Int  `json:"botId"`
	Source   null.Int  `json:"source"`
	Position Position  `json:"pos"`
	Damage   null.Int  `json:"damage"`
}
