package main

import (
	"fmt"
	"math/rand"
	"time"

	"github.com/davecgh/go-spew/spew"
	"github.com/heppu/space-tyckiting/clients/go/client"
)

// Struct for our DummyAi
type DummyAi struct {
	MyTeam     client.Team
	OtherTeams []client.Team
	Config     client.GameConfig
}

// Name for Our Ai
const AiName string = "Dummy Bot"

func main() {
	// Create pointer to your AI struct
	a := &DummyAi{}

	// Seed random number generator
	rand.Seed(time.Now().UnixNano())

	// Call Run function from client package with pointer yto your AI struct
	client.Run(a, AiName)
}

// Move function will be called after OnEvents function returns
func (d *DummyAi) Move() (actions []client.Action) {

	// Move all bots randomly around the area
	for _, bot := range d.MyTeam.Bots {
		validMoves := bot.GetValidMoves(d.Config.Move)
		actions = append(actions, client.Action{
			Type:     client.BOT_MOVE,
			BotId:    bot.BotId,
			Position: validMoves[rand.Intn(len(validMoves))],
		})
	}
	return
}

// OnConnected function will be called when websocket connection is established
// and server has send connected message.
func (d *DummyAi) OnConnected(msg client.ConnectedMessage) {
	fmt.Println("[dummy][OnConnected]")
	spew.Dump(msg)
}

// OnStart will be called when game starts and server sends start message
func (d *DummyAi) OnStart(msg client.StartMessage) {
	fmt.Println("[dummy][OnStart]")
	spew.Dump(msg)

	// Save important information about game to our Ai struct
	d.MyTeam = msg.You
	d.OtherTeams = msg.OtherTeams
	d.Config = msg.Config
}

// OnEvents will be called when server sends events message.
func (d *DummyAi) OnEvents(msg client.EventsMessage) {
	fmt.Println("[dummy][OnEvents]")
	spew.Dump(msg)
}

// OnEvents will be called when server sends events message.
func (d *DummyAi) OnEnd(msg client.EndMessage) {
	fmt.Println("[dummy][OnEnd]")
	spew.Dump(msg)
}

// OnError will be called when server sends message that has unknow message type.
func (d *DummyAi) OnError(msg string) {
	fmt.Println("[dummy][OnError]")
	fmt.Sprintln(msg)
}
